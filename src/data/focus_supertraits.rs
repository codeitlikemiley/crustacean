use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fsuper-1-concept",
        title: "1. Supertraits",
        module_type: ModuleType::Concept,
        content: r#"
# Requiring Other Traits

A supertrait says "to implement B, you must first implement A":

```rust
trait Animal: fmt::Display {
    fn name(&self) -> &str;
}
// Any type implementing Animal MUST also implement Display.
```

This is syntactic sugar for `trait Animal where Self: fmt::Display`.
        "#,
        initial_code: "// Study supertraits, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fsuper-2-practice",
        title: "2. Practice: Define a Supertrait",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `Printable: Display`

### Task:
Define a trait `Printable` that requires `std::fmt::Display` as a supertrait. Add a method `print(&self)` with a default implementation that calls `println!("{}", self)`.
        "#,
        initial_code: "use std::fmt;\n\n// Define trait Printable\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "trait with supertrait bound",
                    matcher: RuleMatcher::AnyContains(&["trait Printable: fmt::Display", "trait Printable: std::fmt::Display"]),
                },
                ValidationRule {
                    label: "fn print with default body",
                    matcher: RuleMatcher::Contains("fn print(&self)"),
                },
                ValidationRule {
                    label: "println in body",
                    matcher: RuleMatcher::Contains("println!"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("trait Printable: fmt::Display {\n    fn print(&self) {\n        println!(\"{}\", self);\n    }\n}"),
            hints: &[
                "Use the colon syntax: `trait Printable: fmt::Display`",
                "Default methods have a body: `fn print(&self) { println!(\"{}\", self); }`",
            ],
        },
        success_message: "Any Printable type is guaranteed to also be Display!",
    },
];
