//! Category Card Component
//!
//! Displays an expandable/collapsible category card with progress tracking.
//! Shows category name, credit progress, and all associated courses when expanded.
//! Uses Tailwind CSS for styling with smooth transitions and animations.

use crate::models::Category;
use leptos::*;

/// Collapsible accordion card for displaying a category and its courses
///
/// **Features:**
/// - Header shows category name and credits (collected/required)
/// - Progress bar with color coding (gray < 50% < amber < 75% < emerald)
/// - Expandable course list with course codes, names, grades, and credits
/// - Completion status indicator
#[component]
pub fn CategoryCard(category: Category) -> impl IntoView {
    let (is_expanded, set_is_expanded) = create_signal(false);
    let percentage = (category.collected_credits / category.required_credits * 100.0).min(100.0);
    let category_clone = category.clone();

    view! {
        <div class="border border-slate-100 rounded-xl overflow-hidden bg-white shadow-sm ring-1 ring-slate-900/5 transition-all duration-200 hover:shadow-md">
            <button
                class="w-full px-5 py-4 flex justify-between items-center hover:bg-slate-50 transition-colors duration-200 text-left group"
                on:click=move |_| set_is_expanded.update(|v| *v = !*v)
            >
                <div>
                    <span class="text-sm font-semibold text-slate-800 group-hover:text-blue-600 transition-colors">{&category.name}</span>
                </div>
                <div class="flex items-center gap-4">
                    <div class="flex flex-col items-end">
                        <span class="text-[11px] font-bold text-slate-400 uppercase tracking-wider mb-0.5">"Credits"</span>
                        <span class="text-xs font-semibold text-slate-700">
                            {format!("{:.0} / {:.0}", category.collected_credits, category.required_credits)}
                        </span>
                    </div>
                    <div class={format!("w-7 h-7 rounded-full flex items-center justify-center transition-colors {}", if is_expanded.get() { "bg-blue-50 text-blue-600" } else { "bg-slate-50 text-slate-400 group-hover:bg-blue-50 group-hover:text-blue-500" })}>
                        <svg
                            class={format!(
                                "w-4 h-4 transition-transform duration-200 {}",
                                if is_expanded.get() { "rotate-180" } else { "" }
                            )}
                            fill="none" stroke="currentColor" viewBox="0 0 24 24"
                        >
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                        </svg>
                    </div>
                </div>
            </button>

            <div class="px-5 pb-4">
                <div class="w-full bg-slate-100 rounded-full h-1.5 overflow-hidden">
                    <div
                        class={format!("h-full rounded-full transition-all duration-1000 ease-out {}",
                            if percentage >= 100.0 { "bg-green-500" } else { "bg-blue-500" }
                        )}
                        style={format!("width: {}%", percentage)}
                    ></div>
                </div>
            </div>

            {move || {
                if is_expanded.get() {
                    view! {
                        <div class="border-t border-slate-100 max-h-[400px] overflow-y-auto bg-slate-50/30">
                            {if category_clone.courses.is_empty() {
                                view! {
                                    <div class="flex flex-col items-center justify-center py-8">
                                        <div class="w-10 h-10 rounded-full bg-slate-50 flex items-center justify-center mb-2 border border-slate-100">
                                            <svg class="w-4 h-4 text-slate-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4M12 20V4"></path></svg>
                                        </div>
                                        <p class="text-xs font-medium text-slate-400">"No courses completed"</p>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <div class="divide-y divide-slate-100">
                                        {category_clone.courses.iter().map(|course| {
                                            let course = course.clone();
                                            view! {
                                                <div class="flex justify-between items-center px-5 py-3.5 hover:bg-slate-50 transition-colors group">
                                                    <div class="flex-1 min-w-0 pr-4">
                                                        <div class="flex items-center gap-2 mb-0.5">
                                                            <span class="font-mono text-[10px] font-bold bg-slate-100 text-slate-500 px-1.5 py-0.5 rounded-md tracking-wider">{&course.code}</span>
                                                        </div>
                                                        <span class="text-sm font-medium text-slate-700 truncate block group-hover:text-slate-900 transition-colors">{&course.name}</span>
                                                    </div>
                                                    <div class="flex items-center gap-4 shrink-0">
                                                        <div class="flex flex-col items-center justify-center w-8 h-8 rounded-lg bg-blue-50 text-blue-600 font-bold text-sm border border-blue-100/50">
                                                            {&course.grade}
                                                        </div>
                                                        <span class="text-[11px] font-semibold text-slate-400 uppercase tracking-wider w-8 text-right">{format!("{} cr", course.credit)}</span>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_view()
                            }}
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}
        </div>
    }
}
