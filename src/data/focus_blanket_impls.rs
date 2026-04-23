use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fblanket-1-concept",
        title: "1. Blanket Implementations",
        module_type: ModuleType::Concept,
        content: r#"
# What are Blanket Impls?

A blanket implementation provides a trait for *all* types that satisfy a bound:

```rust
// From the standard library:
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}
```

This means: **every type that implements `Display` automatically gets `to_string()`** for free. You never implement `ToString` manually.
        "#,
        initial_code: "// Study blanket impls, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fblanket-2-practice",
        title: "2. Practice: Write a Blanket Impl",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `Printable`

### Task:
Write a blanket impl: for any type `T` that implements `std::fmt::Debug`, implement `Printable` by calling `format!("{:?}", self)` in `to_debug_string`.
        "#,
        initial_code: "trait Printable {\n    fn to_debug_string(&self) -> String;\n}\n\n// Write blanket impl here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "impl<T: Debug> or where T: Debug",
                    matcher: RuleMatcher::AnyContains(&["impl<T: std::fmt::Debug>", "impl<T: Debug>", "where T: Debug", "where T: std::fmt::Debug"]),
                },
                ValidationRule {
                    label: "Printable for T",
                    matcher: RuleMatcher::Contains("Printable for T"),
                },
                ValidationRule {
                    label: "use format with Debug",
                    matcher: RuleMatcher::Contains("{:?}"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl<T: std::fmt::Debug> Printable for T {\n    fn to_debug_string(&self) -> String {\n        format!(\"{:?}\", self)\n    }\n}"),
            hints: &[
                "Start: `impl<T: std::fmt::Debug> Printable for T {`",
                "Body: `format!(\"{:?}\", self)`",
            ],
        },
        success_message: "Every Debug type now gets Printable for free!",
    },
    TutorialModule {
        id: "fblanket-3-concept",
        title: "3. Coherence Limitations",
        module_type: ModuleType::Concept,
        content: r#"
# Why You Can't Always Blanket

Blanket impls interact with the **orphan rule**. Once a blanket impl like `impl<T: Display> MyTrait for T` exists, no one else can impl `MyTrait` for their own `Display` type — the blanket already covers it.

This is why adding blanket impls is a **breaking change** in library APIs.

```rust
// If a library adds this:
impl<T: Display> Loggable for T { ... }

// Then downstream code like this breaks:
impl Loggable for MyType { ... }
// ERROR: conflicting implementation!
```
        "#,
        initial_code: "// Study coherence, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
];
