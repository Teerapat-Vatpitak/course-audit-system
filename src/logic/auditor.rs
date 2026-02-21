//! Curriculum Auditing Engine
//!
//! Implements the core logic for validating student transcripts against curriculum requirements:
//! - **GenEd Auditing**: Matches courses to 6 strands with support for sub-groups and sequences
//! - **Major Auditing**: Matches courses to Basic Science, Core, Capstone, and Electives
//! - **Free Elective Detection**: Credits unmatched courses as free electives
//! - **Greedy Matching**: Allows repeatable courses to accumulate credits

use crate::models::{GenEdCurriculum, MajorCurriculum, MissingCourse, ParsedCourse};
use leptos::logging;
use std::collections::HashSet;

/// Audits courses against the GenEd curriculum, honoring strand sub-groups and
/// sequential strand rules. Credits come from the curriculum (golden data).
pub fn audit_gen_ed(
    courses: &[ParsedCourse],
    curriculum: &GenEdCurriculum,
) -> (f32, Vec<MissingCourse>, HashSet<usize>) {
    let mut completed_credits = 0.0;
    let mut missing_courses: Vec<MissingCourse> = Vec::new();
    let mut used_indices = HashSet::new();

    for strand in &curriculum.strands {
        // Handle sequential requirement for Strand 6 by finding any valid pair.
        if strand.id == 6 {
            let mut sequence_satisfied = false;

            if let (Some(strand_courses), Some(sequence_groups)) = (&strand.courses, &strand.sequence_groups) {
                'outer: for pair in sequence_groups {
                    if pair.len() != 2 {
                        continue;
                    }

                    let mut found_indices = Vec::new();
                    let mut credits_sum = 0.0;

                    for code in pair {
                        if let Some(def_course) = strand_courses.iter().find(|c| &c.code == code) {
                            if let Some((idx, _parsed)) = courses.iter().enumerate().find(|(idx, parsed)| {
                                !used_indices.contains(idx)
                                    && parsed.code == *code
                                    && !matches!(parsed.grade.as_str(), "F" | "W" | "S" | "U")
                            }) {
                                found_indices.push(idx);
                                credits_sum += def_course.credits;
                            }
                        }
                    }

                    if found_indices.len() == 2 {
                        for idx in found_indices {
                            used_indices.insert(idx);
                        }
                        completed_credits += credits_sum;
                        sequence_satisfied = true;
                        logging::log!(
                            "[AUDIT] GenEd Strand 6 sequence satisfied with pair {:?} - {} credits",
                            pair,
                            credits_sum
                        );
                        break 'outer;
                    }
                }
            }

            if sequence_satisfied {
                continue;
            }
        }

        if let Some(strand_courses) = &strand.courses {
            for course in strand_courses {
                if let Some((idx, _parsed)) = courses.iter().enumerate().find(|(idx, parsed)| {
                    !used_indices.contains(idx)
                        && parsed.code == course.code
                        && !matches!(parsed.grade.as_str(), "F" | "W" | "S" | "U")
                }) {
                    completed_credits += course.credits;
                    used_indices.insert(idx);
                    logging::log!(
                        "[AUDIT] GenEd Strand {} used: {} (index {}) - {} credits",
                        strand.id,
                        course.code,
                        idx,
                        course.credits
                    );
                } else {
                    missing_courses.push(MissingCourse {
                        category: "General Education".to_string(),
                        description: format!("{} - {}", course.code, course.name),
                    });
                }
            }
        }

        if let Some(sub_groups) = &strand.sub_groups {
            for sub_group in sub_groups {
                for course in &sub_group.courses {
                    if let Some((idx, _parsed)) = courses.iter().enumerate().find(|(idx, parsed)| {
                        !used_indices.contains(idx)
                            && parsed.code == course.code
                            && !matches!(parsed.grade.as_str(), "F" | "W" | "S" | "U")
                    }) {
                        completed_credits += course.credits;
                        used_indices.insert(idx);
                        logging::log!(
                            "[AUDIT] GenEd Strand {} -> {} used: {} (index {}) - {} credits",
                            strand.id,
                            sub_group.name,
                            course.code,
                            idx,
                            course.credits
                        );
                    } else {
                        missing_courses.push(MissingCourse {
                            category: "General Education".to_string(),
                            description: format!("{} - {}", course.code, course.name),
                        });
                    }
                }
            }
        }
    }

    for sub_cat in &curriculum.electives.sub_categories {
        for course in &sub_cat.courses {
            if let Some((idx, _parsed)) = courses.iter().enumerate().find(|(idx, parsed)| {
                !used_indices.contains(idx)
                    && parsed.code == course.code
                    && !matches!(parsed.grade.as_str(), "F" | "W" | "S" | "U")
            }) {
                completed_credits += course.credits;
                used_indices.insert(idx);
                logging::log!(
                    "[AUDIT] GenEd Elective -> {} used: {} (index {}) - {} credits",
                    sub_cat.name,
                    course.code,
                    idx,
                    course.credits
                );
            }
        }
    }

    (completed_credits, missing_courses, used_indices)
}

