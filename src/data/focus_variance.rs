use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{RuleMatcher, ValidationRule, ValidationSpec, NormalizeOptions};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "variance_concept_1",
        module_type: ModuleType::Concept,
        title: "Subtyping & Covariance",
        content: "Rust doesn't have object-oriented inheritance, but it does have **subtyping** for lifetimes. 

If `'static` outlives `'a`, then `'static` is a **subtype** of `'a`.

Because `&'a T` is **covariant** over its lifetime, you can pass a `&'static str` to a function expecting `&'a str`. The compiler automatically shrinks the lifetime to match.

```rust
fn print_str<'a>(s: &'a str) {
    println!(\"{}\", s);
}

let static_str: &'static str = \"hello\";
print_str(static_str); // Works! &'static str coerces to &'a str
```",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's see covariance in action.",
    },
    TutorialModule {
        id: "variance_practice_1",
        module_type: ModuleType::Practice,
        title: "Covariant Lifetimes",
        content: "We have a struct `Context<'a>` that holds a reference.

Write a function `shorten_context<'a>(ctx: Context<'static>) -> Context<'a>`.
Because `Context<'a>` is covariant over `'a`, you can simply return `ctx` directly! The compiler will shrink the lifetime for you.

1. Define `shorten_context<'a>(ctx: Context<'static>) -> Context<'a>`.
2. Return `ctx` inside the function.",
        initial_code: "pub struct Context<'a> {
    pub msg: &'a str,
}

// 1. Define shorten_context<'a>(ctx: Context<'static>) -> Context<'a>
// 2. Return ctx",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Define shorten_context",
                    matcher: RuleMatcher::Regex(r#"fn\s+shorten_context\s*<\s*'a\s*>\s*\(\s*ctx\s*:\s*Context\s*<\s*'static\s*>\s*\)\s*->\s*Context\s*<\s*'a\s*>"#),
                },
                ValidationRule {
                    label: "Return ctx",
                    matcher: RuleMatcher::Regex(r#"\{\s*ctx\s*\}"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("pub struct Context<'a> {\n    pub msg: &'a str,\n}\n\npub fn shorten_context<'a>(ctx: Context<'static>) -> Context<'a> {\n    ctx\n}"),
            hints: &[
                "The signature should be: `pub fn shorten_context<'a>(ctx: Context<'static>) -> Context<'a>`",
                "Inside the function, just write `ctx` without a semicolon to return it."
            ],
        },
        success_message: "Great! Because the struct only holds an immutable reference, it is covariant over `'a`.",
    },
    TutorialModule {
        id: "variance_concept_2",
        module_type: ModuleType::Concept,
        title: "Invariance (Mutable References)",
        content: "While `&'a T` is covariant, `&'a mut T` is **invariant** over `T`. 

This means you **cannot** pass a `&mut &'static str` where a `&mut &'a str` is expected.

Why? If it were allowed, you could write a shorter-lived reference into the mutable location, overwriting the `'static` reference and causing undefined behavior when the original owner tries to read the supposedly `'static` data!

```rust
fn overwrite_with_short<'a>(s: &mut &'a str, short: &'a str) {
    *s = short;
}

let mut static_str: &'static str = \"static\";
let short_str = String::from(\"short\");

// compile error! cannot borrow `static_str` as mutable
// overwrite_with_short(&mut static_str, &short_str); 
```",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's see how wrapping data affects variance.",
    },
    TutorialModule {
        id: "variance_practice_2",
        module_type: ModuleType::Practice,
        title: "Invariance in Structs",
        content: "If a struct contains a mutable reference or interior mutability (like `Cell` or `RefCell`), it becomes **invariant**.

Identify which struct is invariant by modifying the code. Add `PhantomData<Cell<&'a ()>>` to `InvariantStruct` to explicitly mark it as invariant over `'a`.

1. Import `std::marker::PhantomData` and `std::cell::Cell`.
2. Add a `_marker: PhantomData<Cell<&'a ()>>` field to `InvariantStruct`.",
        initial_code: "pub struct CovariantStruct<'a> {
    pub data: &'a str,
}

pub struct InvariantStruct<'a> {
    pub data: &'a str,
    // Add _marker field here
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Add _marker field",
                    matcher: RuleMatcher::Regex(r#"_marker\s*:\s*PhantomData\s*<\s*Cell\s*<\s*&'a\s*\(\)\s*>\s*>"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::marker::PhantomData;\nuse std::cell::Cell;\n\npub struct CovariantStruct<'a> {\n    pub data: &'a str,\n}\n\npub struct InvariantStruct<'a> {\n    pub data: &'a str,\n    _marker: PhantomData<Cell<&'a ()>>,\n}"),
            hints: &[
                "Add `_marker: PhantomData<Cell<&'a ()>>` to `InvariantStruct`.",
                "Make sure to import `PhantomData` and `Cell`."
            ],
        },
        success_message: "Awesome! `Cell` requires invariance because of interior mutability, which automatically makes `InvariantStruct` invariant over `'a`.",
    },
];
