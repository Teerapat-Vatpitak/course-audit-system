//! Course Audit System - Rust + Leptos WebAssembly Application
//!
//! A privacy-first tool for Computer Science students at Prince of Songkla University (PSU).
//! Uploads transcripts locally as PDF, parses them, and audits progress against curriculum
//! requirements for General Education, Major, and Electives—all in the browser.
//!
//! **Key Features:**
//! - Client-side WASM application (no server calls)
//! - PDF parsing using Regex for course extraction
//! - Comprehensive curriculum auditing with sub-group support
//! - Greedy matching for repeatable courses
//! - Responsive Leptos UI with collapsible category cards

use leptos::{logging, *};
use leptos_meta::*;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{DragEvent, Event, HtmlInputElement};

mod components;
mod data;
mod logic;
mod models;

use crate::components::category_card::CategoryCard;
use crate::data::{gen_ed::get_gen_ed_curriculum, major::get_major_curriculum};
use crate::logic::{
    auditor::{audit_gen_ed, audit_major, calculate_free_electives},
    parser::{extract_text_from_pdf, parse_transcript},
};
use crate::models::{AuditResult, Category, Course};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

/// Main application component
/// 
/// Manages state for file upload, PDF preview, audit results, and loading state.
/// Implements drag-and-drop PDF upload with real-time PDF preview.
#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    // State management for file upload and audit processing
    let (file_name, set_file_name) = create_signal(String::new());
    let (preview_url, set_preview_url) = create_signal(Option::<String>::None);
    let (audit_result, set_audit_result) = create_signal(Option::<AuditResult>::None);
    let (is_loading, set_is_loading) = create_signal(false);

    /// Handle file selection from input field
    let on_file_change = move |ev: Event| {
        let input = ev
            .target()
            .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    set_file_name.set(file.name());

                    // Create blob URL for PDF preview display
                    if let Ok(url) = web_sys::Url::create_object_url_with_blob(&file) {
                        set_preview_url.set(Some(url));
                    }
                }
            }
        }
    };

    // Handle drag over
    let on_drag_over = move |ev: DragEvent| {
        ev.prevent_default();
    };

    // Handle drop
    let on_drop = move |ev: DragEvent| {
        ev.prevent_default();

        if let Some(data_transfer) = ev.data_transfer() {
            if let Some(files) = data_transfer.files() {
                if let Some(file) = files.get(0) {
                    set_file_name.set(file.name());

                    // Create blob URL for PDF preview
                    if let Ok(url) = web_sys::Url::create_object_url_with_blob(&file) {
                        set_preview_url.set(Some(url));
                    }
                }
            }
        }
    };

    // Handle start analysis - Call backend server
    let on_start_analysis = move |_| {
        if file_name.get().is_empty() {
            return;
        }

        set_is_loading.set(true);
        set_audit_result.set(None);

        if let Ok(input) = web_sys::window()
            .ok_or(())
            .and_then(|w| w.document().ok_or(()))
            .and_then(|d| {
                d.get_element_by_id("file-input")
                    .ok_or(())
                    .and_then(|e| e.dyn_into::<HtmlInputElement>().ok().ok_or(()))
            })
        {
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let file = web_sys::File::from(file);

                    spawn_local(async move {
                        use wasm_bindgen_futures::JsFuture;
                        use web_sys::FileReader;

                        let reader = FileReader::new().unwrap();

                        let promise = js_sys::Promise::new(&mut |resolve, reject| {
                            let reader_clone = reader.clone();
                            let reject_clone = reject.clone();
                            let onload = Closure::once(move |_event: web_sys::Event| {
                                if let Ok(result) = reader_clone.result() {
                                    resolve.call1(&JsValue::NULL, &result).unwrap();
                                } else {
                                    reject_clone
                                        .call1(
                                            &JsValue::NULL,
                                            &JsValue::from_str("Failed to read file"),
                                        )
                                        .unwrap();
                                }
                            });
                            reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                            onload.forget();

                            let onerror = Closure::once(move |_event: web_sys::Event| {
                                reject
                                    .call1(&JsValue::NULL, &JsValue::from_str("Error reading file"))
                                    .unwrap();
                            });
                            reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                            onerror.forget();
                        });

                        if reader.read_as_array_buffer(&file).is_err() {
                            set_is_loading.set(false);
                            logging::error!("Failed to read file");
                            return;
                        }

                        // Wait for the file to be loaded
                        match JsFuture::from(promise).await {
                            Ok(result) => {
                                let array_buffer = js_sys::ArrayBuffer::from(result);
                                let uint8_array = js_sys::Uint8Array::new(&array_buffer);

                                let promise = extract_text_from_pdf(uint8_array);
                                match JsFuture::from(promise).await {
                                    Ok(text_value) => {
                                        if let Some(text) = text_value.as_string() {
                                            logging::log!(
                                                "[DEBUG] Extracted text length: {} characters",
                                                text.len()
                                            );

                                            let courses = parse_transcript(&text);
                                            logging::log!(
                                                "[DEBUG] Starting audit with {} courses",
                                                courses.len()
                                            );

                                            let gen_ed = get_gen_ed_curriculum();
                                            let major = get_major_curriculum();

                                            let (gen_ed_credits, gen_ed_missing, gen_ed_used) =
                                                audit_gen_ed(&courses, &gen_ed);
                                            let (
                                                major_credits,
                                                elective_credits,
                                                major_missing,
                                                major_used,
                                            ) = audit_major(&courses, &major);

                                            let mut all_used_courses = gen_ed_used.clone();
                                            all_used_courses.extend(major_used.clone());

                                            let (free_elective_credits, free_elective_list) =
                                                calculate_free_electives(
                                                    &courses,
                                                    &all_used_courses,
                                                );

                                            logging::log!(
                                                "[AUDIT] Free Elective courses: {:?}",
                                                free_elective_list
                                            );

                                            let mut all_missing = gen_ed_missing;
                                            all_missing.extend(major_missing);

                                            let total_credits = gen_ed_credits
                                                + major_credits
                                                + elective_credits
                                                + free_elective_credits;

                                            let mut gen_ed_courses = Vec::new();
                                            let mut major_courses = Vec::new();
                                            let mut free_elective_courses = Vec::new();

                                            for (idx, parsed) in courses.iter().enumerate() {
                                                let course = Course {
                                                    code: parsed.code.clone(),
                                                    name: parsed.name.clone(),
                                                    credit: parsed.parsed_credit,
                                                    grade: parsed.grade.clone(),
                                                };

                                                if gen_ed_used.contains(&idx) {
                                                    gen_ed_courses.push(course);
                                                } else if major_used.contains(&idx) {
                                                    major_courses.push(course);
                                                } else if all_used_courses.contains(&idx) {
                                                    major_courses.push(course);
                                                } else {
                                                    free_elective_courses.push(course);
                                                }
                                            }

                                            let audit_result = AuditResult {
                                                total_credits,
                                                categories: vec![
                                                    Category {
                                                        name: "General Education".to_string(),
                                                        required_credits: 30.0,
                                                        collected_credits: gen_ed_credits,
                                                        courses: gen_ed_courses,
                                                    },
                                                    Category {
                                                        name: "Major Courses".to_string(),
                                                        required_credits: 96.0,
                                                        collected_credits: major_credits
                                                            + elective_credits,
                                                        courses: major_courses,
                                                    },
                                                    Category {
                                                        name: "Free Electives".to_string(),
                                                        required_credits: 6.0,
                                                        collected_credits: free_elective_credits,
                                                        courses: free_elective_courses,
                                                    },
                                                ],
                                                missing_subjects: all_missing,
                                            };

                                            set_is_loading.set(false);
                                            set_audit_result.set(Some(audit_result));
                                        } else {
                                            set_is_loading.set(false);
                                            logging::error!("Failed to extract text from PDF");
                                        }
                                    }
                                    Err(e) => {
                                        set_is_loading.set(false);
                                        logging::error!("PDF extraction failed: {:?}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                set_is_loading.set(false);
                                logging::error!("Failed to read file: {:?}", e);
                            }
                        }
                    });
                }
            }
        }
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/course-audit-system.css"/>
        <Title text="Course Audit System"/>

        <div class="min-h-screen bg-gray-50 py-12 px-6">
            <div class="max-w-5xl mx-auto">
                <h1 class="text-5xl font-semibold text-gray-900 mb-12 text-center tracking-tight">
                    "Course Audit System"
                </h1>

                // Top Section - Upload
                <div class="bg-white rounded-xl border border-gray-200 shadow-sm p-8 mb-8">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-6">
                        "Upload Transcript"
                    </h2>

                    // Drag-and-drop zone
                    <div
                        class="bg-white border-2 border-dashed border-gray-300 rounded-xl p-12 text-center hover:border-emerald-500 hover:bg-emerald-50 transition-colors duration-200 cursor-pointer"
                        on:dragover=on_drag_over
                        on:drop=on_drop
                    >
                        <input
                            type="file"
                            accept="application/pdf"
                            class="hidden"
                            id="file-input"
                            on:change=on_file_change
                        />
                        <label
                            for="file-input"
                            class="cursor-pointer block"
                        >
                            <div>
                                <svg class="mx-auto h-12 w-12 text-emerald-600 mb-4" stroke="currentColor" fill="none" viewBox="0 0 24 24" stroke-width="1.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5" />
                                </svg>
                                <p class="text-base font-semibold text-gray-900 mb-1">"Drop your PDF transcript here"</p>
                                <p class="text-sm text-gray-600">"or click to browse files"</p>
                            </div>
                        </label>
                    </div>

                    // Display filename
                    {move || (!file_name.get().is_empty()).then(|| view! {
                        <div class="mt-6 p-4 bg-emerald-50 rounded-lg border border-emerald-200">
                            <p class="text-sm text-gray-900">
                                <span class="font-medium text-gray-600">"Selected file: "</span>
                                <span class="font-semibold text-emerald-700">{file_name.get()}</span>
                            </p>
                        </div>
                    })}

                    // PDF Preview
                    {move || preview_url.get().map(|url| view! {
                        <div class="mt-8">
                            <h3 class="text-base font-semibold text-gray-900 mb-4">"Preview"</h3>
                            <iframe
                                src={url}
                                class="w-full border border-gray-200 rounded-lg shadow-sm"
                                style="height: 500px;"
                            ></iframe>
                        </div>
                    })}

                    // Start Analysis Button
                    <button
                        class="mt-8 w-full bg-emerald-600 hover:bg-emerald-700 text-white font-medium py-3.5 px-6 rounded-lg shadow-sm transition-colors duration-200 disabled:bg-gray-300 disabled:text-gray-500 disabled:cursor-not-allowed disabled:shadow-none"
                        disabled={move || file_name.get().is_empty() || is_loading.get()}
                        on:click=on_start_analysis
                    >
                        {move || if is_loading.get() {
                            "Processing..."
                        } else {
                            "Start Analysis"
                        }}
                    </button>
                </div>

                // Bottom Section - Dashboard
                <div class="bg-white rounded-xl border border-gray-200 shadow-sm p-8">
                    <h2 class="text-2xl font-semibold text-gray-900 mb-6">
                        "Audit Results"
                    </h2>

                    {move || {
                        if is_loading.get() {
                            // Loading state
                            view! {
                                <div class="text-center py-16">
                                    <div class="inline-block animate-spin rounded-full h-10 w-10 border-2 border-gray-200 border-t-emerald-600 mb-4"></div>
                                    <p class="text-gray-600 text-sm font-medium">"Analyzing transcript..."</p>
                                </div>
                            }.into_view()
                        } else if let Some(result) = audit_result.get() {
                            // Display results
                            view! {
                                <div>
                                    // Total Credits
                                    <div class="bg-gradient-to-br from-emerald-600 to-emerald-700 text-white p-8 rounded-xl mb-8 shadow-lg">
                                        <h3 class="text-sm font-semibold text-emerald-100 uppercase tracking-wide mb-2">"Total Credits Earned"</h3>
                                        <p class="text-5xl font-bold">{result.total_credits.to_string()}</p>
                                    </div>

                                    // Categories with Collapsible Cards
                                    <h3 class="text-lg font-semibold text-gray-900 mb-6">"Credits by Category"</h3>
                                    <div class="space-y-4 mb-8">
                                        {result.categories.iter().map(|category| {
                                            let category = category.clone();
                                            view! {
                                                <CategoryCard category={category} />
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>

                                    // Missing Subjects
                                    {(!result.missing_subjects.is_empty()).then(|| view! {
                                        <div class="bg-red-50 border border-red-200 p-6 rounded-xl">
                                            <h4 class="font-semibold text-red-900 mb-3 flex items-center gap-2">
                                                <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                                                    <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
                                                </svg>
                                                "Missing Required Courses"
                                            </h4>
                                            <ul class="space-y-2">
                                                {result.missing_subjects.iter().map(|subject| {
                                                    view! {
                                                        <li class="text-sm text-red-800 flex items-start gap-2 font-medium">
                                                            <span class="text-red-500 mt-0.5">"•"</span>
                                                            <span>{subject}</span>
                                                        </li>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </ul>
                                        </div>
                                    })}
                                </div>
                            }.into_view()
                        } else {
                            // Empty state
                            view! {
                                <div class="text-center py-16">
                                    <div class="inline-flex items-center justify-center w-14 h-14 rounded-full bg-gray-100 mb-4">
                                        <svg class="w-7 h-7 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                                        </svg>
                                    </div>
                                    <p class="text-gray-600 font-medium">"Upload a transcript to view audit results"</p>
                                </div>
                            }.into_view()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}
