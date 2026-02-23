//! PDF Transcript Parser
//!
//! Extracts course data from PDF transcripts using Regex patterns.
//! Uses JavaScript interop (via `wasm-bindgen`) to access PDF.js for text extraction,
//! then parses course entries (code, name, credits, grade) from extracted text.

use crate::models::ParsedCourse;
use regex::Regex;
use wasm_bindgen::prelude::*;

fn normalize_course_code(raw_code: &str) -> String {
    let trimmed = if raw_code.len() >= 7 && raw_code.chars().nth(3) == Some('-') {
        raw_code[..7].to_string()
    } else {
        raw_code.to_string()
    };

    match trimmed.as_str() {
        "890-001" => "890-101".to_string(),
        "890-002" => "890-102".to_string(),
        "890-003" => "890-103".to_string(),
        "890-004" => "890-104".to_string(),
        "890-005" => "890-105".to_string(),
        _ => trimmed,
    }
}

/// JavaScript interop function exposed by the PDF extractor in the frontend runtime.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = extractTextFromPDF)]
    pub fn extract_text_from_pdf(array_buffer: js_sys::Uint8Array) -> js_sys::Promise;
}

/// Parses transcript text into structured course entries, normalizing codes and
/// greedily numbering special-topic courses (e.g., 344-496 Topic 1, Topic 2).
pub fn parse_transcript(text: &str) -> Vec<ParsedCourse> {
    let mut courses = Vec::new();
    let mut special_topics_count: std::collections::HashMap<String, u32> =
        std::collections::HashMap::new();

    // Pattern: course code followed by name, section, credit, then grade
    // Example: 322-101   CALCULUS I   04   3   B+
    let pattern = Regex::new(
        r"([A-Za-z0-9]{3}-?\d{3}[A-Za-z]?\d*[A-Za-z]?)\s+([A-Za-z0-9\s:()&\-\.,'/\*]+?)\s+(\d+)\s+(\d+)\s+([A-D][+]?|[FWPSUGE])",
    )
    .unwrap();

    for captures in pattern.captures_iter(text) {
        let raw_code = captures.get(1).unwrap().as_str();
        let name = captures.get(2).unwrap().as_str().trim();
        let parsed_credit_str = captures.get(4).unwrap().as_str();
        let grade = captures.get(5).unwrap().as_str().to_uppercase();

        let parsed_credit = parsed_credit_str.parse::<f32>().unwrap_or(3.0);

        // Normalize course code by trimming suffix (e.g., 890-103G1 -> 890-103)
        // and applying known equivalence mappings used by the curriculum.
        let normalized_code = normalize_course_code(raw_code);

        // Greedy match: Special topics (344-496 to 344-499) might be repeated.
        // We handle any course starting with 344-49, EXCEPT the specific Capstone/Core ones.
        let is_special_topic = normalized_code.starts_with("344-49")
            && !matches!(
                normalized_code.as_str(),
                "344-491" | "344-492" | "344-493" | "344-494" | "344-495"
            );

        let final_name = if is_special_topic {
            let counter = special_topics_count
                .entry(normalized_code.clone())
                .or_insert(0);
            *counter += 1;
            format!("{} (Topic {})", name, counter)
        } else {
            name.to_string()
        };

        courses.push(ParsedCourse {
            code: normalized_code,
            name: final_name,
            grade,
            parsed_credit,
        });
    }

    courses
}
