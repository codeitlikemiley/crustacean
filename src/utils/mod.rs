use pulldown_cmark::{html, Options, Parser};
use regex::Regex;

pub fn render_markdown(text: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(text, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Post-process: apply syntax highlighting to code blocks
    highlight_code_blocks(&mut html_output);

    html_output
}

fn highlight_code_blocks(html: &mut String) {
    let re = Regex::new(r#"<pre><code(?:\s+class="language-\w+")?>((?s).*?)</code></pre>"#)
        .unwrap();

    let result = re.replace_all(html, |caps: &regex::Captures| {
        let raw_code = &caps[1];
        // Decode HTML entities back to plain text for the highlighter
        let plain = raw_code
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'");
        let highlighted = highlight_rust_code(&plain);
        format!(r#"<pre class="highlighted-block"><code>{}</code></pre>"#, highlighted)
    });

    *html = result.into_owned();
}

pub fn highlight_rust_code(code: &str) -> String {
    let mut output = String::new();
    let chars: Vec<char> = code.chars().collect();
    let mut i = 0usize;

    while i < chars.len() {
        let ch = chars[i];

        if ch == '/' && chars.get(i + 1) == Some(&'/') {
            let start = i;
            i += 2;
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            wrap_token(&mut output, "editor-comment", &chars[start..i].iter().collect::<String>());
            continue;
        }

        if ch == '"' {
            let start = i;
            i += 1;
            while i < chars.len() {
                if chars[i] == '\\' {
                    i += 2;
                    continue;
                }
                if chars[i] == '"' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            wrap_token(&mut output, "editor-string", &chars[start..i].iter().collect::<String>());
            continue;
        }

        if ch.is_ascii_digit() {
            let start = i;
            i += 1;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '_') {
                i += 1;
            }
            wrap_token(&mut output, "editor-number", &chars[start..i].iter().collect::<String>());
            continue;
        }

        if is_ident_start(ch) {
            let start = i;
            i += 1;
            while i < chars.len() && is_ident_continue(chars[i]) {
                i += 1;
            }
            let mut token = chars[start..i].iter().collect::<String>();

            if chars.get(i) == Some(&'!') {
                token.push('!');
                i += 1;
                wrap_token(&mut output, "editor-macro", &token);
                continue;
            }

            if is_rust_keyword(&token) {
                wrap_token(&mut output, "editor-keyword", &token);
            } else if token.chars().next().is_some_and(|first| first.is_uppercase()) {
                wrap_token(&mut output, "editor-type", &token);
            } else {
                output.push_str(&escape_html(&token));
            }
            continue;
        }

        if matches!(ch, '{' | '}' | '(' | ')' | '[' | ']') {
            wrap_token(&mut output, "editor-brace", &ch.to_string());
            i += 1;
            continue;
        }

        if matches!(ch, '<' | '>' | ':' | ';' | ',' | '=') {
            wrap_token(&mut output, "editor-punctuation", &ch.to_string());
            i += 1;
            continue;
        }

        match ch {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            _ => output.push(ch),
        }
        i += 1;
    }

    if output.is_empty() {
        output.push(' ');
    } else if code.ends_with('\n') {
        output.push(' ');
    }

    output
}

fn wrap_token(output: &mut String, class_name: &str, token: &str) {
    output.push_str("<span class=\"");
    output.push_str(class_name);
    output.push_str("\">");
    output.push_str(&escape_html(token));
    output.push_str("</span>");
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_ident_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

fn is_rust_keyword(token: &str) -> bool {
    matches!(
        token,
        "as"
            | "async"
            | "await"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
    )
}
