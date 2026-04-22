use crate::state::AppState;
use leptos::prelude::*;
use leptos::either::Either;

#[component]
pub fn CourseDetail(app: AppState, course_id: String) -> impl IntoView {
    let course = app.get_course(&course_id).cloned();
    let total = course.as_ref().map(|c| c.modules.len()).unwrap_or(0);
    let pct = app.get_course_progress_pct(&course_id);
    let started = app.has_started(&course_id);

    // Effect to scroll selected lesson into view
    let sl = app.selected_lesson;
    Effect::new(move |_| {
        if let Some(idx) = sl.get() {
            let id = format!("lesson-item-{}", idx);
            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    if let Some(el) = doc.get_element_by_id(&id) {
                        let options = web_sys::ScrollIntoViewOptions::new();
                        options.set_behavior(web_sys::ScrollBehavior::Smooth);
                        options.set_block(web_sys::ScrollLogicalPosition::Nearest);
                        el.scroll_into_view_with_scroll_into_view_options(&options);
                    }
                }
            }
        }
    });

    // Render course content or not-found message
    let course_content = move || {
        if let Some(ref course) = course {
            let accent = course.accent;
            let accent_text = format!("text-{}-500", accent);
            let accent_btn = format!("bg-{}-500 hover:bg-{}-600 text-white", accent, accent);
            let accent_bar = format!("bg-{}-500", accent);

            let cid_clone = course_id.clone();

            let lesson_items: Vec<_> = course
                .modules
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let is_concept = matches!(m.module_type, crate::data::ModuleType::Concept);
                    let progress = app.course_progress.get();
                    let is_completed = progress
                        .iter()
                        .find(|(id, _)| id == &course_id)
                        .map(|(_, p)| p.completed.get(i).copied().unwrap_or(false))
                        .unwrap_or(false);
                    let is_skipped = progress
                        .iter()
                        .find(|(id, _)| id == &course_id)
                        .map(|(_, p)| p.skipped.get(i).copied().unwrap_or(false))
                        .unwrap_or(false);

                    let is_selected = Signal::derive(move || app.selected_lesson.get() == Some(i));

                    (i, m, is_concept, is_completed, is_skipped, is_selected)
                })
                .collect();

            Either::Left(view! {
                <div>
                    <button
                        on:click=move |_| app.go_to_catalog()
                        class="flex items-center gap-2 text-slate-600 hover:text-white text-xs font-mono uppercase tracking-widest mb-12 transition-colors"
                    >
                        {"\u{2190}"} Back to Catalog
                    </button>

                    <div class="mb-12">
                        <div class="flex items-center gap-4 mb-4">
                            <span class="text-4xl">{course.icon}</span>
                            <div>
                                <h1 class="text-2xl font-black text-white">{course.title}</h1>
                                <p class="text-slate-500 text-sm">{course.subtitle}</p>
                            </div>
                        </div>

                        <div class="flex items-center gap-6 mt-6">
                            <div class="flex items-center gap-2 text-xs text-slate-600 font-mono">
                                <span>{total} lessons</span>
                                <span class="text-slate-800">|</span>
                                <span>{course.estimated_time}</span>
                            </div>
                            <div class="flex-1 max-w-xs">
                                <div class="h-1.5 bg-slate-800 rounded-full overflow-hidden">
                                    <div
                                        class=format!("h-full rounded-full transition-all duration-500 {}", accent_bar)
                                        style=format!("width: {}%", pct)
                                    />
                                </div>
                            </div>
                            <span class=accent_text>{pct}%</span>
                        </div>
                    </div>

                    <div class="flex items-center justify-between mb-6">
                        <h2 class="text-sm font-black text-slate-400 uppercase tracking-widest">Lessons</h2>
                        <button
                            on:click=move |_| {
                                app.resume_prioritized_course(&cid_clone);
                            }
                            class=format!("px-6 py-2 rounded-lg font-black text-[11px] tracking-widest transition-all {}", accent_btn)
                        >
                            {if started { "RESUME" } else { "START" }}
                        </button>
                    </div>

                    <div class="space-y-1">
                        {lesson_items.into_iter().map(|(i, m, is_concept, is_completed, is_skipped, is_selected)| {
                            let idx = i;
                            let cid = course_id.clone();
                            let item_id = format!("lesson-item-{}", idx);
                            let icon = if is_completed {
                                "\u{2705}"
                            } else if is_skipped {
                                "\u{23ED}"
                            } else if is_concept {
                                "\u{1F4A1}"
                            } else {
                                "\u{26A1}"
                            };

                            let active_highlight = move || {
                                if is_selected.get() && app.highlight_active.get() {
                                    "bg-white/10 ring-1 ring-white/10"
                                } else if is_selected.get() {
                                    "bg-white/5"
                                } else {
                                    "bg-transparent"
                                }
                            };

                            view! {
                                <div
                                    id=item_id
                                    class=move || format!("flex items-center gap-4 px-4 py-3 rounded-xl hover:bg-white/5 transition-all duration-700 group {}", active_highlight())
                                    on:click=move |_| {
                                        app.selected_lesson.set(Some(idx));
                                        app.trigger_highlight();
                                    }
                                >
                                    <span class="text-sm font-mono text-slate-700 w-6 text-right">{idx + 1}</span>
                                    <span class="text-lg">{icon}</span>
                                    <div class="flex-1 min-w-0">
                                        <p class="text-sm text-slate-300 truncate">{m.title}</p>
                                    </div>
                                    <span class=format!("text-[10px] font-mono uppercase tracking-widest {}", if is_concept { "text-blue-500/60" } else { "text-orange-500/60" })>
                                        {if is_concept { "CONCEPT" } else { "PRACTICE" }}
                                    </span>
                                    <button
                                        on:click=move |ev| {
                                            ev.stop_propagation();
                                            app.start_course(&cid);
                                            app.navigate_to(idx);
                                        }
                                        class="opacity-0 group-hover:opacity-100 text-slate-600 hover:text-white transition-all text-xs font-mono uppercase tracking-widest"
                                    >
                                        {"\u{2192}"}
                                    </button>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            })
        } else {
            Either::Right(view! {
                <div class="flex-1 flex items-center justify-center">
                    <p class="text-slate-500">Course not found</p>
                </div>
            })
        }
    };

    view! {
        <div class="flex-1 overflow-y-auto p-8 sm:p-16 custom-scrollbar">
            <div class="max-w-3xl mx-auto">
                {course_content}
            </div>
        </div>
    }
}
