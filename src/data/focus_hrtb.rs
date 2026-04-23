use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fhrtb-1-concept",
        title: "1. The Problem",
        module_type: ModuleType::Concept,
        content: r#"
# Why HRTBs Exist

Sometimes you need a closure that works with *any* lifetime, not just one specific one.

```rust
// This doesn't work:
fn apply_to_ref<F: Fn(&str)>(f: F) { ... }
// What lifetime does &str have? The compiler can't pick one.

// This works — it means "F works for ANY lifetime 'a":
fn apply_to_ref<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{ ... }
```

`for<'a>` is called a **Higher-Rank Trait Bound** — it quantifies over all possible lifetimes.
        "#,
        initial_code: "// Study HRTBs, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fhrtb-2-concept",
        title: "2. Implicit HRTBs",
        module_type: ModuleType::Concept,
        content: r#"
# You've Been Using HRTBs Already!

When you write `Fn(&str)` in a bound, the compiler *implicitly* desugars it to `for<'a> Fn(&'a str)`. You only need the explicit form in complex situations.

```rust
// These are equivalent:
fn a(f: impl Fn(&str)) { ... }
fn b<F: for<'a> Fn(&'a str)>(f: F) { ... }
```

You need explicit `for<'a>` when:
- You're writing trait bounds with multiple references at different lifetimes.
- You're implementing traits for types that contain closures.
        "#,
        initial_code: "// Study implicit HRTBs, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fhrtb-3-practice",
        title: "3. Practice: Write a HRTB",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Explicit `for<'a>`

### Task:
Write a function `call_with_str` that takes a closure `f` bounded by `for<'a> Fn(&'a str) -> &'a str` and calls it with `"hello"`.
        "#,
        initial_code: "// Write `fn call_with_str` with a HRTB\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn call_with_str`",
                    matcher: RuleMatcher::Regex(r#"fn\s+call_with_str"#),
                },
                ValidationRule {
                    label: "use `for<'a>`",
                    matcher: RuleMatcher::Contains("for<'a>"),
                },
                ValidationRule {
                    label: "bound on Fn",
                    matcher: RuleMatcher::Contains("Fn(&'a str)"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn call_with_str<F>(f: F) -> &str\nwhere\n    F: for<'a> Fn(&'a str) -> &'a str,\n{\n    f(\"hello\")\n}"),
            hints: &[
                "Use a `where` clause: `where F: for<'a> Fn(&'a str) -> &'a str`",
                "Call `f(\"hello\")` in the body.",
            ],
        },
        success_message: "You're writing universally quantified lifetime bounds!",
    },
];
