use crate::state::{AppState, AppView};
use crate::utils;
use leptos::prelude::*;
use web_sys::HtmlTextAreaElement;

#[component]
pub fn Editor(app: AppState) -> impl IntoView {
    let code = app.code;
    let current_step = app.current_step;
    let current_view = app.current_view;
    let textarea_ref = NodeRef::<leptos::html::Textarea>::new();
    let highlight_ref = NodeRef::<leptos::html::Div>::new();

    let is_readonly = Signal::derive(move || {
        if let AppView::Lesson { ref course_id } = current_view.get() {
            if let Some(course) = app.get_course(course_id) {
                let step = current_step.get();
                if step < course.modules.len() {
                    return matches!(
                        course.modules[step].module_type,
                        crate::data::ModuleType::Concept
                    );
                }
            }
        }
        false
    });

    let placeholder = Signal::derive(move || {
        if let AppView::Lesson { ref course_id } = current_view.get() {
            if let Some(course) = app.get_course(course_id) {
                let step = current_step.get();
                if step < course.modules.len() {
                    return match course.modules[step].module_type {
                        crate::data::ModuleType::Concept => {
                            "Read the concept on the left to proceed..."
                        }
                        crate::data::ModuleType::Practice => "Write your Rust code here...",
                    };
                }
            }
        }
        "Write your Rust code here..."
    });

    let highlighted_code = Signal::derive(move || utils::highlight_rust_code(&code.get()));

    view! {
        <section class="flex-1 flex flex-col bg-[#050506]">
            <div class="flex-1 flex flex-col relative min-h-0">
                <div class="bg-black/40 px-6 py-2.5 flex items-center justify-between border-b border-white/5 shrink-0">
                    <div class="flex items-center gap-4">
                        <span class="text-[10px] font-mono text-slate-500 uppercase tracking-widest font-bold flex items-center gap-2">
                            <span class="text-orange-500">{"\u{1F4DD}"}</span>
                            traits.rs
                        </span>
                    </div>
                    <button
                        on:click=move |_| app.reset_code()
                        class="text-slate-600 hover:text-white transition-colors p-1 rounded-md hover:bg-white/5"
                    >
                        <span title="Reset">{"\u{21BB}"}</span>
                    </button>
                </div>
                <div class="relative flex-1 min-h-0 editor-shell">
                    <div
                        node_ref=highlight_ref
                        class="absolute inset-0 overflow-hidden pointer-events-none editor-pane"
                    >
                        <pre
                            class="editor-code editor-highlight text-sm"
                            inner_html=move || highlighted_code.get()
                        />
                    </div>
                    <Show when=move || code.get().is_empty()>
                        <div class="absolute top-10 left-10 text-slate-600 font-mono text-sm pointer-events-none">
                            {move || placeholder.get()}
                        </div>
                    </Show>
                    <textarea
                        node_ref=textarea_ref
                        class="absolute inset-0 w-full h-full bg-transparent resize-none focus:outline-none selection:bg-orange-500/30 text-sm leading-relaxed custom-scrollbar disabled:opacity-30 editor-pane editor-code editor-input"
                        prop:value=move || code.get()
                        on:input=move |ev| {
                            let val = event_target_value(&ev);
                            app.set_code(val);
                        }
                        on:scroll=move |_| sync_editor_scroll(&textarea_ref, &highlight_ref)
                        on:keydown=move |ev| {
                            if is_readonly.get() {
                                return;
                            }
                            handle_editor_keydown(ev, &textarea_ref, &app);
                            sync_editor_scroll(&textarea_ref, &highlight_ref);
                        }
                        spellcheck="false"
                        readonly=move || is_readonly.get()
                    />
                </div>
            </div>
        </section>
    }
}

fn handle_editor_keydown(
    ev: web_sys::KeyboardEvent,
    textarea_ref: &NodeRef<leptos::html::Textarea>,
    app: &AppState,
) {
    let Some(textarea) = textarea_ref.get() else {
        return;
    };

    let key = ev.key();
    let value = textarea.value();
    let start = textarea
        .selection_start()
        .ok()
        .flatten()
        .unwrap_or(value.len() as u32) as usize;
    let end = textarea
        .selection_end()
        .ok()
        .flatten()
        .unwrap_or(start as u32) as usize;
    let has_modifier = ev.meta_key() || ev.ctrl_key() || ev.alt_key();

    if key == "Tab" {
        ev.prevent_default();
        if ev.shift_key() {
            outdent_selection(&textarea, app, &value, start, end);
        } else {
            indent_selection(&textarea, app, &value, start, end);
        }
        return;
    }

    if key == "Enter" && !has_modifier {
        ev.prevent_default();
        insert_newline_with_indent(&textarea, app, &value, start, end);
        return;
    }

    if has_modifier {
        return;
    }

    if let Some((open, close)) = pair_for_key(&key) {
        ev.prevent_default();
        insert_pair(&textarea, app, &value, start, end, open, close);
        return;
    }

    if let Some(close) = closing_for_key(&key) {
        if start == end && value[start..].starts_with(close) {
            ev.prevent_default();
            let next = start + close.len();
            let _ = textarea.set_selection_range(next as u32, next as u32);
        }
    }
}

