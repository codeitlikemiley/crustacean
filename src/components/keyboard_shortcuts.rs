#![allow(unused_parens)]
use crate::state::{AppState, AppView};
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

/// Detect if running on macOS
fn is_mac() -> bool {
    window()
        .and_then(|w| w.navigator().user_agent().ok())
        .map(|ua| ua.contains("Macintosh") || ua.contains("Mac OS X"))
        .unwrap_or(false)
}

#[component]
pub fn KeyboardShortcuts(app: AppState) -> impl IntoView {
    let mac = is_mac();

    // Attach global keydown listener
    Effect::new(move |_| {
        let app = app;
        let mac = mac;

        let closure = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |ev: web_sys::KeyboardEvent| {
            let tag = ev.target().map(|t| {
                t.dyn_ref::<web_sys::HtmlElement>()
                    .map(|el| el.tag_name())
                    .unwrap_or_default()
            }).unwrap_or_default();
            let is_input = tag == "INPUT" || tag == "TEXTAREA";

            let key = ev.key();
            let meta = ev.meta_key();
            let ctrl = ev.ctrl_key();
            let shift = ev.shift_key();

            // Modifier depends on platform
            let mod_pressed = if mac { meta } else { ctrl };

            // ─── Command palette: Cmd/Ctrl+K ─────────────────────
            if mod_pressed && key == "k" {
                ev.prevent_default();
                app.show_command_palette.update(|v| *v = !*v);
                return;
            }

            // ─── Close palette: Escape ───────────────────────────
            if key == "Escape" {
                if app.show_command_palette.get() {
                    ev.prevent_default();
                    app.show_command_palette.set(false);
                }
                return;
            }

            let view = app.current_view.get();

            // ─── Course Detail Navigation ────────────────────────
            if let AppView::CourseDetail { ref course_id } = view {
                if !is_input {
                    if mod_pressed && key == "Enter" {
                        ev.prevent_default();
                        app.resume_prioritized_course(course_id);
                        return;
                    }
                    if key == "ArrowDown" || key == "j" {
                        ev.prevent_default();
                        app.select_next_lesson();
                        return;
                    }
                    if key == "ArrowUp" || key == "k" {
                        ev.prevent_default();
                        app.select_prev_lesson();
                        return;
                    }
                    if key == "Enter" || key == "l" {
                        ev.prevent_default();
                        app.open_selected_lesson();
                        return;
                    }
                }
            }

            // Only process lesson shortcuts when in lesson view
            if !matches!(view, AppView::Lesson { .. }) {
                return;
            }

            // ─── Run: Cmd/Ctrl+Enter ─────────────────────────────
            if mod_pressed && key == "Enter" {
                ev.prevent_default();
                if app.handle_run() {
                    app.next_step();
                }
                return;
            }

            // ─── Next/Previous lesson (Bracket shortcuts) ────────
            if mod_pressed && !shift {
                if key == "]" {
                    ev.prevent_default();
                    app.next_step();
                    return;
                }
                if key == "[" {
                    ev.prevent_default();
                    app.prev_step();
                    return;
                }
            }

            // ─── Next/Previous lesson (Legacy/Alt) ───────────────
            if mac {
                if ctrl && key == "n" {
                    ev.prevent_default();
                    app.next_step();
                    return;
                }
                if ctrl && key == "p" {
                    ev.prevent_default();
                    app.prev_step();
                    return;
                }
            } else {
                if ctrl && key == "ArrowRight" {
                    ev.prevent_default();
                    app.next_step();
                    return;
                }
                if ctrl && key == "ArrowLeft" {
                    ev.prevent_default();
                    app.prev_step();
                    return;
                }
            }

            // ─── Clear diagnostics: Cmd/Ctrl+Shift+X ─────────────
            if mod_pressed && shift && key == "x" {
                ev.prevent_default();
                app.clear_diagnostics();
                return;
            }

            // ─── Lesson navigation with vim-like keys ─────────────
            if is_input && !mod_pressed && !ctrl {
                return;
            }

            if mac {
                if ctrl && !shift && key == "j" {
                    ev.prevent_default();
                    app.next_step();
                    return;
                }
                if ctrl && !shift && key == "k" {
                    ev.prevent_default();
                    app.prev_step();
                    return;
                }
                if ctrl && !shift && key == "l" {
                    ev.prevent_default();
                    return;
                }
                if meta && shift && key == "{" {
                    ev.prevent_default();
                    app.go_to_catalog();
                    return;
                }
            } else {
                if ctrl && !shift && key == "j" {
                    ev.prevent_default();
                    app.next_step();
                    return;
                }
                if ctrl && !shift && key == "k" {
                    ev.prevent_default();
                    app.prev_step();
                    return;
                }
                if ctrl && shift && key == "{" {
                    ev.prevent_default();
                    app.go_to_catalog();
                    return;
                }
            }
        });

        if let Some(window) = window() {
            let document = window.document().unwrap();
            document
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .unwrap();
            
            closure.forget();
        }
    });

    view! { <div style="display: none;" /> }
}
