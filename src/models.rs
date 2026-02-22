//! Core data structures for the Course Audit System
//!
//! This module defines all data types used throughout the application:
//! - `Course`: Individual course with grade and credit info
//! - `Category`: Top-level audit category (GenEd, Major, Electives)
//! - `AuditResult`: Final audit result with all categories and missing courses
//! - Curriculum types: `GenEdCurriculum`, `MajorCurriculum` for static curriculum data

use serde::{Deserialize, Serialize};

/// Represents a single course instance in the transcript
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub code: String,  // Course code (e.g., "344-101")
    pub name: String,  // Course name
    pub credit: f32,   // Credits earned
    pub grade: String, // Letter grade (A, B, C, etc.)
}

/// Aggregates courses within a displayable category (e.g., General Education, Major)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub name: String,           // Category name
    pub required_credits: f32,  // Total credits required
    pub collected_credits: f32, // Credits earned so far
    pub courses: Vec<Course>,   // Courses in this category
}

/// A single missing required course, tagged with its curriculum category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingCourse {
    pub category: String,     // e.g. "General Education", "Major Courses"
    pub description: String,  // e.g. "344-101 - Calculus I"
}

/// Final audit result containing all categories and missing requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResult {
    pub total_credits: f32,               // Total credits earned
    pub categories: Vec<Category>,        // All audit categories (GenEd, Major, Electives)
    pub missing_subjects: Vec<MissingCourse>, // Missing courses with their category
}

/// A single General Education course.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenEdCourse {
    pub code: String,
    pub name: String,
    pub credits: f32,
}

/// A nested sub-group under a GenEd strand.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenEdSubGroup {
    pub name: String,
    pub required_credits: f32,
    pub courses: Vec<GenEdCourse>,
}

/// A GenEd strand which may contain direct courses or sub-groups.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenEdStrand {
    pub id: u32,
    pub name: String,
    pub required_credits: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_groups: Option<Vec<GenEdSubGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub courses: Option<Vec<GenEdCourse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_rule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_groups: Option<Vec<Vec<String>>>,
}

/// Elective sub-category within GenEd (e.g., language electives).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenEdElectiveSubCategory {
    pub name: String,
    pub required_credits: f32,
    pub min_courses: u32,
    pub max_courses: u32,
    pub courses: Vec<GenEdCourse>,
}

/// Collects GenEd electives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenEdElectives {
    pub name: String,
    pub total_required_credits: f32,
    pub sub_categories: Vec<GenEdElectiveSubCategory>,
}

/// Top-level General Education curriculum definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenEdCurriculum {
    pub name: String,
    pub total_required_credits: f32,
    pub strands: Vec<GenEdStrand>,
    pub electives: GenEdElectives,
}

/// A course that belongs to the major curriculum.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorCourse {
    pub code: String,
    pub name: String,
    pub credits: f32,
}

/// Cluster of courses inside a domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorCluster {
    pub id: String,
    pub name: String,
    pub min_courses: u32,
    pub description: Option<String>,
    pub courses: Vec<MajorCourse>,
}

/// Domain grouping clusters of major electives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorDomain {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub clusters: Vec<MajorCluster>,
}

/// Basic science portion of the major.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorBasicScience {
    pub name: String,
    pub required_credits: f32,
    pub courses: Vec<MajorCourse>,
}

/// Core courses required for the major.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorCoreCourses {
    pub name: String,
    pub required_credits: f32,
    pub courses: Vec<MajorCourse>,
}

/// Capstone options for the major (project or co-op).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorCapstone {
    pub name: String,
    pub credits_per_option: f32,
    pub options: Vec<MajorCourse>,
}

/// Elective requirements, including domains and other choices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorElectives {
    pub name: String,
    pub total_required_credits: f32,
    pub clusters_to_complete: u32,
    pub domains: Vec<MajorDomain>,
    pub others: Vec<MajorCourse>,
}

/// Top-level Major curriculum definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorCurriculum {
    pub name: String,
    pub total_required_credits: f32,
    pub basic_science: MajorBasicScience,
    pub core_courses: MajorCoreCourses,
    pub capstone: MajorCapstone,
    pub electives: MajorElectives,
}

/// Parsed course details extracted from the transcript text.
#[derive(Debug, Clone)]
pub struct ParsedCourse {
    pub code: String,
    pub name: String,
    pub grade: String,
    pub parsed_credit: f32,
}
