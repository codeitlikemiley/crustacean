use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "felision-1-concept",
        title: "1. The Three Elision Rules",
        module_type: ModuleType::Concept,
        content: r#"
# Lifetime Elision

The compiler applies 3 rules to infer lifetimes so you don't have to write them everywhere:

**Rule 1:** Each reference parameter gets its own lifetime.
```rust
fn foo(x: &i32, y: &i32) → fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
```

**Rule 2:** If there's exactly one input lifetime, it's assigned to all outputs.
```rust
fn foo(x: &i32) -> &i32 → fn foo<'a>(x: &'a i32) -> &'a i32
```

**Rule 3:** If one parameter is `&self` or `&mut self`, its lifetime is assigned to all outputs.
```rust
fn foo(&self, x: &i32) -> &i32 → fn foo<'a, 'b>(&'a self, x: &'b i32) -> &'a i32
```
        "#,
        initial_code: "// Study elision rules, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "felision-2-concept",
        title: "2. When Elision Fails",
        module_type: ModuleType::Concept,
        content: r#"
# Manual Annotation Required

When rules don't determine all lifetimes, you must annotate:

```rust
// Two input references, one output — which input's lifetime?
fn longest(x: &str, y: &str) -> &str { ... }
// ERROR: can't infer lifetime

// Fix: tell Rust both inputs AND the output share 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
```
        "#,
        initial_code: "// Study annotation requirements, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "felision-3-practice",
        title: "3. Practice: Predict the Lifetime",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: When to Annotate

### Task:
This function takes two references and returns one. Add the lifetime annotation `'a` so it compiles. Both inputs and the output should share `'a`.
        "#,
        initial_code: "fn pick_first(a: &str, b: &str) -> &str {\n    a\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "declare lifetime <'a>",
                    matcher: RuleMatcher::Contains("<'a>"),
                },
                ValidationRule {
                    label: "annotate first param &'a str",
                    matcher: RuleMatcher::Regex(r#"a\s*:\s*&'a\s+str"#),
                },
                ValidationRule {
                    label: "annotate return &'a str",
                    matcher: RuleMatcher::Regex(r#"->\s*&'a\s+str"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn pick_first<'a>(a: &'a str, b: &'a str) -> &'a str {\n    a\n}"),
            hints: &[
                "Add `<'a>` after the function name.",
                "Annotate all references with `&'a str`.",
            ],
        },
        success_message: "You understand when the compiler needs your help!",
    },
];
