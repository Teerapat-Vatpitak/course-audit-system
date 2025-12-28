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
        <div class="bg-white border border-gray-200 rounded-xl shadow-sm overflow-hidden hover:shadow-md transition-shadow duration-200">
            <button
                class="w-full px-6 py-4 flex justify-between items-center hover:bg-gray-50 transition-colors duration-150"
                on:click=move |_| set_is_expanded.update(|v| *v = !*v)
            >
                <div class="flex-1 text-left">
                    <h4 class="font-semibold text-gray-900 text-lg">{&category.name}</h4>
                </div>
                <div class="flex items-center gap-4">
                    <span class="text-sm text-gray-600 font-medium whitespace-nowrap">
                        {format!("{:.1} / {:.1}", category.collected_credits, category.required_credits)}
                    </span>
                    <svg
                        class={format!(
                            "w-5 h-5 text-gray-400 transition-transform duration-200 {}",
                            if is_expanded.get() { "transform rotate-180" } else { "" }
                        )}
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                    >
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
                    </svg>
                </div>
            </button>

            <div class="px-6 py-3 bg-gray-50 border-t border-gray-100">
                <div class="w-full bg-gray-200 rounded-full h-2">
                    <div
                        class={format!("h-2 rounded-full transition-all {}",
                            if percentage >= 100.0 { "bg-emerald-500" }
                            else if percentage >= 75.0 { "bg-emerald-600" }
                            else if percentage >= 50.0 { "bg-amber-500" }
                            else { "bg-gray-400" }
                        )}
                        style={format!("width: {}%", percentage)}
                    ></div>
                </div>
                <div class="flex justify-between items-center mt-2">
                    <span class="text-xs text-gray-600 font-medium">
                        {format!("{}%", (percentage as i32))}
                    </span>
                    {if percentage >= 100.0 {
                        view! {
                            <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-semibold bg-emerald-100 text-emerald-800">
                                "✓ Complete"
                            </span>
                        }
                    } else {
                        view! {
                            <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-semibold bg-amber-100 text-amber-800">
                                "In Progress"
                            </span>
                        }
                    }}
                </div>
            </div>

            {move || {
                if is_expanded.get() {
                    view! {
                        <div class="px-6 py-4 border-t border-gray-100 space-y-3 max-h-[500px] overflow-y-auto">
                            {if category_clone.courses.is_empty() {
                                view! {
                                    <p class="text-sm text-gray-500 italic text-center py-4">
                                        "No courses in this category"
                                    </p>
                                }.into_view()
                            } else {
                                category_clone.courses.iter().map(|course| {
                                    let course = course.clone();
                                    view! {
                                        <div class="flex justify-between items-center gap-4 py-2 px-3 hover:bg-gray-50 rounded-lg transition-colors">
                                            <div class="flex-1 min-w-0">
                                                <div class="font-mono text-xs font-bold text-emerald-700 mb-1">
                                                    {&course.code}
                                                </div>
                                                <div class="text-sm text-gray-700 truncate">
                                                    {&course.name}
                                                </div>
                                            </div>
                                            <div class="flex items-center gap-3 ml-4">
                                                <span class="inline-flex items-center px-2.5 py-1 rounded-lg text-xs font-semibold bg-gray-100 text-gray-800 whitespace-nowrap">
                                                    {&course.grade}
                                                </span>
                                                <div class="text-right">
                                                    <div class="text-sm font-semibold text-gray-900">
                                                        {format!("{} cr", course.credit)}
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>().into_view()
                            }}
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div></div>
                    }.into_view()
                }
            }}
        </div>
    }
}
