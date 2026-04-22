use crate::state::{AppState, LineType};
use leptos::prelude::*;
use web_sys::window;

fn is_mac() -> bool {
    window()
        .and_then(|w| w.navigator().user_agent().ok())
        .map(|ua| ua.contains("Macintosh") || ua.contains("Mac OS X"))
        .unwrap_or(false)
}

#[component]
pub fn Terminal(app: AppState) -> impl IntoView {
    let terminal_lines = app.terminal_lines;
    let mac = is_mac();

    let run_hint = if mac { "⌘Enter" } else { "Ctrl+Enter" };
    let next_hint = if mac { "Ctrl+N" } else { "Ctrl+→" };
    let prev_hint = if mac { "Ctrl+P" } else { "Ctrl+←" };
    let search_hint = if mac { "⌘K" } else { "Ctrl+K" };
    let clear_hint = if mac { "⌘⇧X" } else { "Ctrl+⇧X" };

    // Auto-scroll effect
    let terminal_container = NodeRef::<leptos::html::Div>::new();
    Effect::new(move |_| {
        let _ = terminal_lines.get();
        if let Some(el) = terminal_container.get() {
            el.set_scroll_top(el.scroll_height());
        }
    });

    view! {
        <div class="h-40 bg-black/80 border-t border-white/5 flex flex-col shrink-0">
            <div class="bg-slate-900/40 px-6 py-1.5 flex items-center justify-between text-xs font-mono text-slate-600 uppercase tracking-widest font-bold border-b border-white/5">
                <div class="flex items-center gap-3">
                    <span>{"\u{2713}"}</span>
                    <span>validation_feedback</span>
                </div>
                <div class="flex items-center gap-3 text-[9px] font-mono normal-case tracking-normal text-slate-700">
                    <span class="flex items-center gap-1"><kbd class="bg-white/5 border border-white/10 rounded px-1">{run_hint}</kbd> Run</span>
                    <span class="flex items-center gap-1"><kbd class="bg-white/5 border border-white/10 rounded px-1">{next_hint}</kbd> Next</span>
                    <span class="flex items-center gap-1"><kbd class="bg-white/5 border border-white/10 rounded px-1">{prev_hint}</kbd> Prev</span>
                    <span class="flex items-center gap-1"><kbd class="bg-white/5 border border-white/10 rounded px-1">{clear_hint}</kbd> Clear</span>
                    <span class="flex items-center gap-1"><kbd class="bg-white/5 border border-white/10 rounded px-1">{search_hint}</kbd> Search</span>
                </div>
            </div>
            <div 
                node_ref=terminal_container
                class="flex-1 overflow-y-auto p-5 font-mono text-xs custom-scrollbar selection:bg-orange-500/30"
            >
                {move || {
                    terminal_lines.get().into_iter().map(|line| {
                        let color = match line.line_type {
                            LineType::Error => "text-red-400",
                            LineType::Success => "text-green-400",
                            LineType::Info => "text-slate-600",
                        };
                        let text = line.text;
                        // For now simple line display, later we add ANSI support
                        view! { <div class=format!("mb-1 whitespace-pre-wrap {color}")>{text}</div> }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}
