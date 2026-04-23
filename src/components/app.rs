use crate::components::catalog::Catalog;
use crate::components::completion_screen::CompletionScreen;
use crate::components::command_palette::CommandPalette;
use crate::components::content::Content;
use crate::components::course_detail::CourseDetail;
use crate::components::editor::Editor;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::keyboard_shortcuts::KeyboardShortcuts;
use crate::components::terminal::Terminal;
use crate::data::ModuleType;
use crate::state::{AppState, AppView};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let app = AppState::init();
    provide_context(app);

    let current_view = app.current_view;

    let is_catalog = Signal::derive(move || matches!(current_view.get(), AppView::Catalog));
    let is_detail = Signal::derive(move || matches!(current_view.get(), AppView::CourseDetail { .. }));
    let is_lesson = Signal::derive(move || matches!(current_view.get(), AppView::Lesson { .. }));

    let is_completed_screen = Signal::derive(move || {
        if let AppView::Lesson { ref course_id } = current_view.get() {
            if let Some(course) = app.get_course(course_id) {
                return app.current_step.get() >= course.lesson_count();
            }
        }
        false
    });

    let is_active_lesson = Signal::derive(move || is_lesson.get() && !is_completed_screen.get());

    let is_concept = Signal::derive(move || {
        if let AppView::Lesson { ref course_id } = current_view.get() {
            if let Some(course) = app.get_course(course_id) {
                let step = app.current_step.get();
                if step < course.lesson_count() {
                    return matches!(course.get_module(step).unwrap().module_type, ModuleType::Concept);
                }
            }
        }
        false
    });

    let detail_course_id = Signal::derive(move || {
        if let AppView::CourseDetail { ref course_id } = current_view.get() {
            Some(course_id.clone())
        } else {
            None
        }
    });

    view! {
        <div class="flex flex-col h-screen bg-neutral-950 text-slate-300 font-sans overflow-hidden">
            <Header app />

            <Show when=move || is_catalog.get() fallback=|| ()>
                <Catalog app />
            </Show>

            <Show when=move || is_detail.get() fallback=|| ()>
                {move || {
                    let cid = detail_course_id.get().unwrap_or_default();
                    view! { <CourseDetail app course_id=cid /> }
                }}
            </Show>

            <Show when=move || is_active_lesson.get() fallback=|| ()>
                <main class="flex flex-col lg:flex-row flex-1 overflow-hidden">
                    <Content app is_concept=is_concept />
                    <Show when=move || !is_concept.get() fallback=|| ()>
                        <div class="flex-1 flex flex-col min-w-0">
                            <Editor app />
                            <Terminal app />
                        </div>
                    </Show>
                </main>
            </Show>

            <Show when=move || is_completed_screen.get() fallback=|| ()>
                <CompletionScreen app />
            </Show>

            <Show
                when=move || is_active_lesson.get()
                fallback=|| ()
            >
                <Footer app />
            </Show>

            // Global overlays
            <KeyboardShortcuts app />
            <CommandPalette app />
        </div>
    }
}
