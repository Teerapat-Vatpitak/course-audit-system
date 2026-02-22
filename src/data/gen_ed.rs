use crate::models::{
    GenEdCourse, GenEdCurriculum, GenEdElectiveSubCategory, GenEdElectives, GenEdStrand,
    GenEdSubGroup,
};

pub fn get_gen_ed_curriculum() -> GenEdCurriculum {
    GenEdCurriculum {
        name: "General Education".to_string(),
        total_required_credits: 30.0,
        strands: vec![
            GenEdStrand {
                id: 1,
                name: "King's Philosophy and Benefits for Mankind".to_string(),
                required_credits: 4.0,
                sub_groups: None,
                courses: Some(vec![
                    GenEdCourse {
                        code: "003-001".to_string(),
                        name: "Volunteer Leader for Sustainable Community Development".to_string(),
                        credits: 3.0,
                    },
                    GenEdCourse {
                        code: "388-100".to_string(),
                        name: "Health for All".to_string(),
                        credits: 1.0,
                    },
                ]),
                selection_rule: Some("choose_all".to_string()),
                sequence_groups: None,
            },
            GenEdStrand {
                id: 2,
                name: "Citizenship and Peaceful Life".to_string(),
                required_credits: 5.0,
                sub_groups: None,
                courses: Some(vec![
                    GenEdCourse {
                        code: "895-001".to_string(),
                        name: "Good Citizens".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "950-102".to_string(),
                        name: "Happy and Peaceful Life".to_string(),
                        credits: 3.0,
                    },
                ]),
                selection_rule: Some("choose_all".to_string()),
                sequence_groups: None,
            },
            GenEdStrand {
                id: 3,
                name: "Entrepreneurship".to_string(),
                required_credits: 1.0,
                sub_groups: None,
                courses: Some(vec![GenEdCourse {
                    code: "460-001".to_string(),
                    name: "Idea to Entrepreneurship".to_string(),
                    credits: 1.0,
                }]),
                selection_rule: Some("choose_all".to_string()),
                sequence_groups: None,
            },
            GenEdStrand {
                id: 4,
                name: "Living with Awareness and Digital Literacy".to_string(),
                required_credits: 4.0,
                sub_groups: Some(vec![
                    GenEdSubGroup {
                        name: "Living with Awareness".to_string(),
                        required_credits: 2.0,
                        courses: vec![GenEdCourse {
                            code: "315-201".to_string(),
                            name: "Life in the Future".to_string(),
                            credits: 2.0,
                        }],
                    },
                    GenEdSubGroup {
                        name: "Digital Literacy".to_string(),
                        required_credits: 2.0,
                        courses: vec![GenEdCourse {
                            code: "315-104".to_string(),
                            name: "Digital Technology Literacy".to_string(),
                            credits: 2.0,
                        }],
                    },
                ]),
                courses: None,
                selection_rule: Some("choose_all_sub_groups".to_string()),
                sequence_groups: None,
            },
            GenEdStrand {
                id: 5,
                name: "Systems Thinking, Logical and Numerical Thinking".to_string(),
                required_credits: 4.0,
                sub_groups: Some(vec![
                    GenEdSubGroup {
                        name: "Logical and Numerical Thinking".to_string(),
                        required_credits: 2.0,
                        courses: vec![GenEdCourse {
                            code: "315-100".to_string(),
                            name: "The Art of Computing".to_string(),
                            credits: 2.0,
                        }],
                    },
                    GenEdSubGroup {
                        name: "Systems Thinking".to_string(),
                        required_credits: 2.0,
                        courses: vec![GenEdCourse {
                            code: "315-202".to_string(),
                            name: "Thinking and Reasoning".to_string(),
                            credits: 2.0,
                        }],
                    },
                ]),
                courses: None,
                selection_rule: Some("choose_all_sub_groups".to_string()),
                sequence_groups: None,
            },
            GenEdStrand {
                id: 6,
                name: "Language and Communication".to_string(),
                required_credits: 4.0,
                sub_groups: None,
                courses: Some(vec![
                    GenEdCourse {
                        code: "890-101".to_string(),
                        name: "Essential English".to_string(),
                        credits: 0.0,
                    },
                    GenEdCourse {
                        code: "890-102".to_string(),
                        name: "Everyday English".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "890-103".to_string(),
                        name: "English on the Go".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "890-104".to_string(),
                        name: "English in the Digital World".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "890-105".to_string(),
                        name: "English for Academic Success".to_string(),
                        credits: 2.0,
                    },
                ]),
                selection_rule: Some("choose_sequential_pair".to_string()),
                sequence_groups: Some(vec![
                    vec!["890-102".to_string(), "890-103".to_string()],
                    vec!["890-103".to_string(), "890-104".to_string()],
                    vec!["890-104".to_string(), "890-105".to_string()],
                ]),
            },
            GenEdStrand {
                id: 7,
                name: "Aesthetics and Sports".to_string(),
                required_credits: 2.0,
                sub_groups: None,
                courses: Some(vec![
                    GenEdCourse {
                        code: "315-102".to_string(),
                        name: "The Aesthetic in Photography".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-881".to_string(),
                        name: "Fat to Fit".to_string(),
                        credits: 2.0,
                    },
                ]),
                selection_rule: Some("choose_one".to_string()),
                sequence_groups: None,
            },
        ],
        electives: GenEdElectives {
            name: "GenEd Electives".to_string(),
            total_required_credits: 6.0,
            sub_categories: vec![
                GenEdElectiveSubCategory {
                    name: "Language Electives".to_string(),
                    required_credits: 2.0,
                    min_courses: 1,
                    max_courses: 1,
                    courses: vec![
                        GenEdCourse {
                            code: "890-861".to_string(),
                            name: "Consolidating English through News".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-811".to_string(),
                            name: "First Steps to Japanese".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-812".to_string(),
                            name: "Japanese Conversation in Daily Life".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-821".to_string(),
                            name: "Basic Chinese".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-831".to_string(),
                            name: "Strategic Reading for Greater Comprehension".to_string(),
                            credits: 2.0,
                        },
                    ],
                },
                GenEdElectiveSubCategory {
                    name: "General Electives".to_string(),
                    required_credits: 4.0,
                    min_courses: 2,
                    max_courses: 2,
                    courses: vec![
                        GenEdCourse {
                            code: "193-031".to_string(),
                            name: "Natural Therapy".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "003-002".to_string(),
                            name: "PSU FOR MANKIND".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "001-101".to_string(),
                            name: "ASEAN Studies".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "858-154".to_string(),
                            name: "Green packaging in daily life".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "142-011".to_string(),
                            name: "Everyone Can Draw".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-041".to_string(),
                            name: "ETHICAL PHILOSOPHY".to_string(),
                            credits: 2.0,
                        },
                    ],
                },
            ],
        },
    }
}
