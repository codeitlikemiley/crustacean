use crate::state::{AppState, AppView};
use leptos::prelude::*;

#[component]
pub fn Footer(app: AppState) -> impl IntoView {
    let current_step = app.current_step;
    let completed = app.completed;
    let skipped = app.skipped;
    let current_view = app.current_view;

    let module_count = Signal::derive(move || {
        if let AppView::Lesson { ref course_id } = current_view.get() {
            if let Some(course) = app.get_course(course_id) {
                return course.lesson_count();
            }
        }
        1
    });

    let completion_pct = Signal::derive(move || {
        let steps = completed.get();
        let sk = skipped.get();
        let counted = steps.iter().zip(sk.iter()).filter(|(c, s)| **c || **s).count();
        let total = module_count.get();
        if total == 0 {
            return 0;
        }
        ((counted as f64) / (total as f64) * 100.0).round() as usize
    });

    let is_last_step = Signal::derive(move || current_step.get() >= module_count.get() - 1);
    let next_label = Signal::derive(move || if is_last_step.get() { "FINISH" } else { "NEXT" });

    view! {
        <footer class="bg-slate-950 border-t border-white/10 px-8 py-4 flex items-center justify-between shrink-0">
            <button
                on:click=move |_| app.prev_step()
                class="flex items-center gap-3 text-slate-500 hover:text-white disabled:opacity-5 transition-all font-black text-[11px] tracking-widest"
                disabled=move || current_step.get() == 0
            >
                <span>{"\u{25C0}"}</span>
                <span>PREV</span>
            </button>

            <div class="text-[10px] font-mono text-slate-600 font-black tracking-widest uppercase">
                Completion: {completion_pct}%
            </div>

            <div class="flex items-center gap-4">
                <button
                    on:click=move |_| app.next_step()
                    class="flex items-center gap-2 px-6 py-2 rounded-lg font-black text-[11px] tracking-widest transition-all bg-slate-900 text-slate-500 hover:text-white"
                >
                    <span>SKIP</span>
                    <span>{"\u{25B6}\u{25B6}"}</span>
                </button>

                <button
                    on:click=move |_| app.next_step()
                    class="flex items-center gap-3 px-10 py-2.5 rounded-xl font-black text-[11px] tracking-widest transition-all"
                    class=("bg-white text-black hover:bg-orange-500 hover:text-white shadow-xl", move || {
                        let steps = completed.get();
                        steps.get(current_step.get()).copied().unwrap_or(false)
                    })
                    class=("bg-slate-800 text-slate-600", move || {
                        let steps = completed.get();
                        !steps.get(current_step.get()).copied().unwrap_or(false)
                    })
                    disabled=move || is_last_step.get()
                >
                    <span>{next_label}</span>
                    <span>{"\u{25B6}"}</span>
                </button>
            </div>
        </footer>
    }
}
