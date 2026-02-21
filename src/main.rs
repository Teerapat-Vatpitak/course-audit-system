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
use crate::models::{AuditResult, Category, Course, MissingCourse};

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

                                            let mut all_missing: Vec<MissingCourse> = gen_ed_missing;
                                            all_missing.extend(major_missing);

                                            // Drop missing entries for categories that are already complete
                                            let major_collected = major_credits + elective_credits;
                                            all_missing.retain(|m| match m.category.as_str() {
                                                "General Education" => gen_ed_credits < 30.0,
                                                "Basic Science" | "Core Courses" => major_collected < 96.0,
                                                _                   => true,
                                            });

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

        <div class="min-h-screen bg-slate-50 font-sans text-slate-900 flex flex-col">
            // Top bar
            <header class="bg-gradient-to-r from-[#002D62] to-blue-800 text-white px-8 py-4 flex items-baseline gap-4 shadow-md z-10 relative">
                <div class="flex items-center gap-2">
                    <h1 class="text-lg font-bold tracking-tight">"Course Audit"</h1>
                </div>
                <span class="text-blue-300 opacity-50">"|"</span>
                <p class="text-sm text-blue-200 font-medium">"Computer Science"</p>
            </header>

            // Two-column body
            <div class="flex h-[calc(100vh-60px)] p-6 gap-6 max-w-[1600px] w-full mx-auto">

                // Left column — Upload
                <aside class="w-[380px] shrink-0 bg-white rounded-2xl shadow-xl shadow-slate-200/50 flex flex-col p-7 gap-6 border border-slate-100 overflow-hidden relative">
                    <div class="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-blue-500 to-blue-400"></div>
                    <div>
                        <h2 class="text-lg font-semibold text-slate-800">"Upload Transcript"</h2>
                        <p class="text-xs text-slate-500 mt-1">"Upload your unofficial PDF transcript"</p>
                    </div>

                    // Drop zone
                    <div
                        class="group border-2 border-dashed border-slate-200 rounded-xl p-8 text-center hover:border-blue-400 hover:bg-blue-50/50 transition-all duration-300 cursor-pointer flex flex-col items-center justify-center gap-3 relative overflow-hidden"
                        on:dragover=on_drag_over
                        on:drop=on_drop
                    >
                        <div class="w-12 h-12 rounded-full bg-blue-50 text-blue-500 flex items-center justify-center group-hover:scale-110 transition-transform duration-300">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path></svg>
                        </div>
                        <input
                            type="file"
                            accept="application/pdf"
                            class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
                            id="file-input"
                            on:change=on_file_change
                        />
                        <label for="file-input" class="cursor-pointer block">
                            <p class="text-sm font-medium text-slate-700 group-hover:text-blue-600 transition-colors">
                                "Drag & drop your PDF here"
                            </p>
                            <p class="text-xs text-slate-400 mt-1">"or click to browse from your device"</p>
                        </label>
                    </div>
                    
                    // PDF preview (grows to fill remaining space)
                    {move || preview_url.get().map(|url| view! {
                        <div class="flex-1 min-h-0 relative group rounded-xl overflow-hidden border border-slate-200 shadow-inner bg-slate-50">
                            <iframe
                                src={url}
                                class="w-full h-full"
                            ></iframe>
                            <div class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-slate-900/10 to-transparent h-6 pointer-events-none"></div>
                        </div>
                    })}

                    // Spacer when no preview
                    {move || preview_url.get().is_none().then(|| view! {
                        <div class="flex-1 rounded-xl border border-dashed border-slate-100 bg-slate-50/50 flex flex-col items-center justify-center opacity-50">
                            <svg class="w-10 h-10 text-slate-300 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path></svg>
                            <p class="text-xs text-slate-400">"Preview will appear here"</p>
                        </div>
                    })}

                    // Analyse button always at bottom
                    <button
                        class="w-full bg-[#002D62] hover:bg-blue-800 text-white text-sm font-semibold py-3.5 px-4 rounded-xl transition-all duration-300 disabled:bg-slate-100 disabled:text-slate-400 disabled:shadow-none disabled:cursor-not-allowed shadow-lg shadow-blue-900/20 hover:shadow-blue-900/40 active:transform active:scale-[0.98] flex items-center justify-center gap-2"
                        disabled={move || file_name.get().is_empty() || is_loading.get()}
                        on:click=on_start_analysis
                    >
                        {move || if is_loading.get() { 
                            view! { 
                                <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-current" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                                "Analyzing Document..." 
                            }.into_view() 
                        } else { 
                            view! {
                                "Analyze Transcript"
                                <svg class="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
                            }.into_view()
                        }}
                    </button>
                </aside>

                // Right column — Results
                <main class="flex-1 bg-white rounded-2xl shadow-xl shadow-slate-200/50 flex flex-col overflow-hidden relative border border-slate-100">
                    {move || {
                        if is_loading.get() {
                            view! {
                                <div class="flex flex-col items-center justify-center h-full gap-5 z-10 relative">
                                    <div class="relative w-16 h-16">
                                        <div class="absolute inset-0 rounded-full border-4 border-slate-100"></div>
                                        <div class="absolute inset-0 rounded-full border-4 border-[#002D62] border-t-transparent animate-spin"></div>
                                    </div>
                                    <p class="text-sm font-medium text-slate-500 animate-pulse">"Analyzing your academic progress..."</p>
                                </div>
                            }.into_view()
                        } else if let Some(result) = audit_result.get() {
                            view! {
                                <div class="flex flex-col h-full bg-slate-50/50">
                                    // Hero bar
                                    <div class="bg-white px-8 py-8 flex flex-col justify-center border-b border-slate-100 relative overflow-hidden">
                                        <div class="absolute right-0 top-0 w-64 h-64 bg-blue-50/50 rounded-full blur-3xl -mr-20 -mt-20"></div>
                                        <div class="relative">
                                            <p class="text-sm font-semibold text-blue-600 mb-2 tracking-wide uppercase">"Total Progress"</p>
                                            <div class="flex items-baseline gap-3">
                                                <span class="text-6xl font-bold tracking-tight text-slate-900 leading-none">
                                                    {result.total_credits as u32}
                                                </span>
                                                <span class="text-lg font-medium text-slate-400">"credits earned"</span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="overflow-y-auto flex-1 p-6 space-y-6">
                                        // Stat tiles
                                        <div class="grid grid-cols-3 gap-6">
                                            {result.categories.iter().map(|cat| {
                                                let pct = ((cat.collected_credits / cat.required_credits) * 100.0).min(100.0) as u32;
                                                let complete = pct >= 100;
                                                let cat_name = cat.name.clone();
                                                let collected = cat.collected_credits as u32;
                                                let required = cat.required_credits as u32;
                                                view! {
                                                    <div class="bg-white rounded-xl p-5 border border-slate-100 shadow-sm hover:shadow-md transition-shadow relative overflow-hidden group">
                                                        <div class={format!("absolute top-0 left-0 w-1 h-full {}", if complete { "bg-green-500" } else { "bg-blue-500" })}></div>
                                                        <div class="flex justify-between items-start mb-4">
                                                            <p class="text-sm font-semibold text-slate-700">{cat_name}</p>
                                                            {if complete {
                                                                view! { <span class="bg-green-100 text-green-700 text-[10px] font-bold px-2 py-1 rounded-full uppercase tracking-wider">"Complete"</span> }.into_view()
                                                            } else {
                                                                view! { <span class="bg-blue-50 text-blue-600 text-[10px] font-bold px-2 py-1 rounded-full uppercase tracking-wider">"In Progress"</span> }.into_view()
                                                            }}
                                                        </div>
                                                        <div class="flex items-baseline gap-1.5 mb-3">
                                                            <span class="text-3xl font-bold text-slate-900">{collected}</span>
                                                            <span class="text-sm font-medium text-slate-400">{format!("/ {}", required)}</span>
                                                        </div>
                                                        <div class="w-full bg-slate-100 rounded-full h-1.5 mb-1 overflow-hidden">
                                                            <div
                                                                class={format!("h-full rounded-full transition-all duration-1000 ease-out {}", if complete { "bg-green-500" } else { "bg-blue-500" })}
                                                                style={format!("width: {}%", pct)}
                                                            ></div>
                                                        </div>
                                                        <p class="text-[10px] text-slate-400 text-right font-medium">{format!("{}%", pct)}</p>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>

                                        // Course detail accordions
                                        <div class="bg-white rounded-xl border border-slate-100 shadow-sm p-6">
                                            <div class="flex items-center gap-3 mb-5">
                                                <div class="w-8 h-8 rounded-lg bg-blue-50 text-blue-600 flex items-center justify-center">
                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path></svg>
                                                </div>
                                                <h3 class="text-lg font-semibold text-slate-800">"Course Details"</h3>
                                            </div>
                                            <div class="space-y-3">
                                                {result.categories.iter().map(|category| {
                                                    let category = category.clone();
                                                    view! { <CategoryCard category={category} /> }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        </div>

                                        // Missing courses grouped by category
                                        {(!result.missing_subjects.is_empty()).then(|| {
                                            let mut seen_cats: Vec<String> = Vec::new();
                                            for m in &result.missing_subjects {
                                                if !seen_cats.contains(&m.category) {
                                                    seen_cats.push(m.category.clone());
                                                }
                                            }
                                            view! {
                                                <div class="bg-white rounded-xl border border-rose-100 shadow-sm p-6 relative overflow-hidden">
                                                    <div class="absolute top-0 left-0 w-1 h-full bg-rose-400"></div>
                                                    <div class="flex items-center gap-3 mb-5">
                                                        <div class="w-8 h-8 rounded-lg bg-rose-50 text-rose-500 flex items-center justify-center">
                                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
                                                        </div>
                                                        <h3 class="text-lg font-semibold text-slate-800">"Missing Requirements"</h3>
                                                    </div>
                                                    <div class="space-y-5">
                                                        {seen_cats.iter().map(|cat| {
                                                            let cat_courses: Vec<_> = result.missing_subjects.iter()
                                                                .filter(|m| &m.category == cat)
                                                                .collect();
                                                            let cat_label = cat.clone();
                                                            view! {
                                                                <div>
                                                                    <p class="text-sm font-medium text-slate-700 mb-2">{cat_label}</p>
                                                                    <div class="bg-rose-50/50 rounded-lg border border-rose-100 divide-y divide-rose-100">
                                                                        {cat_courses.iter().map(|m| {
                                                                            let desc = m.description.clone();
                                                                            view! {
                                                                                <div class="px-4 py-3 flex items-start gap-3">
                                                                                    <div class="w-1.5 h-1.5 rounded-full bg-rose-400 mt-1.5 shrink-0"></div>
                                                                                    <p class="text-sm text-slate-600">{desc}</p>
                                                                                </div>
                                                                            }
                                                                        }).collect::<Vec<_>>()}
                                                                    </div>
                                                                </div>
                                                            }
                                                        }).collect::<Vec<_>>()}
                                                    </div>
                                                </div>
                                            }
                                        })}
                                    </div>
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div class="flex flex-col items-center justify-center h-full text-center px-12 z-10 relative">
                                    <div class="w-24 h-24 bg-slate-50 rounded-full flex items-center justify-center mb-6 border-8 border-white shadow-sm">
                                        <svg class="w-10 h-10 text-slate-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9.5L18.5 7H20"></path></svg>
                                    </div>
                                    <h3 class="text-xl font-semibold text-slate-800 mb-2">"No Transcript Selected"</h3>
                                    <p class="text-sm text-slate-500 max-w-sm">"Upload your academic transcript PDF on the left to see your progress, completed courses, and missing requirements."</p>
                                </div>
                            }.into_view()
                        }
                    }}
                </main>
            </div>
        </div>
    }
}