fn sync_editor_scroll(
    textarea_ref: &NodeRef<leptos::html::Textarea>,
    highlight_ref: &NodeRef<leptos::html::Div>,
) {
    let Some(textarea) = textarea_ref.get() else {
        return;
    };
    let Some(highlight) = highlight_ref.get() else {
        return;
    };

    highlight.set_scroll_top(textarea.scroll_top());
    highlight.set_scroll_left(textarea.scroll_left());
}

fn indent_selection(
    textarea: &HtmlTextAreaElement,
    app: &AppState,
    value: &str,
    start: usize,
    end: usize,
) {
    if start == end {
        replace_selection(textarea, app, value, start, end, "    ", start + 4, start + 4);
        return;
    }

    let line_start = line_start(value, start);
    let selected = &value[line_start..end];
    let indented = selected
        .lines()
        .map(|line| format!("    {line}"))
        .collect::<Vec<_>>()
        .join("\n");
    let new_end = line_start + indented.len();
    replace_selection(textarea, app, value, line_start, end, &indented, start + 4, new_end);
}

fn outdent_selection(
    textarea: &HtmlTextAreaElement,
    app: &AppState,
    value: &str,
    start: usize,
    end: usize,
) {
    let line_start = line_start(value, start);
    let selected = &value[line_start..end];
    let mut removed_before_start = 0usize;
    let mut new_len = 0usize;

    let outdented = selected
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let trimmed = line.strip_prefix("    ").unwrap_or(line.strip_prefix('\t').unwrap_or(line));
            let removed = line.len() - trimmed.len();
            if index == 0 {
                removed_before_start = removed.min(start - line_start);
            }
            new_len += trimmed.len();
            trimmed.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");
    new_len += selected.matches('\n').count();

    let new_start = start.saturating_sub(removed_before_start);
    let new_end = line_start + new_len;
    replace_selection(textarea, app, value, line_start, end, &outdented, new_start, new_end);
}

fn insert_newline_with_indent(
    textarea: &HtmlTextAreaElement,
    app: &AppState,
    value: &str,
    start: usize,
    end: usize,
) {
    let before_cursor = &value[..start];
    let current_indent = before_cursor
        .rsplit_once('\n')
        .map(|(_, line)| leading_whitespace(line))
        .unwrap_or_else(|| leading_whitespace(before_cursor));
    let next_char = value[end..].chars().next();
    let trimmed_before = before_cursor.trim_end();
    let extra_indent = if trimmed_before.ends_with('{') { "    " } else { "" };
    let closing_indent = if next_char == Some('}') && !extra_indent.is_empty() {
        format!("\n{current_indent}")
    } else {
        String::new()
    };

    let insertion = format!("\n{current_indent}{extra_indent}{closing_indent}");
    let caret = start + 1 + current_indent.len() + extra_indent.len();
    replace_selection(textarea, app, value, start, end, &insertion, caret, caret);
}

fn insert_pair(
    textarea: &HtmlTextAreaElement,
    app: &AppState,
    value: &str,
    start: usize,
    end: usize,
    open: &str,
    close: &str,
) {
    let selected = &value[start..end];
    let replacement = format!("{open}{selected}{close}");
    let caret = if start == end {
        start + open.len()
    } else {
        start + replacement.len()
    };
    replace_selection(textarea, app, value, start, end, &replacement, caret, caret);
}

fn replace_selection(
    textarea: &HtmlTextAreaElement,
    app: &AppState,
    value: &str,
    start: usize,
    end: usize,
    replacement: &str,
    new_start: usize,
    new_end: usize,
) {
    let mut next = String::with_capacity(value.len() + replacement.len());
    next.push_str(&value[..start]);
    next.push_str(replacement);
    next.push_str(&value[end..]);

    textarea.set_value(&next);
    app.set_code(next);
    let _ = textarea.set_selection_range(new_start as u32, new_end as u32);
}

fn pair_for_key(key: &str) -> Option<(&'static str, &'static str)> {
    match key {
        "{" => Some(("{", "}")),
        "(" => Some(("(", ")")),
        "[" => Some(("[", "]")),
        "\"" => Some(("\"", "\"")),
        "'" => Some(("'", "'")),
        _ => None,
    }
}

fn closing_for_key(key: &str) -> Option<&'static str> {
    match key {
        "}" => Some("}"),
        ")" => Some(")"),
        "]" => Some("]"),
        "\"" => Some("\""),
        "'" => Some("'"),
        _ => None,
    }
}

fn line_start(value: &str, position: usize) -> usize {
    value[..position]
        .rfind('\n')
        .map(|index| index + 1)
        .unwrap_or(0)
}

fn leading_whitespace(line: &str) -> String {
    line.chars()
        .take_while(|ch| ch.is_whitespace() && *ch != '\n')
        .collect()
}
