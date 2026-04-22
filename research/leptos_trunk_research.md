# Leptos + Trunk Research: Interactive Tutorial Code Editor App

## 1. Project Setup: New Leptos Project with Trunk

### Option A: `create-leptos-csr` CLI (Recommended for CSR + Tailwind)
```bash
cargo install create-leptos-csr-tw
create-leptos-csr-tw
```
This generates a complete CSR Leptos app with:
- TailwindCSS pre-configured
- Trunk build setup
- `leptos-use` utilities included
- Mobile viewport config
- Public assets dir

### Option B: Manual Setup
```bash
cargo init --name rust-trait-mastery
# Add to Cargo.toml:
#   leptos = { version = "0.8", features = ["csr"] }
#   leptos_router = "0.8"
#   console_error_panic_hook = "0.1"
#   console_log = "1"
#   log = "0.4"
#   serde = { version = "1", features = ["derive"] }
#   wasm-bindgen = "0.2"
cargo install trunk --locked
```

### Option C: `cargo generate` with Leptos template
```bash
cargo install cargo-generate
cargo generate leptos-rs/start  # Full-stack template (overkill for CSR-only)
```

### Minimum `index.html`:
```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <link data-trunk rel="rust" data-wasm-opt="z" />
    <link data-trunk rel="tailwind-css" href="input.css" />
    <title>Rust Trait Mastery</title>
  </head>
  <body></body>
</html>
```

### Minimum `main.rs` (Leptos 0.8 CSR):
```rust
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! { <h1>"Hello Leptos 0.8"</h1> }
}
```

---

## 2. Recommended Project Structure

```
rust-trait-mastery/
├── Cargo.toml
├── Trunk.toml
├── index.html
├── input.css                    # Tailwind input (@import "tailwindcss";)
├── public/                      # Static assets (favicon, etc.)
│   └── favicon.ico
├── src/
│   ├── main.rs                  # Entry point: mount_to_body(App)
│   ├── app.rs                   # Root App component (layout shell)
│   ├── components/
│   │   ├── mod.rs
│   │   ├── header.rs            # Progress dots header
│   │   ├── tutorial_panel.rs    # Left: markdown tutorial content
│   │   ├── code_panel.rs        # Right: code editor (kode-leptos)
│   │   ├── terminal_panel.rs    # Bottom: diagnostics/output
│   │   ├── nav_footer.rs        # Prev/Next navigation
│   │   └── progress_dots.rs     # Module progress indicators
│   ├── modules/
│   │   ├── mod.rs               # TutorialModule struct definition
│   │   ├── data.rs              # All 32 modules as const data
│   │   └── content/             # Per-module markdown files
│   │       ├── 01_trait_basics.md
│   │       ├── 02_trait_bounds.md
│   │       └── ...
│   ├── state/
│   │   ├── mod.rs
│   │   └── tutorial_state.rs    # Global state (current module, progress)
│   └── utils/
│       ├── mod.rs
│       └── solution_checker.rs  # Regex-based solution validation
└── style/                       # Optional: additional SCSS
    └── custom.scss
```

---

## 3. Client-side Markdown Rendering in Leptos/WASM

### Option A: `markdown_view_leptos` (MDX-like, embeds Leptos components)
```toml
markdown_view_leptos = "0.1"
```
- Renders markdown with embedded Leptos components via macro
- Version 0.1.92, MIT license
- Good for interactive tutorial content

### Option B: `leptos-md` (Simple, Tailwind-styled)
```toml
leptos-md = "0.1"  # Uses pulldown-cmark internally
```
- Signal-free Markdown renderer for Leptos
- Built-in Tailwind CSS styling
- Supports `full` feature with SIMD optimization

### Option C: Manual with `pulldown-cmark` or `comrak`
```toml
pulldown-cmark = { version = "0.13", default-features = false }
# or
comrak = { version = "0.52", default-features = false }  # GFM support
```
Both compile to WASM with `default-features = false` (disables fs, SIMD).

Example with pulldown-cmark:
```rust
use pulldown_cmark::{html, Options, Parser};

pub fn render_markdown(md: &str) -> String {
    let options = Options::empty();
    let parser = Parser::new_ext(md, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

// In component:
view! {
    <div inner_html=render_markdown(&module.content) />
}
```

### Recommendation: Use `leptos-md` for simplicity or `pulldown-cmark` for full control.

---

## 4. Split-Pane Layout

### Use `leptos-resize` crate (v0.3.0, Leptos 0.8 compatible)
```toml
leptos-resize = "0.3"
```

