use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fdisplay-1-concept",
        title: "1. Display vs Debug",
        module_type: ModuleType::Concept,
        content: r#"
# Two Formatting Traits

- **`Display`** (`{}`) — user-facing output. Must be manually implemented.
- **`Debug`** (`{:?}`) — developer-facing output. Can be `#[derive(Debug)]`.

```rust
use std::fmt;

struct Point { x: f64, y: f64 }

// Debug: derived automatically
#[derive(Debug)] // gives: Point { x: 1.0, y: 2.0 }

// Display: you control the format
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```
        "#,
        initial_code: "// Study Display vs Debug, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fdisplay-2-practice",
        title: "2. Practice: Implement Display",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Custom Display

### Task:
Implement `Display` for `Color`. The `fmt` method should write `rgb(R, G, B)` using the struct's fields.
        "#,
        initial_code: "use std::fmt;\n\nstruct Color { r: u8, g: u8, b: u8 }\n\n// Implement Display for Color\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "impl Display for Color",
                    matcher: RuleMatcher::OrderedContains(&["impl", "fmt::Display", "for Color"]),
                },
                ValidationRule {
                    label: "fn fmt method",
                    matcher: RuleMatcher::Contains("fn fmt("),
                },
                ValidationRule {
                    label: "use write! macro",
                    matcher: RuleMatcher::Contains("write!("),
                },
                ValidationRule {
                    label: "output rgb format",
                    matcher: RuleMatcher::Contains("rgb("),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl fmt::Display for Color {\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n        write!(f, \"rgb({}, {}, {})\", self.r, self.g, self.b)\n    }\n}"),
            hints: &[
                "Return `write!(f, \"rgb({}, {}, {})\", self.r, self.g, self.b)`",
            ],
        },
        success_message: "Now `println!(\"{}\", color)` outputs beautiful formatted text!",
    },
];
