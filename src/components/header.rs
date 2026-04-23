#![allow(unused_parens)]
use crate::state::{AppState, AppView};
use leptos::either::Either;
use leptos::prelude::*;

#[component]
pub fn Header(app: AppState) -> impl IntoView {
    let current_view = app.current_view;
    let current_step = app.current_step;
    let completed = app.completed;

    // Course context signals
    let lesson_course_id = Signal::derive(move || match current_view.get() {
        AppView::Lesson { ref course_id } => Some(course_id.clone()),
        AppView::CourseDetail { ref course_id } => Some(course_id.clone()),
        _ => None,
    });

    let is_lesson = Signal::derive(move || matches!(current_view.get(), AppView::Lesson { .. }));
    let is_catalog = Signal::derive(move || matches!(current_view.get(), AppView::Catalog));
    let is_detail = Signal::derive(move || matches!(current_view.get(), AppView::CourseDetail { .. }));

    let mac = Signal::derive(move || {
        web_sys::window()
            .and_then(|w| w.navigator().user_agent().ok())
            .map(|ua| ua.contains("Macintosh") || ua.contains("Mac OS X"))
            .unwrap_or(false)
    });

    // Lesson-specific signals
    let step_icon = Signal::derive(move || {
        if let Some(cid) = lesson_course_id.get() {
            if let Some(course) = app.get_course(&cid) {
                let step = current_step.get();
                if step < course.lesson_count() {
                    return match course.get_module(step).unwrap().module_type {
                        crate::data::ModuleType::Concept => "\u{1F4A1}",
                        crate::data::ModuleType::Practice => "\u{26A1}",
                    };
                }
            }
        }
        "\u{26A1}"
    });

    let run_label = Signal::derive(move || {
        if let Some(cid) = lesson_course_id.get() {
            if let Some(course) = app.get_course(&cid) {
                let step = current_step.get();
                if step < course.lesson_count() {
                    return match course.get_module(step).unwrap().module_type {
                        crate::data::ModuleType::Concept => "ACKNOWLEDGE",
                        crate::data::ModuleType::Practice => "CHECK_ANSWER",
                    };
                }
            }
        }
        "CHECK_ANSWER"
    });

    let run_btn_class = Signal::derive(move || {
        if let Some(cid) = lesson_course_id.get() {
            if let Some(course) = app.get_course(&cid) {
                let step = current_step.get();
                if step < course.lesson_count() {
                    return match course.get_module(step).unwrap().module_type {
                        crate::data::ModuleType::Concept => "flex items-center gap-2 px-4 sm:px-6 py-2 rounded-lg font-black text-[11px] tracking-widest transition-all active:scale-95 bg-blue-600 text-white",
                        crate::data::ModuleType::Practice => "flex items-center gap-2 px-4 sm:px-6 py-2 rounded-lg font-black text-[11px] tracking-widest transition-all active:scale-95 bg-orange-600 text-white shadow-[0_0_20px_rgba(234,88,12,0.2)]",
                    };
                }
            }
        }
        "flex items-center gap-2 px-4 sm:px-6 py-2 rounded-lg font-black text-[11px] tracking-widest transition-all active:scale-95 bg-orange-600 text-white"
    });

    let module_count = Signal::derive(move || {
        if let Some(cid) = lesson_course_id.get() {
            if let Some(course) = app.get_course(&cid) {
                return course.lesson_count();
            }
        }
        0
    });

    let lesson_course = Signal::derive(move || {
        lesson_course_id.get().and_then(|cid| app.get_course(&cid))
    });

    // Overall progress for catalog
    let progress_pct = Signal::derive({
        let app = app;
        move || app.get_platform_progress_pct()
    });
    let mastery = Signal::derive({
        let app = app;
        move || app.get_mastery_level()
    });
    let has_progress = Signal::derive(move || progress_pct.get() > 0);

    view! {
        <header class="flex items-center justify-between px-4 sm:px-6 py-3 bg-slate-900/50 border-b border-white/5 backdrop-blur-xl shrink-0">

            // LEFT: Brand + Context
            <div class="flex items-center gap-3 min-w-0">

                // Goldcoders brand (always visible, clickable -> catalog)
                <button
                    on:click=move |_| app.go_to_catalog()
                    class="flex items-center gap-2.5 shrink-0 hover:opacity-80 transition-opacity"
                >
                    <img
                        src="https://goldcoders.dev/goldcoders_logo.png"
                        alt="Goldcoders"
                        class="w-8 h-8 rounded-lg"
                    />
                    <div class="hidden sm:block">
                        <div class="text-sm font-black tracking-wide text-[#C9A96E]">Goldcoders</div>
                        <div class="text-[9px] font-bold tracking-[0.2em] text-slate-500 uppercase -mt-0.5">Tutorials</div>
                    </div>
                </button>

                // Breadcrumb separator (detail + lesson only)
                <Show when=move || is_detail.get() || is_lesson.get()>
                    <span class="text-slate-700 text-xs font-mono">/</span>
                    <span class="text-xs font-bold text-slate-300 truncate max-w-[120px] sm:max-w-xs">
                        {move || lesson_course.get().map(|c| c.title).unwrap_or_default()}
                    </span>
                </Show>

                // Catalog: show overall progress pill
                <Show when=move || is_catalog.get() && has_progress.get()>
                    <div class="hidden md:flex items-center gap-2 ml-2 bg-white/5 border border-white/10 rounded-full px-3 py-1">
                        <span class="text-xs">{move || mastery.get().1}</span>
                        <span class="text-[10px] font-bold text-slate-400">{move || format!("{}%", progress_pct.get())}</span>
                    </div>
                </Show>
            </div>

            // RIGHT: Actions

            // Catalog: nothing extra
            <Show when=move || is_catalog.get() fallback=|| ()>
                <div class="flex items-center gap-2">
                    <span class="text-[10px] font-mono text-slate-600 hidden sm:inline">
                        <kbd class="bg-white/5 border border-white/10 rounded px-1 mr-1">{move || if mac.get() { "⌘K" } else { "Ctrl+K" }}</kbd> search
                    </span>
                </div>
            </Show>

            // Course detail: start/resume button
            <Show when=move || is_detail.get() fallback=|| ()>
                <div class="flex items-center gap-3 shrink-0">
                    <button
                        on:click=move |_| app.go_to_catalog()
                        class="flex items-center gap-1.5 text-slate-500 hover:text-white text-xs font-mono uppercase tracking-wider transition-colors"
                    >
                        {"\u{2190}"} <span class="hidden sm:inline">Back</span>
                    </button>
                    {move || {
                        if let Some(course) = lesson_course.get() {
                            let cid = course.id.to_string();
                            let started = app.has_started(&cid);
                            let accent_btn = format!("bg-{}-500 hover:bg-{}-600 text-white", course.accent, course.accent);
                            let label = if started { "RESUME" } else { "START" };
                            Either::Left(view! {
                                <button
                                    on:click=move |_| {
                                        if started {
                                            app.resume_course(&cid);
                                        } else {
                                            app.start_course(&cid);
                                        }
                                    }
                                    class=format!("px-5 py-2 rounded-lg font-black text-[11px] tracking-widest transition-all active:scale-95 {}", accent_btn)
                                >
                                    {label}
                                </button>
                            })
                        } else {
                            Either::Right(view! { <div/> })
                        }
                    }}
                </div>
            </Show>

            // Lesson: step indicators + run button
            <Show when=move || is_lesson.get() fallback=|| ()>
                <div class="flex items-center gap-3 shrink-0">
                    // Step dots (desktop only)
                    <div class="hidden lg:flex gap-0.5 max-w-[200px] overflow-x-auto">
                        {(0..module_count.get()).map(|idx| {
                            let app_inner = app;
                            let is_current = Signal::derive(move || current_step.get() == idx);
                            let is_completed = Signal::derive(move || {
                                let steps = completed.get();
                                steps.get(idx).copied().unwrap_or(false)
                            });
                            view! {
                                <button
                                    on:click=move |_| app_inner.navigate_to(idx)
                                    class="h-1.5 rounded-full transition-all duration-300 shrink-0"
                                    class=("w-6 bg-orange-500", move || is_current.get())
                                    class=("w-2 bg-green-500/60", move || !is_current.get() && is_completed.get())
                                    class=("w-2 bg-slate-800 hover:bg-slate-700", move || !is_current.get() && !is_completed.get())
                                />
                            }
                        }).collect::<Vec<_>>()}
                    </div>

                    // Back to catalog
                    <button
                        on:click=move |_| app.go_to_catalog()
                        class="flex items-center gap-1.5 bg-white/5 p-2 rounded-lg border border-white/10 hover:bg-white/10 transition-colors"
                        title=move || if mac.get() { "Back to Catalog (⌘⇧[)" } else { "Back to Catalog (Ctrl+Shift+[)" }
                    >
                        <span class="text-slate-400 text-sm">{"\u{2190}"}</span>
                    </button>

                    // Run / Acknowledge button
                    <button
                        on:click=move |_| {
                            if app.handle_run() {
                                app.next_step();
                            }
                        }
                        class=run_btn_class
                    >
                        <span>{step_icon}</span>
                        <span class="hidden sm:inline">{run_label}</span>
                        <span class="sm:hidden">{move || if run_label.get() == "ACKNOWLEDGE" { "\u{1F4A1}" } else { "\u{2713}" }}</span>
                    </button>
                </div>
            </Show>
        </header>
    }
}
