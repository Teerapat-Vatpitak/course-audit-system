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
        <Title text="Course Audit — PSU CS"/>

        <div class="min-h-screen font-sans text-zinc-900 flex flex-col selection:bg-brand-100">

            // ── Navbar ──────────────────────────────────────────────────
            <header class="sticky top-0 z-50 border-b border-zinc-200/60 bg-white/80 backdrop-blur-xl backdrop-saturate-150">
                <div class="max-w-[1440px] mx-auto px-4 sm:px-6 lg:px-8 h-14 flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div class="w-7 h-7 rounded-lg bg-brand-600 flex items-center justify-center">
                            <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" stroke-width="2.2" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4.26 10.147a60.436 60.436 0 00-.491 6.347A48.627 48.627 0 0112 20.904a48.627 48.627 0 018.232-4.41 60.46 60.46 0 00-.491-6.347m-15.482 0a50.57 50.57 0 00-2.658-.813A59.905 59.905 0 0112 3.493a59.902 59.902 0 0110.399 5.84c-.896.248-1.783.52-2.658.814m-15.482 0A50.697 50.697 0 0112 13.489a50.702 50.702 0 017.74-3.342"/>
                            </svg>
                        </div>
                        <span class="text-[15px] font-semibold tracking-tight text-zinc-900">"Course Audit"</span>
                        <span class="hidden sm:inline text-xs font-medium text-zinc-400 bg-zinc-100 px-2 py-0.5 rounded-md">"CS · PSU"</span>
                    </div>
                    <div class="flex items-center gap-2 text-xs text-zinc-400">
                        <div class="w-1.5 h-1.5 rounded-full bg-emerald-400 pulse-dot"></div>
                        <span class="hidden sm:inline font-medium">"100% client-side"</span>
                    </div>
                </div>
            </header>

            // ── Main Content ────────────────────────────────────────────
            <div class="flex-1 flex flex-col lg:flex-row max-w-[1440px] w-full mx-auto p-4 sm:p-6 gap-5 animate-fade-in">

                // ── Left Sidebar ────────────────────────────────────────
                <aside class="w-full lg:w-[360px] shrink-0 flex flex-col gap-4">

                    // Upload Card
                    <div class="bg-white rounded-2xl border border-zinc-200/80 shadow-soft p-5 flex flex-col gap-4">
                        <div>
                            <h2 class="text-base font-semibold text-zinc-900 tracking-tight">"Upload Transcript"</h2>
                            <p class="text-[13px] text-zinc-500 mt-0.5 leading-relaxed">"Your PDF is processed entirely in the browser. Nothing leaves your device."</p>
                        </div>

                        // Drop zone
                        <div
                            class="group relative border border-dashed border-zinc-300 rounded-xl p-5 text-center bg-zinc-50/50 hover:border-brand-400 hover:bg-brand-50/30 transition-all duration-200 cursor-pointer"
                            on:dragover=on_drag_over
                            on:drop=on_drop
                        >
                            <input
                                type="file"
                                accept="application/pdf"
                                class="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10"
                                id="file-input"
                                on:change=on_file_change
                            />
                            <div class="flex flex-col items-center gap-2.5 pointer-events-none">
                                <div class="w-10 h-10 rounded-xl bg-white border border-zinc-200 text-zinc-400 flex items-center justify-center group-hover:text-brand-500 group-hover:border-brand-200 group-hover:bg-brand-50 transition-colors">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5"/>
                                    </svg>
                                </div>
                                <div>
                                    <p class="text-sm font-medium text-zinc-700 group-hover:text-brand-600 transition-colors">"Drop PDF here or click to browse"</p>
                                    <p class="text-2xs text-zinc-400 mt-0.5">"Accepts .pdf files only"</p>
                                </div>
                            </div>
                        </div>

                        // Selected file
                        {move || (!file_name.get().is_empty()).then(|| view! {
                            <div class="flex items-center gap-2.5 px-3 py-2.5 rounded-lg bg-emerald-50 border border-emerald-200/60 animate-scale-in">
                                <svg class="w-4 h-4 text-emerald-500 shrink-0" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                                <p class="text-[13px] text-emerald-800 font-medium truncate">{file_name.get()}</p>
                            </div>
                        })}

                        // Analyze button
                        <button
                            class="w-full flex items-center justify-center gap-2 bg-zinc-900 hover:bg-zinc-800 text-white text-sm font-medium py-2.5 px-4 rounded-xl transition-all duration-200 disabled:opacity-40 disabled:cursor-not-allowed active:scale-[0.98] shadow-soft hover:shadow-medium"
                            disabled={move || file_name.get().is_empty() || is_loading.get()}
                            on:click=on_start_analysis
                        >
                            {move || if is_loading.get() {
                                view! {
                                    <svg class="animate-spin h-4 w-4 text-white/70" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/></svg>
                                    <span>"Analyzing..."</span>
                                }.into_view()
                            } else {
                                view! {
                                    <span>"Analyze Transcript"</span>
                                    <svg class="w-4 h-4 opacity-50" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M13.5 4.5L21 12m0 0l-7.5 7.5M21 12H3"/></svg>
                                }.into_view()
                            }}
                        </button>
                    </div>

                    // How it works card (only when no file selected)
                    {move || preview_url.get().is_none().then(|| view! {
                        <div class="bg-white rounded-2xl border border-zinc-200/80 shadow-soft p-5 animate-fade-in-up">
                            <h3 class="text-xs font-semibold text-zinc-500 uppercase tracking-widest mb-3">"How it works"</h3>
                            <div class="space-y-3">
                                <div class="flex items-start gap-3">
                                    <div class="mt-0.5 w-5 h-5 shrink-0 rounded-full bg-zinc-900 text-white flex items-center justify-center text-2xs font-bold">"1"</div>
                                    <p class="text-[13px] text-zinc-600 leading-relaxed">"Download your unofficial transcript PDF from SIS."</p>
                                </div>
                                <div class="flex items-start gap-3">
                                    <div class="mt-0.5 w-5 h-5 shrink-0 rounded-full bg-zinc-900 text-white flex items-center justify-center text-2xs font-bold">"2"</div>
                                    <p class="text-[13px] text-zinc-600 leading-relaxed">"Upload the PDF using the drop zone above."</p>
                                </div>
                                <div class="flex items-start gap-3">
                                    <div class="mt-0.5 w-5 h-5 shrink-0 rounded-full bg-zinc-900 text-white flex items-center justify-center text-2xs font-bold">"3"</div>
                                    <p class="text-[13px] text-zinc-600 leading-relaxed">"Click Analyze — everything is processed locally for maximum privacy."</p>
                                </div>
                            </div>
                        </div>
                    })}

                    // Privacy badge
                    <div class="hidden lg:flex items-center gap-2 px-4 py-3 rounded-xl bg-zinc-50 border border-zinc-100">
                        <svg class="w-4 h-4 text-zinc-400 shrink-0" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z"/>
                        </svg>
                        <p class="text-2xs text-zinc-500 font-medium">"Your transcript never leaves this device. All processing runs in WebAssembly."</p>
                    </div>
                </aside>

                // ── Right Panel — Results ────────────────────────────────
                <main class="flex-1 min-w-0">
                    {move || {
                        if is_loading.get() {
                            // Loading state
                            view! {
                                <div class="bg-white rounded-2xl border border-zinc-200/80 shadow-soft h-full min-h-[500px] flex flex-col items-center justify-center gap-4">
                                    <div class="relative">
                                        <div class="w-12 h-12 rounded-full border-2 border-zinc-200"></div>
                                        <div class="absolute inset-0 w-12 h-12 rounded-full border-2 border-brand-500 border-t-transparent animate-spin"></div>
                                    </div>
                                    <div class="text-center">
                                        <p class="text-sm font-medium text-zinc-700">"Analyzing transcript..."</p>
                                        <p class="text-xs text-zinc-400 mt-1">"Parsing courses and validating requirements"</p>
                                    </div>
                                </div>
                            }.into_view()
                        } else if let Some(err) = error_msg.get() {
                            // Error state
                            view! {
                                <div class="bg-white rounded-2xl border border-zinc-200/80 shadow-soft h-full min-h-[500px] flex flex-col items-center justify-center gap-4 px-8 text-center">
                                    <div class="w-12 h-12 rounded-full bg-red-50 flex items-center justify-center">
                                        <svg class="w-6 h-6 text-red-500" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="text-sm font-semibold text-zinc-800">"Analysis Failed"</p>
                                        <p class="text-[13px] text-zinc-500 mt-1 max-w-sm leading-relaxed">{err}</p>
                                    </div>
                                </div>
                            }.into_view()
                        } else if let Some(result) = audit_result.get() {
                            // Results view
                            view! {
                                <div class="space-y-5 animate-fade-in">

                                    // ── Hero: Total Credits ─────────────────
                                    <div class="bg-white rounded-2xl border border-zinc-200/80 shadow-soft p-6 sm:p-8 relative overflow-hidden">
                                        <div class="absolute -right-16 -top-16 w-48 h-48 bg-brand-100/40 rounded-full blur-3xl pointer-events-none"></div>
                                        <div class="relative flex flex-col sm:flex-row sm:items-end sm:justify-between gap-4">
                                            <div>
                                                <p class="text-xs font-semibold text-brand-600 uppercase tracking-widest mb-1">"Total Progress"</p>
                                                <div class="flex items-baseline gap-2">
                                                    <span class="text-5xl sm:text-6xl font-extrabold tracking-tighter text-zinc-900 tabular-nums">
                                                        {result.total_credits as u32}
                                                    </span>
                                                    <span class="text-base font-medium text-zinc-400">"credits earned"</span>
                                                </div>
                                            </div>
                                            <div class="flex items-center gap-1.5 text-xs text-zinc-500 bg-zinc-50 rounded-lg px-3 py-1.5 self-start sm:self-auto">
                                                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M4.26 10.147a60.436 60.436 0 00-.491 6.347A48.627 48.627 0 0112 20.904a48.627 48.627 0 018.232-4.41 60.46 60.46 0 00-.491-6.347"/></svg>
                                                <span class="font-medium">"B.Eng. Computer Science"</span>
                                            </div>
                                        </div>
                                    </div>

                                    // ── Category Progress Cards ─────────────
                                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 stagger-in">
                                        {result.categories.iter().map(|cat| {
                                            let pct = ((cat.collected_credits / cat.required_credits) * 100.0).min(100.0);
                                            let complete = pct >= 100.0;
                                            let cat_name = cat.name.clone();
                                            let collected = cat.collected_credits;
                                            let required = cat.required_credits;
                                            let pct_display = pct as u32;

                                            // SVG donut params
                                            let circumference = 100.0;
                                            let dash = (pct / 100.0) * circumference;

                                            let color_class = if complete { "text-emerald-500" } else { "text-brand-500" };
                                            let bg_class = if complete { "bg-emerald-50 border-emerald-100" } else { "bg-white border-zinc-200/80" };
                                            let badge_class = if complete { "bg-emerald-100 text-emerald-700" } else { "bg-brand-50 text-brand-600" };
                                            let badge_text = if complete { "Complete" } else { "In Progress" };

                                            view! {
                                                <div class={format!("rounded-2xl border shadow-soft p-5 flex flex-col gap-4 transition-shadow hover:shadow-medium {}", bg_class)}>
                                                    <div class="flex items-start justify-between">
                                                        <p class="text-sm font-semibold text-zinc-800">{cat_name}</p>
                                                        <span class={format!("text-2xs font-semibold px-2 py-0.5 rounded-full {}", badge_class)}>{badge_text}</span>
                                                    </div>
                                                    <div class="flex items-center gap-4">
                                                        // Donut chart
                                                        <div class="relative w-14 h-14 shrink-0">
                                                            <svg class="w-14 h-14 -rotate-90" viewBox="0 0 36 36">
                                                                <circle cx="18" cy="18" r="15.9155" fill="none" stroke="#e4e4e7" stroke-width="3"/>
                                                                <circle cx="18" cy="18" r="15.9155" fill="none"
                                                                    class={format!("{} donut-animated", color_class)}
                                                                    stroke="currentColor" stroke-width="3" stroke-linecap="round"
                                                                    stroke-dasharray={format!("{} {}", dash, circumference - dash)}/>
                                                            </svg>
                                                            <div class="absolute inset-0 flex items-center justify-center">
                                                                <span class="text-xs font-bold text-zinc-700">{format!("{}%", pct_display)}</span>
                                                            </div>
                                                        </div>
                                                        // Credits
                                                        <div>
                                                            <div class="flex items-baseline gap-1">
                                                                <span class="text-2xl font-bold text-zinc-900 tabular-nums">{collected as u32}</span>
                                                                <span class="text-sm text-zinc-400 font-medium">{format!("/ {}", required as u32)}</span>
                                                            </div>
                                                            <p class="text-2xs text-zinc-400 mt-0.5 font-medium">"credits"</p>
                                                        </div>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>

                                    // ── Course Details Accordion ─────────────
                                    <div class="bg-white rounded-2xl border border-zinc-200/80 shadow-soft overflow-hidden">
                                        <div class="px-5 py-4 border-b border-zinc-100 flex items-center gap-2.5">
                                            <svg class="w-4 h-4 text-zinc-400" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M3.75 12h16.5m-16.5 3.75h16.5M3.75 19.5h16.5M5.625 4.5h12.75a1.875 1.875 0 010 3.75H5.625a1.875 1.875 0 010-3.75z"/></svg>
                                            <h3 class="text-sm font-semibold text-zinc-800">"Course Details"</h3>
                                        </div>
                                        <div class="divide-y divide-zinc-100">
                                            {result.categories.iter().map(|category| {
                                                let category = category.clone();
                                                view! { <CategoryCard category={category} /> }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>

                                    // ── Missing Requirements ─────────────────
                                    {(!result.missing_subjects.is_empty()).then(|| {
                                        let mut seen_cats: Vec<String> = Vec::new();
                                        for m in &result.missing_subjects {
                                            if !seen_cats.contains(&m.category) {
                                                seen_cats.push(m.category.clone());
                                            }
                                        }
                                        view! {
                                            <div class="bg-white rounded-2xl border border-red-200/60 shadow-soft overflow-hidden">
                                                <div class="px-5 py-4 border-b border-red-100 flex items-center gap-2.5 bg-red-50/50">
                                                    <svg class="w-4 h-4 text-red-500" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"/></svg>
                                                    <h3 class="text-sm font-semibold text-red-800">"Missing Requirements"</h3>
                                                    <span class="ml-auto text-2xs font-semibold text-red-600 bg-red-100 px-2 py-0.5 rounded-full">{format!("{} items", result.missing_subjects.len())}</span>
                                                </div>
                                                <div class="divide-y divide-red-100/60">
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
                                                                    description.split(':').next().unwrap_or(description).trim().to_string()
                                                                };
                                                                if !ge_groups.contains(&group) {
                                                                    ge_groups.push(group);
                                                                }
                                                            }
                                                            ge_groups
                                                        } else {
                                                            cat_courses.iter().map(|m| m.description.clone()).collect()
                                                        };
                                                        let cat_label = cat.clone();
                                                        view! {
                                                            <div class="p-5">
                                                                <p class="text-xs font-semibold text-zinc-700 uppercase tracking-wider mb-2.5">{cat_label}</p>
                                                                <div class="space-y-1.5">
                                                                    {display_items.iter().map(|item| {
                                                                        let desc = item.clone();
                                                                        view! {
                                                                            <div class="flex items-start gap-2.5 py-1.5">
                                                                                <div class="w-1.5 h-1.5 rounded-full bg-red-400 mt-1.5 shrink-0"></div>
                                                                                <p class="text-[13px] text-zinc-600 leading-relaxed">{desc}</p>
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
                            }.into_view()
                        } else {
                            // Empty state
                            view! {
                                <div class="bg-white rounded-2xl border border-zinc-200/80 shadow-soft h-full min-h-[500px] flex flex-col items-center justify-center gap-5 px-8 text-center">
                                    <div class="w-16 h-16 rounded-2xl bg-zinc-50 border border-zinc-200 flex items-center justify-center">
                                        <svg class="w-7 h-7 text-zinc-300" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <h3 class="text-base font-semibold text-zinc-800 mb-1">"No transcript uploaded"</h3>
                                        <p class="text-sm text-zinc-400 max-w-xs leading-relaxed">"Upload your PDF transcript to see a breakdown of your progress toward graduation."</p>
                                    </div>
                                </div>
                            }.into_view()
                        }
                    }}
                </main>
            </div>
        </div>
    }
}
