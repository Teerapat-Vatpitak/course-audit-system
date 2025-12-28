//! Core business logic for transcript parsing and curriculum auditing
//!
//! **Parser:** Extracts course data from PDF transcripts using Regex patterns
//! **Auditor:** Implements curriculum validation logic including:
//! - General Education strand matching with sequential pair constraints
//! - Major course requirement matching
//! - Free elective detection and credit accumulation
//! - Greedy matching for repeatable courses

pub mod auditor;
pub mod parser;