```rust
use leptos_resize::{ResizableSplit, SplitDirection};

#[component]
fn App() -> impl IntoView {
    let main_split = RwSignal::new(vec![50.0]); // 50/50 vertical split
    let bottom_split = RwSignal::new(vec![70.0]); // 70/30 horizontal split

    view! {
        <div style="height: 100vh; display: flex; flex-direction: column;">
            <!-- Main area: tutorial left, code right -->
            <div style="flex: 1;">
                <ResizableSplit percentages=main_split>
                    <TutorialPanel />    // Left: markdown content
                    <CodePanel />        // Right: code editor
                </ResizableSplit>
            </div>
            <!-- Bottom: terminal/diagnostics -->
            <div style="height: 30%;">
                <TerminalPanel />
            </div>
        </div>
    }
}
```

Features:
- Horizontal (Row) and vertical (Column) splits
- Signal-bound percentages for reactive sizing
- Nested splits for complex layouts
- Custom handle styling via `handle_class` prop

### Alternative: Pure CSS with Tailwind (no crate needed)
```rust
view! {
    <div class="h-screen flex flex-col">
        <div class="flex-1 flex">
            <div class="w-1/2 overflow-auto">/* Tutorial */</div>
            <div class="w-px bg-gray-700 cursor-col-resize">/* Divider */</div>
            <div class="w-1/2">/* Editor */</div>
        </div>
        <div class="h-48 border-t border-gray-700">/* Terminal */</div>
    </div>
}
```

---

## 5. Syntax Highlighting for Rust Code in WASM/Leptos

### RECOMMENDED: `kode-leptos` (v0.2.0)
```toml
kode-leptos = "0.2"
# Enable specific languages via arborium features
arborium = { version = "2.16", default-features = false, features = ["lang-rust"] }
```

```rust
use kode_leptos::{CodeEditor, Language, Theme, EditorHandle, Marker, MarkerSeverity, Position};
use std::sync::Arc;

#[component]
fn CodePanel() -> impl IntoView {
    let content = RwSignal::new(String::from("fn main() {}"));

    view! {
        <div style="height: 100%;">
            <CodeEditor
                language=Signal::stored(Language::Rust)
                content=content.read_only()
                theme=Signal::stored(Theme::tokyo_night())  // Dark theme built-in
                on_change=Arc::new(move |text: String| {
                    content.set(text);
                })
            />
        </div>
    }
}
```

Features:
- Syntax highlighting for Rust, Python, JS/TS, HTML, CSS, JSON, SQL, YAML, Bash, Markdown
- Built-in themes: `tokyo_night`, `one_dark`, `github_light`
- Diagnostics API with markers (error/warning/info squiggles)
- Completions API
- Imperative `EditorHandle` for programmatic control
- Multi-cursor, undo/redo, IME, placeholder text

### Diagnostics integration (for solution checking):
```rust
let (diagnostics, set_diagnostics) = signal(Vec::<Marker>::new());

let providers = Signal::derive(move || {
    vec![DiagnosticProvider::new(|text: &str| {
        // Run solution regex, return markers for errors
        vec![]
    })]
});

view! {
    <CodeEditor
        language=Signal::stored(Language::Rust)
        content=content.read_only()
        diagnostic_providers=providers
        diagnostic_debounce_ms=Some(300)
    />
}
```

---

## 6. Reusable Components Structure

### Tutorial State (using Leptos Context + Signals):
```rust
// state/tutorial_state.rs
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TutorialProgress {
    pub completed_modules: Vec<usize>,  // indices of completed modules
    pub current_module: usize,
}

pub fn provide_tutorial_state(modules: Vec<TutorialModule>) -> RwSignal<TutorialProgress> {
    // Load from localStorage or start fresh
    let stored = load_from_storage();
    let state = RwSignal::new(stored.unwrap_or(TutorialProgress {
        completed_modules: vec![],
        current_module: 0,
    }));
    provide_context(modules);
    provide_context(state);
    state
}
```

### Progress Dots Component:
```rust
#[component]
fn ProgressDots() -> impl IntoView {
    let modules = use_context::<Vec<TutorialModule>>().unwrap();
    let progress = use_context::<RwSignal<TutorialProgress>>().unwrap();

    view! {
        <div class="flex gap-2">
            {modules.iter().enumerate().map(|(i, m)| {
                let is_completed = move || progress.read().completed_modules.contains(&i);
                let is_current = move || progress.read().current_module == i;
                let color = if is_completed() { "bg-green-500" }
                    else if is_current() { "bg-blue-500" }
                    else { "bg-gray-600" };
                view! {
                    <div class={format!("w-3 h-3 rounded-full {}", color)}
                         on:click=move |_| { /* navigate to module i */ } />
                }
            }).collect_view()}
        </div>
    }
}
```

