use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fops-1-concept",
        title: "1. Operator Overloading",
        module_type: ModuleType::Concept,
        content: r#"
# Custom Operators

Operators like `+`, `-`, `*`, `==` are just syntax sugar for trait methods!

```rust
use std::ops::Add;

struct Vec2 { x: f64, y: f64 }

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

let a = Vec2 { x: 1.0, y: 2.0 };
let b = Vec2 { x: 3.0, y: 4.0 };
let c = a + b; // Uses our Add impl!
```
        "#,
        initial_code: "// Study operator overloading, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fops-2-practice",
        title: "2. Practice: Implement Add",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `Money + Money`

### Task:
Implement `std::ops::Add` for `Money`. Adding two Money values should add their `cents` fields.
        "#,
        initial_code: "use std::ops::Add;\n\nstruct Money { cents: i64 }\n\n// Implement Add for Money\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "impl Add for Money",
                    matcher: RuleMatcher::OrderedContains(&["impl", "Add", "for Money"]),
                },
                ValidationRule {
                    label: "type Output = Money",
                    matcher: RuleMatcher::Contains("type Output = Money"),
                },
                ValidationRule {
                    label: "fn add",
                    matcher: RuleMatcher::Contains("fn add("),
                },
                ValidationRule {
                    label: "add cents",
                    matcher: RuleMatcher::Contains("self.cents + rhs.cents"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl Add for Money {\n    type Output = Money;\n    fn add(self, rhs: Money) -> Money {\n        Money { cents: self.cents + rhs.cents }\n    }\n}"),
            hints: &[
                "Set `type Output = Money;`",
                "Return `Money { cents: self.cents + rhs.cents }`",
            ],
        },
        success_message: "You can now use `wallet1 + wallet2`!",
    },
    TutorialModule {
        id: "fops-3-practice",
        title: "3. Practice: Implement PartialEq",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Custom Equality

### Task:
Implement `PartialEq` for `Email`. Two emails are equal if their `address` fields are equal (case-insensitive — use `.to_lowercase()`).
        "#,
        initial_code: "struct Email { address: String }\n\n// Implement PartialEq for Email\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "impl PartialEq for Email",
                    matcher: RuleMatcher::OrderedContains(&["impl", "PartialEq", "for Email"]),
                },
                ValidationRule {
                    label: "fn eq",
                    matcher: RuleMatcher::Contains("fn eq("),
                },
                ValidationRule {
                    label: "to_lowercase comparison",
                    matcher: RuleMatcher::Contains("to_lowercase()"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl PartialEq for Email {\n    fn eq(&self, other: &Self) -> bool {\n        self.address.to_lowercase() == other.address.to_lowercase()\n    }\n}"),
            hints: &[
                "Compare `self.address.to_lowercase() == other.address.to_lowercase()`",
            ],
        },
        success_message: "Case-insensitive email comparison!",
    },
];
