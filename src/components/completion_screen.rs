use crate::state::{AppState, AppView};
use leptos::prelude::*;

#[component]
pub fn CompletionScreen(app: AppState) -> impl IntoView {
    let current_view = app.current_view;
    
    let course = Signal::derive(move || {
        if let AppView::Lesson { ref course_id } = current_view.get() {
            app.get_course(course_id)
        } else {
            None
        }
    });

    let parents = Signal::derive(move || {
        if let AppView::Lesson { ref course_id } = current_view.get() {
            app.find_parent_curriculums(course_id)
        } else {
            Vec::new()
        }
    });

    view! {
        <div class="flex-1 flex items-center justify-center bg-neutral-950 p-8 overflow-y-auto">
            <div class="max-w-2xl w-full bg-[#0B0C10] border border-white/5 rounded-3xl p-12 text-center shadow-2xl relative overflow-hidden">
                {move || {
                    if let Some(c) = course.get() {
                        if matches!(c.kind, crate::data::model::CourseKind::Curriculum) {
                            // Celebration for full path
                            view! {
                                <div class="absolute inset-0 pointer-events-none opacity-20">
                                    <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-96 h-96 bg-yellow-500 rounded-full blur-[100px]"></div>
                                </div>
                                <div class="relative z-10">
                                    <div class="text-6xl mb-6">"🏆"</div>
                                    <h2 class="text-3xl font-black text-white mb-4">"Journey Complete!"</h2>
                                    <p class="text-slate-400 mb-8 text-lg">"You have fully mastered the " <span class="text-yellow-400 font-bold">{c.title}</span> " guided path."</p>
                                    <button 
                                        on:click=move |_| app.go_to_catalog()
                                        class="px-8 py-3 bg-yellow-500 hover:bg-yellow-400 text-yellow-950 font-black tracking-widest text-sm rounded-xl transition-all"
                                    >
                                        "RETURN TO DASHBOARD"
                                    </button>
                                </div>
                            }.into_any()
                        } else {
                            // Focus module completion
                            view! {
                                <div class="relative z-10">
                                    <div class="text-6xl mb-6">"✨"</div>
                                    <h2 class="text-3xl font-black text-white mb-4">"Deep Dive Complete!"</h2>
                                    <p class="text-slate-400 mb-10 text-lg">"You successfully finished " <span class="text-white font-bold">{c.title}</span> "."</p>
                                    
                                    {
                                        let parent_list = parents.get();
                                        if !parent_list.is_empty() {
                                            view! {
                                                <div class="text-left bg-slate-900/50 p-6 rounded-2xl border border-slate-800">
                                                    <div class="text-xs font-black text-slate-500 uppercase tracking-widest mb-4">"Continue Your Journey"</div>
                                                    <p class="text-sm text-slate-400 mb-6">"This topic is part of a larger guided path. Keep going to master the complete ecosystem:"</p>
                                                    <div class="flex flex-col gap-3">
                                                        {parent_list.into_iter().map(|parent| {
                                                            let parent_id = parent.id;
                                                            let title = parent.title;
                                                            view! {
                                                                <button 
                                                                    on:click=move |_| app.start_course(parent_id)
                                                                    class="w-full flex items-center justify-between p-4 bg-slate-800 hover:bg-slate-700 rounded-xl transition-all border border-slate-700 hover:border-slate-600"
                                                                >
                                                                    <span class="font-bold text-white">{title}</span>
                                                                    <span class="text-xs font-black tracking-widest text-slate-400">"START PATH \u{25B6}"</span>
                                                                </button>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <button 
                                                    on:click=move |_| app.go_to_catalog()
                                                    class="px-8 py-3 bg-slate-800 hover:bg-slate-700 text-white font-black tracking-widest text-sm rounded-xl transition-all"
                                                >
                                                    "RETURN TO DASHBOARD"
                                                </button>
                                            }.into_any()
                                        }
                                    }
                                </div>
                            }.into_any()
                        }
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}
