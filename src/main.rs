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

use leptos::*;
use leptos_meta::*;
use std::collections::HashSet;
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
use crate::models::{
    free_elective_dedupe_key, is_passing_grade, AuditResult, Category, Course, MissingCourse,
};

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
    let (error_msg, set_error_msg) = create_signal(Option::<String>::None);

    // Handle file selection from input field
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

    // Handle start analysis
    let on_start_analysis = move |_| {
        if file_name.get().is_empty() {
            return;
        }

        set_is_loading.set(true);
        set_audit_result.set(None);
        set_error_msg.set(None);

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
                            // SAFETY: Closure::forget leaks memory but is the standard
                            // wasm-bindgen pattern for one-shot callbacks. Each analysis
                            // leaks a small, bounded amount — acceptable for this use case.
                            onload.forget();

                            let onerror = Closure::once(move |_event: web_sys::Event| {
                                reject
                                    .call1(&JsValue::NULL, &JsValue::from_str("Error reading file"))
                                    .unwrap();
                            });
                            reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
                            onerror.forget(); // See onload.forget() comment above
                        });

                        if reader.read_as_array_buffer(&file).is_err() {
                            set_is_loading.set(false);
                            set_error_msg.set(Some(
                                "Failed to read the PDF file. Please try again.".to_string(),
                            ));
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
                                            let courses = parse_transcript(&text);

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

                                            let (free_elective_credits, _free_elective_list) =
                                                calculate_free_electives(
                                                    &courses,
                                                    &all_used_courses,
                                                );

                                            let mut all_missing: Vec<MissingCourse> =
                                                gen_ed_missing;
                                            all_missing.extend(major_missing);

                                            // Drop missing entries for GenEd if total GenEd credits are already met.
                                            // DO NOT drop Major Core/Basic Science misses, as they are strictly required regardless of total accumulated elective credits.
                                            all_missing.retain(|m| match m.category.as_str() {
                                                "General Education" => {
                                                    gen_ed_credits < gen_ed.total_required_credits
                                                }
                                                _ => true,
                                            });

                                            let total_credits = gen_ed_credits
                                                + major_credits
                                                + elective_credits
                                                + free_elective_credits;

                                            let mut gen_ed_courses = Vec::new();
                                            let mut major_courses = Vec::new();
                                            let mut free_elective_courses = Vec::new();
                                            let mut seen_free_electives: HashSet<String> =
                                                HashSet::new();

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
                                                } else if is_passing_grade(&parsed.grade) {
                                                    let dedupe_key = free_elective_dedupe_key(
                                                        &parsed.code,
                                                        &parsed.name,
                                                    );
                                                    if seen_free_electives.insert(dedupe_key) {
                                                        free_elective_courses.push(course);
                                                    }
                                                }
                                            }

                                            let audit_result = AuditResult {
                                                total_credits,
                                                categories: vec![
                                                    Category {
                                                        name: "General Education".to_string(),
                                                        required_credits: gen_ed
                                                            .total_required_credits,
                                                        collected_credits: gen_ed_credits,
                                                        courses: gen_ed_courses,
                                                    },
                                                    Category {
                                                        name: "Major Courses".to_string(),
                                                        required_credits: major
                                                            .total_required_credits,
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
                                            set_error_msg.set(Some("Could not extract text from the PDF. Make sure it's a valid transcript.".to_string()));
                                        }
                                    }
                                    Err(_e) => {
                                        set_is_loading.set(false);
                                        set_error_msg.set(Some("PDF extraction failed. The file may be corrupted or encrypted.".to_string()));
                                    }
                                }
                            }
                            Err(_e) => {
                                set_is_loading.set(false);
                                set_error_msg
                                    .set(Some("Failed to read the uploaded file.".to_string()));
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

        <div class="min-h-screen bg-transparent font-sans text-slate-900 flex flex-col selection:bg-blue-200">
            // Top bar
            <header class="bg-white/80 backdrop-blur-xl border-b border-white/50 text-slate-900 px-8 py-4 flex items-center gap-4 shadow-sm z-10 sticky top-0 transition-all">
                <div class="flex items-center gap-3">
                    <div class="w-8 h-8 rounded-xl bg-gradient-to-br from-[#002D62] to-blue-500 flex items-center justify-center shadow-lg shadow-blue-500/30">
                        <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path></svg>
                    </div>
                    <h1 class="text-xl font-extrabold tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-[#002D62] to-blue-600">"Course Audit"</h1>
                </div>
                <div class="h-5 w-px bg-slate-200 mx-1"></div>
                <p class="text-sm text-slate-500 font-medium tracking-wide">"Computer Science"</p>
            </header>

            // Two-column body (Responsive)
            <div class="flex flex-col lg:flex-row h-[calc(100vh-73px)] p-4 lg:p-6 gap-6 max-w-[1600px] w-full mx-auto relative z-0 animate-fade-in-up md:overflow-hidden overflow-auto">
                // Left column — Upload & Instructions
                <aside class="w-full lg:w-[380px] shrink-0 bg-white/70 backdrop-blur-2xl rounded-[2rem] shadow-[0_8px_30px_rgb(0,0,0,0.04)] ring-1 ring-white flex flex-col p-6 gap-5 relative overflow-y-auto overflow-x-hidden scrollbar-hide transition-all duration-300 hover:shadow-[0_8px_30px_rgb(0,0,0,0.08)]">
                    <div class="absolute inset-0 bg-gradient-to-br from-white/40 to-white/10 pointer-events-none"></div>
                    <div class="absolute top-0 left-0 w-full h-1.5 bg-gradient-to-r from-blue-500 to-[#002D62] shrink-0"></div>
                    <div class="relative z-10 shrink-0">
                        <h2 class="text-2xl font-bold tracking-tight text-slate-800">"Transcript"</h2>
                        <p class="text-sm text-slate-500 mt-1 leading-relaxed">"Upload your unofficial PDF transcript to begin assessing your progress."</p>
                    </div>

                    // Drop zone
                    <div
                        class="relative z-10 shrink-0 group border-2 border-dashed border-slate-300/80 rounded-2xl p-6 text-center bg-white/50 hover:border-blue-400 hover:bg-blue-50/50 transition-all duration-300 cursor-pointer flex flex-col items-center justify-center gap-3"
                        on:dragover=on_drag_over
                        on:drop=on_drop
                    >
                        <div class="w-14 h-14 rounded-2xl bg-white shadow-sm ring-1 ring-slate-900/5 text-blue-500 flex items-center justify-center group-hover:scale-110 group-hover:bg-blue-500 group-hover:text-white transition-all duration-300">
                            <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path></svg>
                        </div>
                        <input
                            type="file"
                            accept="application/pdf"
                            class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
                            id="file-input"
                            on:change=on_file_change
                        />
                        <label for="file-input" class="cursor-pointer block relative z-10">
                            <p class="text-[15px] font-semibold text-slate-700 group-hover:text-blue-600 transition-colors">
                                "Drag & drop your PDF here"
                            </p>
                            <p class="text-[13px] text-slate-400 mt-1.5 font-medium">"or click to browse from your device"</p>
                        </label>
                    </div>

                    // Selected File Indicator
                    {move || (!file_name.get().is_empty()).then(|| view! {
                        <div class="flex items-center gap-3 bg-white px-4 py-3 rounded-xl border border-blue-100 shadow-sm ring-1 ring-blue-50/50 relative z-10 animate-fade-in-up shrink-0">
                            <svg class="w-5 h-5 text-green-500 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                            <p class="text-sm text-slate-700 truncate font-semibold">{file_name.get()}</p>
                        </div>
                    })}

                    // Instructions (replaces PDF preview)
                    {move || preview_url.get().is_none().then(|| view! {
                        <div class="relative z-10 mt-1 shrink-0">
                            <h3 class="text-xs font-bold text-slate-800 uppercase tracking-widest mb-3">"How it works"</h3>
                            <ul class="space-y-3">
                                <li class="flex items-start gap-3">
                                    <div class="mt-0.5 shrink-0 w-5 h-5 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center text-[10px] font-bold">1</div>
                                    <p class="text-[13.5px] text-slate-600 leading-relaxed font-medium">"Download your transcript from SIS PSU."</p>
                                </li>
                                <li class="flex items-start gap-3">
                                    <div class="mt-0.5 shrink-0 w-5 h-5 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center text-[10px] font-bold">2</div>
                                    <p class="text-[13.5px] text-slate-600 leading-relaxed font-medium">"Upload the PDF file using the drop zone above."</p>
                                </li>
                                <li class="flex items-start gap-3">
                                    <div class="mt-0.5 shrink-0 w-5 h-5 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center text-[10px] font-bold">3</div>
                                    <p class="text-[13.5px] text-slate-600 leading-relaxed font-medium">"Click Analyze. The system will process everything locally on your device for maximum privacy."</p>
                                </li>
                            </ul>
                        </div>
                    })}

                    // Spacer when file is selected so button stays at bottom
                    {move || preview_url.get().is_some().then(|| view! {
                        <div class="flex-1"></div>
                    })}

                    // Analyse button always at bottom
                    <div class="mt-auto pt-2 shrink-0">
                        <button
                            class="relative w-full overflow-hidden bg-gradient-to-r from-[#002D62] to-blue-600 hover:from-blue-700 hover:to-blue-500 text-white text-[15px] font-semibold py-3.5 px-4 rounded-xl transition-all duration-300 disabled:opacity-50 disabled:shadow-none disabled:cursor-not-allowed shadow-[0_8px_20px_rgba(0,45,98,0.25)] hover:shadow-[0_12px_25px_rgba(0,45,98,0.35)] active:transform active:scale-[0.98] flex items-center justify-center gap-2 group z-10"
                            disabled={move || file_name.get().is_empty() || is_loading.get()}
                            on:click=on_start_analysis
                        >
                        {move || if is_loading.get() {
                            view! {
                                <svg class="animate-spin -ml-1 mr-2 h-5 w-5 text-current" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                                "Analyzing Document..."
                            }.into_view()
                        } else {
                            view! {
                                "Analyze Transcript"
                                <svg class="w-5 h-5 ml-1 transform group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
                            }.into_view()
                        }}
                        </button>
                    </div>
                </aside>

                // Right column — Results
                <main class="flex-1 bg-white/70 backdrop-blur-2xl rounded-[2rem] shadow-[0_8px_30px_rgb(0,0,0,0.04)] ring-1 ring-white flex flex-col overflow-hidden relative transition-all duration-300 h-[800px] lg:h-auto min-h-[600px]">
                    {move || {
                        if is_loading.get() {
                            view! {
                                <div class="flex flex-col items-center justify-center h-full gap-5 z-10 relative">
                                    <div class="relative w-16 h-16">
                                        <div class="absolute inset-0 rounded-full border-4 border-slate-100/50"></div>
                                        <div class="absolute inset-0 rounded-full border-4 border-[#002D62] border-t-transparent animate-spin ring-1 ring-white/50"></div>
                                    </div>
                                    <p class="text-[15px] font-medium text-slate-500 animate-pulse tracking-wide">"Analyzing your academic progress..."</p>
                                </div>
                            }.into_view()
                        } else if let Some(err) = error_msg.get() {
                            view! {
                                <div class="flex flex-col items-center justify-center h-full gap-5 z-10 relative px-12 text-center">
                                    <div class="w-16 h-16 rounded-full bg-rose-50 flex items-center justify-center">
                                        <svg class="w-8 h-8 text-rose-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path></svg>
                                    </div>
                                    <h3 class="text-xl font-bold text-slate-800">"Something went wrong"</h3>
                                    <p class="text-[15px] text-slate-500 max-w-md leading-relaxed font-medium">{err}</p>
                                </div>
                            }.into_view()
                        } else if let Some(result) = audit_result.get() {
                            view! {
                                <div class="flex flex-col h-full bg-transparent">
                                    // Hero bar
                                    <div class="bg-white/40 backdrop-blur-md px-10 py-10 flex flex-col justify-center border-b border-white/60 relative overflow-hidden">
                                        <div class="absolute right-0 top-0 w-80 h-80 bg-blue-100/50 rounded-full blur-3xl -mr-20 -mt-20 pointer-events-none mix-blend-multiply"></div>
                                        <div class="relative">
                                            <p class="text-[13px] font-bold text-blue-600/80 mb-3 tracking-[0.2em] uppercase">"Total Progress"</p>
                                            <div class="flex items-baseline gap-4">
                                                <span class="text-[5rem] font-extrabold tracking-tight text-slate-800 leading-none drop-shadow-sm">
                                                    {result.total_credits as u32}
                                                </span>
                                                <span class="text-xl font-medium text-slate-400">"credits earned"</span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="overflow-y-auto flex-1 p-6 space-y-6">
                                        // Stat tiles
                                        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
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
                                                            let display_items: Vec<String> = if cat == "General Education" {
                                                                let mut ge_groups: Vec<String> = Vec::new();

                                                                for m in &cat_courses {
                                                                    let description = m.description.trim();
                                                                    let group = if description.contains("missing") {
                                                                        description.to_string()
                                                                    } else {
                                                                        description
                                                                            .split(':')
                                                                            .next()
                                                                            .unwrap_or(description)
                                                                            .trim()
                                                                            .to_string()
                                                                    };

                                                                    if !ge_groups.contains(&group) {
                                                                        ge_groups.push(group);
                                                                    }
                                                                }

                                                                ge_groups
                                                            } else {
                                                                cat_courses
                                                                    .iter()
                                                                    .map(|m| m.description.clone())
                                                                    .collect()
                                                            };
                                                            let cat_label = cat.clone();
                                                            view! {
                                                                <div>
                                                                    <p class="text-sm font-medium text-slate-700 mb-2">{cat_label}</p>
                                                                    <div class="bg-rose-50/50 rounded-lg border border-rose-100 divide-y divide-rose-100">
                                                                        {display_items.iter().map(|item| {
                                                                            let desc = item.clone();
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
                                    <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_center,_var(--tw-gradient-stops))] from-blue-50/40 via-transparent to-transparent opacity-60 pointer-events-none"></div>
                                    <div class="w-28 h-28 bg-white/80 backdrop-blur-xl rounded-[2rem] flex items-center justify-center mb-8 shadow-[0_8px_30px_rgb(0,0,0,0.06)] ring-1 ring-white transform transition-transform hover:scale-105 duration-500 relative group">
                                        <div class="absolute inset-0 bg-gradient-to-br from-blue-100/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity rounded-[2rem]"></div>
                                        <svg class="w-12 h-12 text-blue-500/80 drop-shadow-sm transition-transform duration-500 group-hover:-translate-y-1" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9.5L18.5 7H20"></path></svg>
                                    </div>
                                    <h3 class="text-2xl font-bold tracking-tight text-slate-800 mb-3 drop-shadow-sm">"Waiting for Transcript"</h3>
                                    <p class="text-[15px] text-slate-500 max-w-md leading-relaxed font-medium">"Upload your academic transcript PDF on the left. We'll instantly analyze your progress, completed courses, and missing requirements."</p>
                                </div>
                            }.into_view()
                        }
                    }}
                </main>
            </div>
        </div>
    }
}
