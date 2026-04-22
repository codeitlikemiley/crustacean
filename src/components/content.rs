use crate::state::{AppState, AppView};
use crate::utils;
use leptos::prelude::*;

#[component]
pub fn Content(app: AppState, is_concept: Signal<bool>) -> impl IntoView {
    let current_step = app.current_step;
    let is_success = app.is_success;
    let current_view = app.current_view;

    let course_id = Signal::derive(move || match current_view.get() {
        AppView::Lesson { ref course_id } => Some(course_id.clone()),
        _ => None,
    });

    let module_ref = Signal::derive(move || {
        if let Some(cid) = course_id.get() {
            if let Some(course) = app.get_course(&cid) {
                let step = current_step.get();
                if step < course.modules.len() {
                    return Some(&course.modules[step]);
                }
            }
        }
        None
    });

    let step_type_label = Signal::derive(move || {
        module_ref
            .get()
            .map(|m| match m.module_type {
                crate::data::ModuleType::Concept => "CONCEPT",
                crate::data::ModuleType::Practice => "PRACTICE",
            })
            .unwrap_or("PRACTICE")
    });

    let step_type_icon = Signal::derive(move || {
        module_ref
            .get()
            .map(|m| match m.module_type {
                crate::data::ModuleType::Concept => "\u{1F4A1}",
                crate::data::ModuleType::Practice => "\u{26A1}",
            })
            .unwrap_or("\u{26A1}")
    });

    let rendered_html = Signal::derive(move || {
        module_ref
            .get()
            .map(|m| utils::render_markdown(m.content))
            .unwrap_or_default()
    });

    let success_message = Signal::derive(move || {
        module_ref
            .get()
            .map(|m| m.success_message)
            .unwrap_or("")
    });

    let level_text = move || format!("Phase // Level {}", current_step.get() + 1);

    let section_class = move || {
        if is_concept.get() {
            // Full-width centered reading mode
            "flex-1 flex flex-col bg-neutral-950 overflow-y-auto px-6 sm:px-10 lg:px-16 py-10 custom-scrollbar h-full"
        } else {
            // Side panel mode (practice) — stacks on mobile, side-by-side on desktop
            "w-full lg:w-2/5 flex flex-col border-b lg:border-b-0 lg:border-r border-white/5 bg-neutral-950 overflow-y-auto px-6 sm:px-8 py-10 custom-scrollbar h-full max-h-[50vh] lg:max-h-none"
        }
    };

    let wrapper_class = move || {
        if is_concept.get() {
            "max-w-3xl mx-auto w-full"
        } else {
            ""
        }
    };

    view! {
        <section class=section_class>
            <div class=wrapper_class>
                <div class="flex items-center gap-2 text-orange-500 mb-8 font-mono text-xs font-black uppercase tracking-widest">
                    {step_type_icon}
                    {step_type_label}
                    {level_text}
                </div>

                <div class="markdown-body" inner_html=rendered_html />

                <Show
                    when=move || is_success.get()
                    fallback=|| ()
                >
                    <div class="mt-12 p-8 bg-green-500/5 border border-green-500/20 rounded-2xl">
                        <div class="flex items-center gap-3 text-green-400 font-black mb-3 text-xs tracking-widest">
                            {"\u{2705}"}
                            SUCCESS_LOADED
                        </div>
                        <p class="text-slate-400 text-sm leading-relaxed">{success_message}</p>
                    </div>
                </Show>
            </div>
        </section>
    }
}