/// Audits courses against the major curriculum, including greedy matching for
/// special-topics and other elective buckets. Credits are taken from curriculum
/// data to avoid PDF parsing drift.
pub fn audit_major(
    courses: &[ParsedCourse],
    curriculum: &MajorCurriculum,
) -> (f32, f32, Vec<MissingCourse>, HashSet<usize>) {
    let mut completed_credits = 0.0;
    let mut elective_credits = 0.0;
    let mut missing_courses: Vec<MissingCourse> = Vec::new();
    let mut used_indices = HashSet::new();

    for course in &curriculum.basic_science.courses {
        if let Some((idx, _)) = courses.iter().enumerate().find(|(idx, parsed)| {
            !used_indices.contains(idx)
                && parsed.code == course.code
                && !matches!(parsed.grade.as_str(), "F" | "W" | "S" | "U")
        }) {
            completed_credits += course.credits;
            used_indices.insert(idx);
            logging::log!(
                "[AUDIT] Major Basic Science used: {} - {} credits (from curriculum)",
                course.code,
                course.credits
            );
        } else if courses.iter().any(|c| c.code == course.code) {
            logging::log!("[AUDIT] Major Basic Science failed: {}", course.code);
        } else {
            missing_courses.push(MissingCourse {
                category: "Basic Science".to_string(),
                description: format!("{} - {}", course.code, course.name),
            });
        }
    }

    for course in &curriculum.core_courses.courses {
        if let Some((idx, _)) = courses.iter().enumerate().find(|(idx, parsed)| {
            !used_indices.contains(idx)
                && parsed.code == course.code
                && !matches!(parsed.grade.as_str(), "F" | "W" | "S" | "U")
        }) {
            completed_credits += course.credits;
            used_indices.insert(idx);
            logging::log!(
                "[AUDIT] Major Core used: {} - {} credits (from curriculum)",
                course.code,
                course.credits
            );
        } else if courses.iter().any(|c| c.code == course.code) {
            logging::log!("[AUDIT] Major Core failed: {}", course.code);
        } else {
            missing_courses.push(MissingCourse {
                category: "Core Courses".to_string(),
                description: format!("{} - {}", course.code, course.name),
            });
        }
    }

    for option in &curriculum.capstone.options {
        if let Some((idx, _)) = courses.iter().enumerate().find(|(idx, parsed)| {
            !used_indices.contains(idx)
                && parsed.code == option.code
                && !matches!(parsed.grade.as_str(), "F" | "W" | "S" | "U")
        }) {
            completed_credits += option.credits;
            used_indices.insert(idx);
            logging::log!(
                "[AUDIT] Major Capstone used: {} - {} credits (from curriculum)",
                option.code,
                option.credits
            );
            break;
        }
    }

    for domain in &curriculum.electives.domains {
        for cluster in &domain.clusters {
            for course in &cluster.courses {
                if let Some((idx, _)) = courses.iter().enumerate().find(|(idx, parsed)| {
                    !used_indices.contains(idx)
                        && parsed.code == course.code
                        && !matches!(parsed.grade.as_str(), "F" | "W" | "S" | "U")
                }) {
                    elective_credits += course.credits;
                    used_indices.insert(idx);
                    logging::log!(
                        "[AUDIT] Major Elective used: {} - {} credits (from curriculum)",
                        course.code,
                        course.credits
                    );
                }
            }
        }
    }

    // Greedy match "others" electives so repeated special topics accumulate credits.
    for course in &curriculum.electives.others {
        for (idx, parsed) in courses.iter().enumerate() {
            if !used_indices.contains(&idx)
                && parsed.code == course.code
                && !matches!(parsed.grade.as_str(), "F" | "W" | "S" | "U")
            {
                elective_credits += course.credits;
                used_indices.insert(idx);
                logging::log!(
                    "[AUDIT] Major Elective (Other) used: {} (index {}) - {} credits (from curriculum)",
                    course.code,
                    idx,
                    course.credits
                );
            }
        }
    }

    (completed_credits, elective_credits, missing_courses, used_indices)
}

/// Calculates free-elective credits from unused courses, pulling credit values
/// directly from the PDF when the course is not mapped elsewhere.
pub fn calculate_free_electives(
    courses: &[ParsedCourse],
    used_indices: &HashSet<usize>,
) -> (f32, Vec<String>) {
    let mut free_elective_credits = 0.0;
    let mut free_elective_list = Vec::new();

    for (idx, parsed) in courses.iter().enumerate() {
        if !used_indices.contains(&idx) {
            if !matches!(parsed.grade.as_str(), "F" | "W" | "S" | "U") {
                let credits = parsed.parsed_credit;
                free_elective_credits += credits;
                free_elective_list.push(format!(
                    "{} (Grade: {}, {} cr)",
                    parsed.code, parsed.grade, credits
                ));
                logging::log!(
                    "[AUDIT] Free Elective: {} - Grade: {} - {} credits (from PDF)",
                    parsed.code,
                    parsed.grade,
                    credits
                );
            }
        }
    }

    logging::log!("[AUDIT] Total Free Elective Credits: {}", free_elective_credits);

    (free_elective_credits, free_elective_list)
}