### Navigation Footer:
```rust
#[component]
fn NavFooter() -> impl IntoView {
    let modules = use_context::<Vec<TutorialModule>>().unwrap();
    let progress = use_context::<RwSignal<TutorialProgress>>().unwrap();

    let can_prev = move || progress.read().current_module > 0;
    let can_next = move || progress.read().current_module < modules.len() - 1;

    view! {
        <div class="flex justify-between p-4">
            <button class="px-4 py-2 bg-gray-700 rounded disabled:opacity-50"
                    disabled=move || !can_prev()
                    on:click=move |_| {
                        progress.update(|p| if p.current_module > 0 { p.current_module -= 1; });
                    }>
                "← Previous"
            </button>
            <span>{move || format!("{} / {}", progress.read().current_module + 1, modules.len())}</span>
            <button class="px-4 py-2 bg-blue-600 rounded disabled:opacity-50"
                    disabled=move || !can_next()
                    on:click=move |_| {
                        progress.update(|p| if p.current_module < modules.len() - 1 { p.current_module += 1; });
                    }>
                "Next →"
            </button>
        </div>
    }
}
```

---

## 7. Tutorial Module Data Structure

### Module definition:
```rust
// modules/mod.rs
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ModuleType {
    Concept,  // Reading/learning module
    Practice, // Coding exercise
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TutorialModule {
    pub id: usize,
    pub title: String,
    pub module_type: ModuleType,
    pub content: String,           // Markdown content
    pub initial_code: String,      // Starting code for practice modules
    pub solution_regex: Option<String>,  // Regex to validate solution
    pub success_message: Option<String>,
}
```

### Data loading strategies:

#### Strategy A: Compile-time with `include_str!` (Recommended for 32 modules)
```rust
// modules/data.rs
use crate::modules::{TutorialModule, ModuleType};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: 0,
        title: "Trait Basics".into(),
        module_type: ModuleType::Concept,
        content: include_str!("content/01_trait_basics.md").into(),
        initial_code: String::new(),
        solution_regex: None,
        success_message: None,
    },
    TutorialModule {
        id: 1,
        title: "Implementing Traits".into(),
        module_type: ModuleType::Practice,
        content: include_str!("content/02_implementing_traits.md").into(),
        initial_code: include_str!("content/02_initial_code.rs").into(),
        solution_regex: Some(r"impl\s+Display\s+for\s+MyStruct".into()),
        success_message: Some("Great! You implemented Display correctly.".into()),
    },
    // ... 30 more modules
];
```

#### Strategy B: `include_dir` for directory embedding
```toml
include_dir = "0.7"
```
```rust
use include_dir::{include_dir, Dir};
static MODULE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/modules/content");

// Access files at compile time
let content = MODULE_DIR.get_file("01_trait_basics.md")
    .unwrap()
    .contents_utf8()
    .unwrap();
```

#### Strategy C: JSON data file
Bundle a `modules.json` file and parse at startup. Less efficient but easier to generate programmatically.

### Solution checking:
```rust
// utils/solution_checker.rs
use regex::Regex;

pub fn check_solution(code: &str, pattern: &str) -> bool {
    Regex::new(pattern)
        .map(|re| re.is_match(code))
        .unwrap_or(false)
}
```

---

## 8. TailwindCSS Integration with Leptos/Trunk

### Trunk v0.21+ has built-in Tailwind support:

**Trunk.toml:**
```toml
[tools]
tailwindcss = "4.1.13"  # Trunk downloads this automatically
```

**index.html:**
```html
<link data-trunk rel="tailwind-css" href="input.css" />
```

**input.css:**
```css
@import "tailwindcss";
```

### For Trunk hooks-based approach (older Trunk or custom config):
```toml
# Trunk.toml
[[hooks]]
stage = "pre_build"
command = "npx"
command_arguments = ["tailwindcss", "-i", "./input.css", "-o", "./dist/output.css", "--minify"]
```

### Dark theme configuration:
```css
@import "tailwindcss";

@theme {
  --color-bg-primary: #1a1b26;
  --color-bg-secondary: #1e1e2e;
  --color-bg-tertiary: #2a2b3d;
  --color-text-primary: #a9b1d6;
  --color-text-secondary: #565f89;
  --color-accent: #7aa2f7;
  --color-success: #9ece6a;
  --color-error: #f7768e;
}
```

### `tailwind-rs-leptos` crate (optional utility):
```toml
tailwind-rs-leptos = "0.15"
```
Provides type-safe Tailwind class generation for Leptos.

---

## 9. Commonly Used Crates

