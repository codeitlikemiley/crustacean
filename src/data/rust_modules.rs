use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "modules-1-concept",
        title: "1. Concept: Modules and `pub`",
        module_type: ModuleType::Concept,
        content: r#"
# Modules and Visibility

Rust uses **modules** to control scope and privacy. 

By default, everything in a module is private (hidden from the outside). To let other code use a function, struct, or module, you must mark it as `pub`.

```rust
mod network {
    // This is private!
    fn connect() {}

    pub mod server {
        // This is public!
        pub fn start() {}
    }
}

fn main() {
    // network::connect(); // ERROR: private
    network::server::start(); // OK!
}
```
        "#,
        initial_code: "// Learn about modules, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "modules-2-practice",
        title: "2. Practice: Make it Public",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `pub`

### Task:
Fix the code so that `main` can call `math::add`. You will need to add the `pub` keyword to the function.
        "#,
        initial_code: "mod math {\n    fn add(a: i32, b: i32) -> i32 {\n        a + b\n    }\n}\n\nfn main() {\n    math::add(1, 2);\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "make `add` public",
                    matcher: RuleMatcher::Regex(r#"pub\s+fn\s+add"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("mod math {\n    pub fn add(a: i32, b: i32) -> i32 {\n        a + b\n    }\n}\n\nfn main() {\n    math::add(1, 2);\n}"),
            hints: &[
                "Change `fn add` to `pub fn add`.",
            ],
        },
        success_message: "Great! You exposed an API from your module.",
    },
    TutorialModule {
        id: "modules-3-concept",
        title: "3. Concept: The `use` Keyword",
        module_type: ModuleType::Concept,
        content: r#"
# Bringing Paths into Scope

Writing out long paths like `std::collections::HashMap` every time is annoying. You can bring paths into scope with the `use` keyword.

```rust
// Bring HashMap into local scope
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new(); // Much shorter!
}
```

You can also re-export items using `pub use`, which makes them part of your module's public API.
```rust
pub mod math {
    pub use self::advanced::calculus;
    pub mod advanced {
        pub fn calculus() {}
    }
}
// Now others can call math::calculus() directly!
```
        "#,
        initial_code: "// Study use paths, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "modules-4-practice",
        title: "4. Practice: `use` paths",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Shorten Paths

### Task:
Add a `use` statement at the top of the file to bring `engine::v8::start` into scope.
Then update `main` to just call `start();`.
        "#,
        initial_code: "mod engine {\n    pub mod v8 {\n        pub fn start() {}\n    }\n}\n\nfn main() {\n    engine::v8::start();\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "add `use engine::v8::start;`",
                    matcher: RuleMatcher::Regex(r#"use\s+engine::v8::start\s*;"#),
                },
                ValidationRule {
                    label: "call `start();`",
                    matcher: RuleMatcher::Regex(r#"start\s*\(\s*\)\s*;"#),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not use the full path in main",
                    matcher: RuleMatcher::Contains("engine::v8::start();"),
                },
            ],
            canonical_solution: Some("use engine::v8::start;\n\nmod engine {\n    pub mod v8 {\n        pub fn start() {}\n    }\n}\n\nfn main() {\n    start();\n}"),
            hints: &[
                "Write `use engine::v8::start;` at the very top.",
                "Inside `main`, replace the old call with just `start();`.",
            ],
        },
        success_message: "Awesome! Your code will be much cleaner now.",
    },
    TutorialModule {
        id: "modules-5-concept",
        title: "5. Concept: File-based Modules",
        module_type: ModuleType::Concept,
        content: r#"
# Modules in Multiple Files

When a module gets too big, you should extract it into its own file.

If you have a `src/main.rs` and you declare `mod network;`, Rust will look for the contents of that module in either:
1. `src/network.rs`
2. `src/network/mod.rs` (older style, but still used for folders with submodules)

**Crucial:** You *must* declare `mod network;` in your `main.rs` or `lib.rs` for Rust to compile the file. It doesn't automatically compile every `.rs` file in the folder!
        "#,
        initial_code: "// Read about file modules, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "modules-6-practice",
        title: "6. Practice: Declare a Module File",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `mod` declaration

### Task:
Imagine this file is `main.rs`, and you have created a new file named `utils.rs`.
Declare the module so Rust compiles it, and bring its public `helper` function into scope.
        "#,
        initial_code: "// Declare the `utils` module\n\n// Bring `utils::helper` into scope\n\nfn main() {\n    helper();\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "declare `mod utils;`",
                    matcher: RuleMatcher::Regex(r#"mod\s+utils\s*;"#),
                },
                ValidationRule {
                    label: "use `utils::helper;`",
                    matcher: RuleMatcher::Regex(r#"use\s+utils::helper\s*;"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("mod utils;\nuse utils::helper;\n\nfn main() {\n    helper();\n}"),
            hints: &[
                "Add `mod utils;` first to tell Rust the file exists.",
                "Then add `use utils::helper;` to shorten the path.",
            ],
        },
        success_message: "Correct! This is how all multi-file Rust projects are structured.",
    },
    TutorialModule {
        id: "modules-7-concept",
        title: "7. Concept: Crates and `Cargo.toml`",
        module_type: ModuleType::Concept,
        content: r#"
# External Crates

A **crate** is a compilation unit in Rust (a library or an executable). The rust package registry is called [crates.io](https://crates.io).

To use an external library, you add it to your `Cargo.toml` file under `[dependencies]`.

```toml
[dependencies]
rand = "0.8.5"
serde = { version = "1.0", features = ["derive"] }
```

Once added, you can bring its contents into scope in your `.rs` files just like any local module:
```rust
use rand::Rng;
```
        "#,
        initial_code: "// Study crates, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "modules-8-practice",
        title: "8. Practice: Using an External Crate",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Use `rand`

### Task:
Assume `rand = "0.8.5"` is in our `Cargo.toml`.
Import the `Rng` trait from the `rand` crate.
        "#,
        initial_code: "// Bring `rand::Rng` into scope\n\nfn main() {\n    // let num = rand::thread_rng().gen_range(1..10);\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "use `rand::Rng`",
                    matcher: RuleMatcher::Regex(r#"use\s+rand::Rng\s*;"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use rand::Rng;\n\nfn main() {\n    // let num = rand::thread_rng().gen_range(1..10);\n}"),
            hints: &[
                "Add `use rand::Rng;` at the top of the file.",
            ],
        },
        success_message: "Excellent! You now know how to organize code and use the Rust ecosystem.",
    },
];
