#![allow(unused_parens)]
use crate::data::COURSES;
use crate::state::AppState;
use leptos::html::Input;
use leptos::prelude::*;

#[component]
pub fn CommandPalette(app: AppState) -> impl IntoView {
    let show = app.show_command_palette;
    let search_query = RwSignal::new(String::new());
    let selected_idx = RwSignal::new(0usize);
    let input_ref = NodeRef::<Input>::new();

    // Focus input when palette opens
    Effect::new(move |_| {
        if show.get() {
            search_query.set(String::new());
            selected_idx.set(0);
            if let Some(input) = input_ref.get() {
                let _ = input.focus();
            }
        }
    });

    let results = Signal::derive({
        move || {
            let query = search_query.get().to_lowercase();
            COURSES
                .iter()
                .enumerate()
                .filter(|(_, c)| {
                    if query.is_empty() {
                        return true;
                    }
                    c.title.to_lowercase().contains(&query)
                        || c.subtitle.to_lowercase().contains(&query)
                        || c.id.to_lowercase().contains(&query)
                        || c.difficulty.label().to_lowercase().contains(&query)
                })
                .collect::<Vec<_>>()
        }
    });

    view! {
        <Show when=move || show.get()>
            // Backdrop
            <div
                class="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-start justify-center pt-[15vh]"
                on:click=move |_| show.set(false)
                on:keydown=move |ev| {
                    if ev.key() == "Escape" {
                        show.set(false);
                    }
                }
            >
                <div
                    class="w-full max-w-xl bg-slate-900 border border-white/10 rounded-2xl shadow-2xl overflow-hidden"
                    on:click=|ev| ev.stop_propagation()
                >
                    // Search input
                    <div class="flex items-center gap-3 px-4 py-3 border-b border-white/10">
                        <span class="text-slate-500 text-lg">{"\u{1F50D}"}</span>
                        <input
                            node_ref=input_ref
                            type="text"
                            placeholder="Search courses..."
                            class="flex-1 bg-transparent text-sm text-white placeholder-slate-600 focus:outline-none"
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                search_query.set(value);
                                selected_idx.set(0);
                            }
                            on:keydown=move |ev| {
                                let key = ev.key();
                                let r = results.get();
                                let len = r.len();
                                if len == 0 { return; }

                                match key.as_str() {
                                    "ArrowDown" => {
                                        ev.prevent_default();
                                        selected_idx.update(|i| {
                                            *i = (*i + 1).min(len.saturating_sub(1));
                                        });
                                    }
                                    "ArrowUp" => {
                                        ev.prevent_default();
                                        selected_idx.update(|i| {
                                            *i = i.saturating_sub(1);
                                        });
                                    }
                                    "Enter" => {
                                        ev.prevent_default();
                                        let idx = selected_idx.get();
                                        if let Some((_, course)) = r.get(idx) {
                                            app.go_to_course_detail(course.id.to_string());
                                            show.set(false);
                                        }
                                    }
                                    "Escape" => {
                                        show.set(false);
                                    }
                                    _ => {}
                                }
                            }
                        />
                        <kbd class="text-[10px] font-mono text-slate-600 bg-white/5 border border-white/10 rounded px-1.5 py-0.5 hidden sm:inline">ESC</kbd>
                    </div>

                    // Results list
                    <div class="max-h-72 overflow-y-auto p-2">
                        {move || {
                            let r = results.get();
                            if r.is_empty() {
                                return view! {
                                    <div class="py-8 text-center text-slate-500 text-sm">
                                        No courses found
                                    </div>
                                }
                                .into_any();
                            }
                            r.into_iter()
                                .enumerate()
                                .map(|(idx, (_orig_idx, course))| {
                                    let app_clone = app;
                                    let is_selected = Signal::derive(move || selected_idx.get() == idx);
                                    let started = Signal::derive(move || app_clone.has_started(course.id));
                                    let course_id = course.id.to_string();
                                    view! {
                                        <div
                                            class=format!(
                                                "flex items-center gap-3 px-3 py-2.5 rounded-xl cursor-pointer transition-colors {}",
                                                if is_selected.get() { "bg-white/10" } else { "hover:bg-white/5" }
                                            )
                                            on:click=move |_| {
                                                app_clone.go_to_course_detail(course_id.clone());
                                                show.set(false);
                                            }
                                            on:mouseenter=move |_| {
                                                selected_idx.set(idx);
                                            }
                                        >
                                            <span class="text-lg">{course.icon}</span>
                                            <div class="flex-1 min-w-0">
                                                <div class="text-sm font-bold text-white truncate">{course.title}</div>
                                                <div class="text-[10px] text-slate-500 truncate">{course.subtitle}</div>
                                            </div>
                                            <div class="flex items-center gap-2 shrink-0">
                                                <span class="text-[10px] font-mono text-slate-600">{course.difficulty.label()}</span>
                                                {move || if started.get() {
                                                    view! { <span class="text-[10px] font-mono text-green-400">In Progress</span> }.into_any()
                                                } else {
                                                    view! { <span class="text-[10px] font-mono text-slate-600">New</span> }.into_any()
                                                }}
                                            </div>
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()
                                .into_view()
                                .into_any()
                        }}
                    </div>

                    // Footer hints
                    <div class="px-4 py-2 border-t border-white/5 flex items-center gap-4 text-[10px] text-slate-600 font-mono">
                        <span class="flex items-center gap-1"><kbd class="bg-white/5 border border-white/10 rounded px-1">{"\u{2191}"}</kbd><kbd class="bg-white/5 border border-white/10 rounded px-1">{"\u{2193}"}</kbd> navigate</span>
                        <span class="flex items-center gap-1"><kbd class="bg-white/5 border border-white/10 rounded px-1">Enter</kbd> open</span>
                        <span class="flex items-center gap-1"><kbd class="bg-white/5 border border-white/10 rounded px-1">Esc</kbd> close</span>
                    </div>
                </div>
            </div>
        </Show>
    }
}
