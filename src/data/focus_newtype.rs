use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fnt-1-concept",
        title: "1. The Newtype Pattern",
        module_type: ModuleType::Concept,
        content: r#"
# Newtype: Type Safety for Free

The newtype pattern wraps a primitive in a single-field tuple struct to create a *distinct* type.

```rust
struct Meters(f64);
struct Seconds(f64);

fn speed(d: Meters, t: Seconds) -> f64 { d.0 / t.0 }
// speed(Seconds(10.0), Meters(5.0)); // Compile error!
```

Zero-cost: in release builds, `Meters(5.0)` compiles to just `5.0`.
        "#,
        initial_code: "// Study the newtype pattern, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fnt-2-practice",
        title: "2. Practice: Define Newtypes",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `UserId` and `OrderId`

### Task:
Define two newtype structs: `UserId(u64)` and `OrderId(u64)`.
        "#,
        initial_code: "// Define `UserId` and `OrderId`\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `struct UserId(u64)`",
                    matcher: RuleMatcher::Contains("struct UserId(u64)"),
                },
                ValidationRule {
                    label: "define `struct OrderId(u64)`",
                    matcher: RuleMatcher::Contains("struct OrderId(u64)"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("struct UserId(u64);\nstruct OrderId(u64);"),
            hints: &[
                "A newtype looks like `struct TypeName(InnerType);`",
                "Don't forget the semicolons!",
            ],
        },
        success_message: "IDs can never be accidentally swapped now!",
    },
    TutorialModule {
        id: "fnt-3-concept",
        title: "3. Orphan Rule Workaround",
        module_type: ModuleType::Concept,
        content: r#"
# Bypassing the Orphan Rule

You can't implement a foreign trait on a foreign type. The newtype wraps the foreign type, making it *yours*:

```rust
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```
        "#,
        initial_code: "// Study orphan rule workaround, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fnt-4-practice",
        title: "4. Practice: Display via Newtype",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Display for a Vec

### Task:
Define `struct Words(Vec<String>);` and implement `std::fmt::Display` for it. The `fmt` method should write `self.0.join(", ")`.
        "#,
        initial_code: "use std::fmt;\n\n// Define `Words` and implement Display\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `struct Words`",
                    matcher: RuleMatcher::Contains("struct Words(Vec<String>)"),
                },
                ValidationRule {
                    label: "impl Display for Words",
                    matcher: RuleMatcher::OrderedContains(&["impl", "fmt::Display", "for Words"]),
                },
                ValidationRule {
                    label: "join the elements",
                    matcher: RuleMatcher::Contains("self.0.join("),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("struct Words(Vec<String>);\n\nimpl fmt::Display for Words {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n        write!(f, \"{}\", self.0.join(\", \"))\n    }\n}"),
            hints: &[
                "First define `struct Words(Vec<String>);`",
                "Then implement `fmt::Display` with a `fn fmt` method.",
            ],
        },
        success_message: "Orphan rule defeated!",
    },
];