| Crate | Version | Purpose |
|-------|---------|---------|
| `leptos` | 0.8.19 | Core framework (CSR mode) |
| `leptos_router` | 0.8.13 | Client-side routing |
| `leptos-use` | 0.18.3 | Utilities (use_local_storage, use_debounce_fn, etc.) |
| `leptos-resize` | 0.3.0 | Resizable split panes |
| `kode-leptos` | 0.2.0 | Code editor with syntax highlighting |
| `arborium` | 2.16.0 | Tree-sitter syntax highlighting (used by kode) |
| `leptos-md` | 0.1.0 | Markdown renderer with Tailwind |
| `markdown_view_leptos` | 0.1.92 | MDX-like markdown with embedded components |
| `pulldown-cmark` | 0.13.3 | Low-level CommonMark parser |
| `comrak` | 0.52.0 | GFM parser (GitHub Flavored Markdown) |
| `regex` | 1.x | Solution regex validation |
| `serde` | 1.x | Serialization (localStorage, JSON) |
| `serde_json` | 1.x | JSON parsing |
| `include_dir` | 0.7.4 | Embed directories at compile time |
| `console_error_panic_hook` | 0.1.7 | Better WASM error messages |
| `console_log` | 1.0 | Logging in WASM |
| `trunk` | 0.21.0 | Build tool (binary, not a crate dep) |
| `gloo-net` | 0.7.0 | HTTP requests (if needed) |
| `leptos-shadcn-ui` | 0.9.0 | UI component library (optional) |
| `leptos-lucide-rs` | 0.2.0 | Icon library (optional) |
| `leptos_icons` | 0.7.1 | Icon components (optional) |

### Key leptos-use features for this project:
```toml
leptos-use = { version = "0.18", features = ["use_local_storage"] }
```
- `use_local_storage` - persist tutorial progress
- `use_debounce_fn` - debounce code evaluation
- `use_element_size` - responsive layout adjustments
- `use_cycle_list` - cycle through themes

---

## 10. Trunk Configuration

### Complete Trunk.toml:
```toml
[build]
dist = "dist"              # Output directory

[tools]
tailwindcss = "4.1.13"     # Built-in Tailwind support

# Optional: SCSS compilation
# dart-sass is auto-downloaded if .scss files are detected

# Optional: Hooks for custom build steps
# [[hooks]]
# stage = "pre_build"
# command = "echo"
# command_arguments = ["Building tutorial assets..."]

# Optional: Watch config
[watch]
watch = ["src", "public"]

# Optional: Serve config
[serve]
port = 3000
open = false

# Optional: Proxy (if connecting to backend)
# [[proxies]]
# backend = "http://localhost:8080"
```

### Cargo.toml (complete):
```toml
[package]
name = "rust-trait-mastery"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[profile.wasm-release]
inherits = "release"
opt-level = 'z'
lto = true
codegen-units = 1
panic = "abort"

[dependencies]
leptos = { version = "0.8", features = ["csr"] }
leptos_router = "0.8"
leptos-use = { version = "0.18", features = ["use_local_storage"] }
leptos-resize = "0.3"
kode-leptos = "0.2"
leptos-md = "0.1"
arborium = { version = "2.16", default-features = false, features = ["lang-rust"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
regex = "1"
pulldown-cmark = { version = "0.13", default-features = false }
console_error_panic_hook = "0.1"
console_log = "1"
log = "0.4"
include_dir = "0.7"
wasm-bindgen = "0.2"
web-sys = "0.3"
```

---

## Migration Strategy from React (744 lines main.ts)

1. **Data layer first**: Convert the 32 module definitions to Rust structs with `include_str!`
2. **State management**: Replace React useState/useContext with Leptos signals (`RwSignal`) and context (`provide_context`/`use_context`)
3. **UI components**: Rewrite each React component as a Leptos `#[component]`
4. **Code editor**: Replace the React code editor with `kode-leptos`'s `CodeEditor`
5. **Markdown rendering**: Replace React markdown library with `leptos-md` or `pulldown-cmark`
6. **Styling**: Convert Tailwind classes directly (same utility classes work in Leptos)
7. **Solution checking**: Port regex-based validation to Rust `regex` crate
8. **Persistence**: Replace localStorage hooks with `leptos-use`'s `use_local_storage`

---

## Development Workflow

```bash
# Install prerequisites
rustup target add wasm32-unknown-unknown
cargo install trunk --locked

# Start dev server (auto-reload on changes)
trunk serve --open

# Build for production
trunk build --release

# Check WASM compatibility
cargo check --target wasm32-unknown-unknown
```

---

## Key Leptos 0.8 API Changes (vs 0.6/0.7)

- `create_signal` → `signal`
- `create_rw_signal` → `RwSignal::new`
- `Signal::derive` still works
- `view!` macro same syntax
- `mount_to_body` for CSR
- `.get()` on signals → `.read()` or `.get()` (both work)
- `.set()` on signals → `.set()` or `.write()`
- `For` component for keyed lists
- `Suspend` for async rendering
- `path!()` macro for type-safe routes
