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
                    // ==========================================
                    // GE2A: การคิดเชิงตรรกะและตัวเลข
                    // ==========================================
                    GenEdSubGroup {
                        name: "Logical and Numerical Thinking (GE2A)".to_string(),
                        required_credits: 2.0,
                        courses: vec![
                            GenEdCourse {
                                code: "895-211".to_string(),
                                name: "Thinking and Behavioral Prediction".to_string(),
                                credits: 2.0,
                            },
                            GenEdCourse {
                                code: "315-100".to_string(),
                                name: "The Art of Computing".to_string(),
                                credits: 2.0,
                            },
                            GenEdCourse {
                                code: "322-100".to_string(),
                                name: "Getting rich with mathematics".to_string(),
                                credits: 2.0,
                            },
                            GenEdCourse {
                                code: "473-001".to_string(),
                                name: "Financial Literacy for a Better Life".to_string(),
                                credits: 2.0,
                            },
                            GenEdCourse {
                                code: "473-002".to_string(),
                                name: "Reading Financial Statements for Investment".to_string(),
                                credits: 2.0,
                            },
                            GenEdCourse {
                                code: "142-010".to_string(),
                                name: "Organic Thinking".to_string(),
                                credits: 2.0,
                            },
                        ],
                    },
                    // ==========================================
                    // GE2B: การคิดเชิงระบบ
                    // ==========================================
                    GenEdSubGroup {
                        name: "Systems Thinking (GE2B)".to_string(),
                        required_credits: 2.0,
                        courses: vec![
                            GenEdCourse {
                                code: "895-221".to_string(),
                                name: "Thinking and Systematic Problem Solving".to_string(),
                                credits: 2.0,
                            },
                            GenEdCourse {
                                code: "895-222".to_string(),
                                name: "Critical Thinking".to_string(),
                                credits: 2.0,
                            },
                            GenEdCourse {
                                code: "895-223".to_string(),
                                name: "Cultivating Happiness through Positivity".to_string(),
                                credits: 2.0,
                            },
                            GenEdCourse {
                                code: "895-224".to_string(),
                                name: "Logic in Daily Life".to_string(),
                                credits: 2.0,
                            },
                            GenEdCourse {
                                code: "895-225".to_string(),
                                name: "The World Today".to_string(),
                                credits: 2.0,
                            },
                            GenEdCourse {
                                code: "315-202".to_string(),
                                name: "Thinking and Reasoning".to_string(),
                                credits: 2.0,
                            },
                            GenEdCourse {
                                code: "200-108".to_string(),
                                name: "MOBA and Strategy Development".to_string(),
                                credits: 2.0,
                            },
                            GenEdCourse {
                                code: "142-009".to_string(),
                                name: "Creative Problem Solving".to_string(),
                                credits: 2.0,
                            },
                        ],
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
                sub_groups: None, // ดูข้อเสนอแนะด้านล่างเกี่ยวกับการปรับใช้ sub_groups
                courses: Some(vec![
                    // --- ด้านสุนทรียศาสตร์ (Aesthetics) ---
                    GenEdCourse {
                        code: "895-861".to_string(),
                        name: "The Guitar".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-862".to_string(),
                        name: "The Ukulele".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-863".to_string(),
                        name: "The Harmonica".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-864".to_string(),
                        name: "Western Music".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-865".to_string(),
                        name: "The Traditional Thai Dulcimer".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-866".to_string(),
                        name: "Piphat Ensembles".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-867".to_string(),
                        name: "Creative Music".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-868".to_string(),
                        name: "ASEAN Music".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-833".to_string(),
                        name: "Drama and Self-reflection".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-834".to_string(),
                        name: "Creative Drawing".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-843".to_string(),
                        name: "Appreciation of the Thai Language".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "315-102".to_string(),
                        name: "The Aesthetic in Photography".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "061-001".to_string(),
                        name: "Aesthetics of Thai Dance".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "061-002".to_string(),
                        name: "Music aesthetics in Life".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "061-003".to_string(),
                        name: "Nora for health".to_string(),
                        credits: 2.0,
                    },
                    // --- ด้านกีฬา (Sports) ---
                    GenEdCourse {
                        code: "895-871".to_string(),
                        name: "Pétanque".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-872".to_string(),
                        name: "Takraw".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-873".to_string(),
                        name: "Futsal".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-874".to_string(),
                        name: "Social Dance".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-875".to_string(),
                        name: "Badminton".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-876".to_string(),
                        name: "Swimming".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-877".to_string(),
                        name: "Swimming for Lifesaving".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-878".to_string(),
                        name: "Table Tennis".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-879".to_string(),
                        name: "Tennis".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-880".to_string(),
                        name: "Exercise for Health".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-881".to_string(),
                        name: "Fat to Fit".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-882".to_string(),
                        name: "Fit and Firm".to_string(),
                        credits: 2.0,
                    },
                    GenEdCourse {
                        code: "895-883".to_string(),
                        name: "Happy Camping".to_string(),
                        credits: 2.0,
                    }, // เปิดสอน 2/2567
                    GenEdCourse {
                        code: "895-884".to_string(),
                        name: "Basketball".to_string(),
                        credits: 2.0,
                    },
                ]),
                selection_rule: Some("choose_one".to_string()),
                sequence_groups: None,
            },
        ],
        electives: GenEdElectives {
            name: "GenEd Electives (GE8)".to_string(),
            total_required_credits: 6.0, // ปรับแต่งตัวเลขนี้ตามโครงสร้างหลักสูตรจริง
            sub_categories: vec![
                // ==========================================
                // 1. กลุ่มวิชาภาษาอังกฤษ (คณะศิลปศาสตร์)
                // ==========================================
                GenEdElectiveSubCategory {
                    name: "English Language".to_string(),
                    required_credits: 0.0,
                    min_courses: 0,
                    max_courses: 99,
                    courses: vec![
                        GenEdCourse {
                            code: "890-811".to_string(),
                            name: "English Grammar for Real Life Communication".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-821".to_string(),
                            name: "English Pronunciation through Songs".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-831".to_string(),
                            name: "Strategic Reading for Greater Comprehension".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-841".to_string(),
                            name: "English for Presentations and Visual Aids Design".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-842".to_string(),
                            name: "English Listening and Speaking for Digital Citizens".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-843".to_string(),
                            name: "English Conversation".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-851".to_string(),
                            name: "Reading to Write in English".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-852".to_string(),
                            name: "Academic Reading and Writing in English".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-861".to_string(),
                            name: "Consolidating English through News".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-862".to_string(),
                            name: "English around the Clock".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-863".to_string(),
                            name: "English for Digital Literacy".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-871".to_string(),
                            name: "English Writing with Online Technology".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-872".to_string(),
                            name: "English and Digital Tools".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-873".to_string(),
                            name: "Discovering English with Online Corpora".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-874".to_string(),
                            name: "Google Translate Me".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-881".to_string(),
                            name: "English for Job Applications".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-882".to_string(),
                            name: "English in the Workplace".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-883".to_string(),
                            name: "English for Travelers".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-884".to_string(),
                            name: "English for Entrepreneurs and Consumers".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-885".to_string(),
                            name: "English Test Taking Strategies for Employment".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "890-886".to_string(),
                            name: "Learning English through Cultures".to_string(),
                            credits: 2.0,
                        },
                    ],
                },
                // ==========================================
                // 2. กลุ่มวิชาภาษาต่างประเทศ (คณะศิลปศาสตร์)
                // ==========================================
                GenEdElectiveSubCategory {
                    name: "Foreign Languages".to_string(),
                    required_credits: 0.0,
                    min_courses: 0,
                    max_courses: 99,
                    courses: vec![
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
                            code: "891-813".to_string(),
                            name: "Japanese Conversation in the Workplace".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-821".to_string(),
                            name: "Basic Chinese".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-822".to_string(),
                            name: "Chinese Conversation in Daily Life".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-823".to_string(),
                            name: "Chinese Conversation in the Workplace".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-824".to_string(),
                            name: "Chinese Calligraphy".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-831".to_string(),
                            name: "Basic Malay".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-832".to_string(),
                            name: "Malay Conversation in Daily Life".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-833".to_string(),
                            name: "Malay Conversation for Tourism".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-841".to_string(),
                            name: "Survival Korean for Thais".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-842".to_string(),
                            name: "Korean Conversation for Beginners".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-843".to_string(),
                            name: "Insights into Korean Language and Culture".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-861".to_string(),
                            name: "Getting to Know Bahasa Indonesia".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-862".to_string(),
                            name: "Bahasa Indonesia for Everyday Communication".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "891-863".to_string(),
                            name: "Bahasa Indonesia in the Workplace".to_string(),
                            credits: 2.0,
                        },
                    ],
                },
                // ==========================================
                // 3. กลุ่มวิชามนุษยศาสตร์และสังคมศาสตร์ (คณะศิลปศาสตร์)
                // ==========================================
                GenEdElectiveSubCategory {
                    name: "Humanities and Social Sciences".to_string(),
                    required_credits: 0.0,
                    min_courses: 0,
                    max_courses: 99,
                    courses: vec![
                        GenEdCourse {
                            code: "895-811".to_string(),
                            name: "Psychology of Love".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-812".to_string(),
                            name: "Psychology for Good Life".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-813".to_string(),
                            name: "Workplace Newcomers".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-814".to_string(),
                            name: "Knowing Others and Yourself through Human Behaviors".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-815".to_string(),
                            name: "Social Interaction".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-816".to_string(),
                            name: "Development Studies".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-817".to_string(),
                            name: "Charming Personality".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-818".to_string(),
                            name: "Life Skills in Society 5.0".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-819".to_string(),
                            name: "Me and Others".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-821".to_string(),
                            name: "Tourism and Superstition".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-822".to_string(),
                            name: "Backpacking Trips".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-823".to_string(),
                            name: "Psychology for Service".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-824".to_string(),
                            name: "Creative Tourism".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-825".to_string(),
                            name: "Volunteer Tourism".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-826".to_string(),
                            name: "Passengers Your attention Please".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-827".to_string(),
                            name: "ASEAN World Heritage Sites".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-831".to_string(),
                            name: "Ethics for Life".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-832".to_string(),
                            name: "Religious Diversity".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-835".to_string(),
                            name: "Art in Multicultural Society".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-836".to_string(),
                            name: "China : Past, Present, and Future".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-837".to_string(),
                            name: "Astrology and Life".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-838".to_string(),
                            name: "History in Movies".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-841".to_string(),
                            name: "Communication Skills".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-842".to_string(),
                            name: "Thai Listening and Speaking Skills".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-844".to_string(),
                            name: "The Art of Creative Writing".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-845".to_string(),
                            name: "ASEAN Literature".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-846".to_string(),
                            name: "Man and Literature".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-847".to_string(),
                            name: "Culture in Literature".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-848".to_string(),
                            name: "Thai Language and Culture".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-849".to_string(),
                            name: "The Art of Listening".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-850".to_string(),
                            name: "Thai Usage".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "895-851".to_string(),
                            name: "The Charm of Southern Thai Dialects".to_string(),
                            credits: 2.0,
                        },
                    ],
                },
                // ==========================================
                // 4. กลุ่มวิชาวิทยาศาสตร์และสุขภาพ (คณะวิทยาศาสตร์)
                // ==========================================
                GenEdElectiveSubCategory {
                    name: "Science and Health".to_string(),
                    required_credits: 0.0,
                    min_courses: 0,
                    max_courses: 99,
                    courses: vec![
                        GenEdCourse {
                            code: "315-103".to_string(),
                            name: "Introduction to Intellectual Property".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "336-214".to_string(),
                            name: "Smart Eating and Being Healthy".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "336-215".to_string(),
                            name: "Safety Life from Toxic Substances".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "336-216".to_string(),
                            name: "Drug and Health".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "315-203".to_string(),
                            name: "Key to Nature".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "315-205".to_string(),
                            name: "Science Entrepreneur Pitching".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "315-206".to_string(),
                            name: "Science Facts".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "338-101".to_string(),
                            name: "My body and health".to_string(),
                            credits: 2.0,
                        },
                    ],
                },
                // ==========================================
                // 5. กลุ่มวิชากฎหมาย (คณะนิติศาสตร์)
                // ==========================================
                GenEdElectiveSubCategory {
                    name: "Law".to_string(),
                    required_credits: 0.0,
                    min_courses: 0,
                    max_courses: 99,
                    courses: vec![
                        GenEdCourse {
                            code: "874-191".to_string(),
                            name: "Introduction to Thai Legal System".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "874-192".to_string(),
                            name: "Law relating to Occupations and Everyday Life".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "874-193".to_string(),
                            name: "General Principles of Law and Judicial Process".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "874-194".to_string(),
                            name: "Taxation and Life".to_string(),
                            credits: 2.0,
                        },
                        GenEdCourse {
                            code: "874-195".to_string(),
                            name: "Human Rights and Citizenship".to_string(),
                            credits: 2.0,
                        },
                    ],
                },
                // ==========================================
                // 6. กลุ่มวิชาสหวิทยาการและคณะอื่นๆ (แพทย์แผนไทย, อุตสาหกรรมเกษตร, ทันตแพทย์ ฯลฯ)
                // ==========================================
                GenEdElectiveSubCategory {
                    name: "Interdisciplinary and Others".to_string(),
                    required_credits: 0.0,
                    min_courses: 0,
                    max_courses: 99,
                    courses: vec![
                        GenEdCourse {
                            code: "193-031".to_string(),
                            name: "Natural Therapy".to_string(),
                            credits: 2.0,
                        }, // คณะการแพทย์แผนไทย
                        GenEdCourse {
                            code: "003-002".to_string(),
                            name: "PSU FOR MANKIND".to_string(),
                            credits: 2.0,
                        }, // ศูนย์อาสาสมัคร
                        GenEdCourse {
                            code: "001-101".to_string(),
                            name: "ASEAN Studies".to_string(),
                            credits: 2.0,
                        }, // ศูนย์อาเซียนศึกษา
                        GenEdCourse {
                            code: "858-154".to_string(),
                            name: "Green packaging in daily life".to_string(),
                            credits: 2.0,
                        }, // คณะอุตสาหกรรมเกษตร
                        GenEdCourse {
                            code: "858-161".to_string(),
                            name: "Nutrition and Healthy Food in Daily Life".to_string(),
                            credits: 2.0,
                        }, // คณะอุตสาหกรรมเกษตร
                        GenEdCourse {
                            code: "858-162".to_string(),
                            name: "Being a Smart Consumer".to_string(),
                            credits: 2.0,
                        }, // คณะอุตสาหกรรมเกษตร
                        GenEdCourse {
                            code: "670-411".to_string(),
                            name: "Leading your life".to_string(),
                            credits: 2.0,
                        }, // คณะทันตแพทยศาสตร์
                        GenEdCourse {
                            code: "500-101".to_string(),
                            name: "Happy farm".to_string(),
                            credits: 2.0,
                        }, // คณะทรัพยากรธรรมชาติ
                    ],
                },
            ],
        },
    }
}
