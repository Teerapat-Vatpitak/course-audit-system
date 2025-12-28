//! Major-Specific Curriculum Data
//!
//! Defines Computer Science major requirements:
//! - **Basic Science**: Foundational math and science courses (e.g., Calculus, Physics)
//! - **Core Courses**: Required major courses (e.g., Data Structures, Algorithms, Databases)
//! - **Capstone**: Final project or co-op requirement (choose one)
//! - **Electives**: Domain-specific clusters and other approved electives
//!   - Cloud Computing
//!   - Cyber Security
//!   - Software Engineering & Dev Ops
//!   - Data Science & AI
//!   - Mobile/Web Development
//!   - Game Development
//!   - IoT & Embedded Systems
//!   - Other approved electives

use crate::models::{
    MajorBasicScience, MajorCapstone, MajorCluster, MajorCoreCourses, MajorCourse, MajorCurriculum,
    MajorDomain, MajorElectives,
};

/// Returns the static Major curriculum definition used during audits.
pub fn get_major_curriculum() -> MajorCurriculum {
    MajorCurriculum {
        name: "Major Specific Courses - Computer Science".to_string(),
        total_required_credits: 96.0,

        // 2.1 Basic Science (12 Credits)
        basic_science: MajorBasicScience {
            name: "Basic Science".to_string(),
            required_credits: 12.0,
            courses: vec![
                MajorCourse {
                    code: "324-101".to_string(),
                    name: "General Chemistry I".to_string(),
                    credits: 3.0,
                },
                MajorCourse {
                    code: "325-101".to_string(),
                    name: "General Chemistry Laboratory I".to_string(),
                    credits: 1.0,
                },
                MajorCourse {
                    code: "330-101".to_string(),
                    name: "Principles of Biology I".to_string(),
                    credits: 3.0,
                },
                MajorCourse {
                    code: "331-101".to_string(),
                    name: "Principles of Biology Laboratory I".to_string(),
                    credits: 1.0,
                },
                MajorCourse {
                    code: "332-101".to_string(),
                    name: "Fundamental Physics".to_string(),
                    credits: 3.0,
                },
                MajorCourse {
                    code: "333-101".to_string(),
                    name: "Fundamental Physics Laboratory".to_string(),
                    credits: 1.0,
                },
            ],
        },

        // 2.2 Core Courses (56 Credits)
        core_courses: MajorCoreCourses {
            name: "Core Courses".to_string(),
            required_credits: 56.0,
            courses: vec![
                MajorCourse {
                    code: "322-101".to_string(),
                    name: "Calculus I".to_string(),
                    credits: 3.0,
                },
                MajorCourse {
                    code: "322-102".to_string(),
                    name: "Calculus II".to_string(),
                    credits: 3.0,
                },
                MajorCourse {
                    code: "344-201".to_string(),
                    name: "MODULE: Computing for Computer Science".to_string(),
                    credits: 6.0,
                },
                MajorCourse {
                    code: "344-111".to_string(),
                    name: "MODULE: Programming Concepts and Algorithms".to_string(),
                    credits: 6.0,
                },
                MajorCourse {
                    code: "344-181".to_string(),
                    name: "Communication Skill in Technology".to_string(),
                    credits: 1.0,
                },
                MajorCourse {
                    code: "344-233".to_string(),
                    name: "MODULE: Information Systems Analysis and Design and Principles of Database Systems".to_string(),
                    credits: 6.0,
                },
                MajorCourse {
                    code: "344-211".to_string(),
                    name: "Introduction to Object-Oriented Programming".to_string(),
                    credits: 2.0,
                },
                MajorCourse {
                    code: "344-243".to_string(),
                    name: "Software Interactive Design".to_string(),
                    credits: 1.0,
                },
                MajorCourse {
                    code: "344-221".to_string(),
                    name: "Computer Architectures and Organization".to_string(),
                    credits: 2.0,
                },
                MajorCourse {
                    code: "344-222".to_string(),
                    name: "Operating Systems".to_string(),
                    credits: 2.0,
                },
                MajorCourse {
                    code: "344-223".to_string(),
                    name: "Fundamentals of Computer Security".to_string(),
                    credits: 2.0,
                },
                MajorCourse {
                    code: "344-281".to_string(),
                    name: "Public Speaking in Computer Science".to_string(),
                    credits: 1.0,
                },
                MajorCourse {
                    code: "344-341".to_string(),
                    name: "Software Engineering".to_string(),
                    credits: 3.0,
                },
                MajorCourse {
                    code: "344-351".to_string(),
                    name: "Data Communications and Networking".to_string(),
                    credits: 3.0,
                },
                MajorCourse {
                    code: "344-361".to_string(),
                    name: "Principles of Artificial Intelligence".to_string(),
                    credits: 3.0,
                },
                MajorCourse {
                    code: "344-381".to_string(),
                    name: "Thinking and Creativity for Innovation Design".to_string(),
                    credits: 2.0,
                },
                MajorCourse {
                    code: "344-382".to_string(),
                    name: "Ethics for Digital Technology".to_string(),
                    credits: 1.0,
                },
                MajorCourse {
                    code: "344-491".to_string(),
                    name: "Seminar in Computer Science".to_string(),
                    credits: 1.0,
                },
            ],
        },

        // Capstone (Select one: Project or Co-op)
        capstone: MajorCapstone {
            name: "Capstone".to_string(),
            credits_per_option: 3.0,
            options: vec![
                MajorCourse {
                    code: "344-492".to_string(),
                    name: "Projects in Computer Science".to_string(),
                    credits: 3.0,
                },
                MajorCourse {
                    code: "344-495".to_string(),
                    name: "Cooperative Education".to_string(),
                    credits: 6.0,
                },
            ],
        },

        // 2.3 Electives (Pick 2 clusters, complete all courses in them)
        electives: MajorElectives {
            name: "Electives".to_string(),
            total_required_credits: 12.0,
            clusters_to_complete: 2,
            domains: vec![
                // Domain 1: Big Data & Business Intelligence
                MajorDomain {
                    id: 1,
                    name: "Big Data & Business Intelligence".to_string(),
                    description: Some("Cluster-based electives for data-driven technologies".to_string()),
                    clusters: vec![
                        MajorCluster {
                            id: "1.1".to_string(),
                            name: "Big Data".to_string(),
                            description: None,
                            courses: vec![
                                MajorCourse {
                                    code: "344-331".to_string(),
                                    name: "Data Science".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-332".to_string(),
                                    name: "Data Mining".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-431".to_string(),
                                    name: "Big Data".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                        MajorCluster {
                            id: "1.2".to_string(),
                            name: "Business Intelligence".to_string(),
                            description: None,
                            courses: vec![
                                MajorCourse {
                                    code: "344-232".to_string(),
                                    name: "Knowledge Management and Decision Support Systems".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-333".to_string(),
                                    name: "Data Analytics and Visualization".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-334".to_string(),
                                    name: "Business Intelligent Systems".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                        MajorCluster {
                            id: "1.3".to_string(),
                            name: "Information-driven Technology".to_string(),
                            description: None,
                            courses: vec![
                                MajorCourse {
                                    code: "344-311".to_string(),
                                    name: "Advanced Object-Oriented Programming".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-432".to_string(),
                                    name: "Next Generation Database Technologies".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-401".to_string(),
                                    name: "Cryptography and Security".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                    ],
                },

                // Domain 2: Internet & Network Technology
                MajorDomain {
                    id: 2,
                    name: "Internet & Network Technology".to_string(),
                    description: Some("Cluster-based electives for network and internet technologies".to_string()),
                    clusters: vec![
                        MajorCluster {
                            id: "2.1".to_string(),
                            name: "Network Technology".to_string(),
                            description: None,
                            courses: vec![
                                MajorCourse {
                                    code: "344-352".to_string(),
                                    name: "Computer Network Systems".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-353".to_string(),
                                    name: "Computer Systems and Network Security".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-451".to_string(),
                                    name: "Internet Technology and Applications".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                        MajorCluster {
                            id: "2.2".to_string(),
                            name: "Wireless and Mobile Technology".to_string(),
                            description: None,
                            courses: vec![
                                MajorCourse {
                                    code: "344-212".to_string(),
                                    name: "Web Application Programming".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-312".to_string(),
                                    name: "Mobile Application Development".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-321".to_string(),
                                    name: "Wireless Technology".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                        MajorCluster {
                            id: "2.3".to_string(),
                            name: "Internet Technology".to_string(),
                            description: None,
                            courses: vec![
                                MajorCourse {
                                    code: "344-322".to_string(),
                                    name: "Embedded Systems".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-323".to_string(),
                                    name: "Internet of Things".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-324".to_string(),
                                    name: "Cloud Computing Systems".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                    ],
                },

                // Domain 3: Software Development
                MajorDomain {
                    id: 3,
                    name: "Software Development".to_string(),
                    description: Some("Cluster-based electives for software development specialization".to_string()),
                    clusters: vec![
                        MajorCluster {
                            id: "3.1".to_string(),
                            name: "Software Assessment and QA".to_string(),
                            description: None,
                            courses: vec![
                                MajorCourse {
                                    code: "344-342".to_string(),
                                    name: "Software Testing Techniques".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-441".to_string(),
                                    name: "Software Project and Quality Management".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-442".to_string(),
                                    name: "Software Measurement and Evaluation".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                        MajorCluster {
                            id: "3.2".to_string(),
                            name: "Software Development and Management".to_string(),
                            description: None,
                            courses: vec![
                                MajorCourse {
                                    code: "344-242".to_string(),
                                    name: "Principles of Business Software Development".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-335".to_string(),
                                    name: "Database Application Development".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-443".to_string(),
                                    name: "Object-Oriented Analysis and Design".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                        MajorCluster {
                            id: "3.3".to_string(),
                            name: "UI/UX Design".to_string(),
                            description: None,
                            courses: vec![
                                MajorCourse {
                                    code: "344-343".to_string(),
                                    name: "Introduction to User Experience Design".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-344".to_string(),
                                    name: "Usability Evaluation".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-444".to_string(),
                                    name: "Information Architecture for User Experience Design".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                        MajorCluster {
                            id: "3.4".to_string(),
                            name: "Database Development".to_string(),
                            description: None,
                            courses: vec![
                                MajorCourse {
                                    code: "344-335".to_string(),
                                    name: "Database Application Development".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-433".to_string(),
                                    name: "Database Administration and Maintenance".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-434".to_string(),
                                    name: "Database Performance Tuning".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                    ],
                },

                // Domain 4: AI & Computer Vision
                MajorDomain {
                    id: 4,
                    name: "AI & Computer Vision".to_string(),
                    description: Some("Cluster-based electives for AI and computer vision specialization".to_string()),
                    clusters: vec![
                        MajorCluster {
                            id: "4.1".to_string(),
                            name: "AI".to_string(),
                            description: Some("Choose 1 from: Neural Networks / Pattern Recognition / Internet of Robotic Things".to_string()),
                            courses: vec![
                                MajorCourse {
                                    code: "344-261".to_string(),
                                    name: "Artificial Intelligence for Everyone".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-362".to_string(),
                                    name: "Machine Learning".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-461".to_string(),
                                    name: "Neural Networks".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-462".to_string(),
                                    name: "Pattern Recognition".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-463".to_string(),
                                    name: "Internet of Robotic Things".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                        MajorCluster {
                            id: "4.2".to_string(),
                            name: "Linguistic Intelligence".to_string(),
                            description: None,
                            courses: vec![
                                MajorCourse {
                                    code: "344-363".to_string(),
                                    name: "Natural Language Processing".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-464".to_string(),
                                    name: "Text Mining and Sentiment Analysis".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-465".to_string(),
                                    name: "Linguistic Intelligence and Machine Translation".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                        MajorCluster {
                            id: "4.3".to_string(),
                            name: "Game Programming".to_string(),
                            description: None,
                            courses: vec![
                                MajorCourse {
                                    code: "344-271".to_string(),
                                    name: "3D Modeling and Animation".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-371".to_string(),
                                    name: "Introduction to Computer Game Programming".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-372".to_string(),
                                    name: "Advanced Game Development".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                        MajorCluster {
                            id: "4.4".to_string(),
                            name: "Computer Vision".to_string(),
                            description: None,
                            courses: vec![
                                MajorCourse {
                                    code: "344-373".to_string(),
                                    name: "Fundamentals of Digital Image Processing".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-374".to_string(),
                                    name: "Advanced Digital Image Processing".to_string(),
                                    credits: 3.0,
                                },
                                MajorCourse {
                                    code: "344-471".to_string(),
                                    name: "Computer Vision and Applications".to_string(),
                                    credits: 3.0,
                                },
                            ],
                        },
                    ],
                },
            ],
            others: vec![
                MajorCourse {
                    code: "344-496".to_string(),
                    name: "Special Topics in Computer Science".to_string(),
                    credits: 3.0,
                },
                MajorCourse {
                    code: "344-493".to_string(),
                    name: "Selected Topic in Computer Science I".to_string(),
                    credits: 3.0,
                },
                MajorCourse {
                    code: "344-494".to_string(),
                    name: "Selected Topic in Computer Science II".to_string(),
                    credits: 3.0,
                },
            ],
        },
    }
}
