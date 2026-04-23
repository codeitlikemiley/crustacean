use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fstate-1-concept",
        title: "1. Enum State Machines",
        module_type: ModuleType::Concept,
        content: r#"
# State Machines with Enums

Use an enum to represent all possible states. The `match` forces you to handle every case.

```rust
enum ConnectionState {
    Disconnected,
    Connecting { attempt: u32 },
    Connected { session_id: String },
    Error { message: String },
}
```

Each variant can carry different data. Transitions are just functions that return the next state.
        "#,
        initial_code: "// Study enum state machines, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fstate-2-practice",
        title: "2. Practice: Define States",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Traffic Light

### Task:
Define an enum `Light` with variants `Red`, `Yellow`, and `Green`.
        "#,
        initial_code: "// Define enum Light\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define enum Light",
                    matcher: RuleMatcher::Contains("enum Light"),
                },
                ValidationRule {
                    label: "variant Red",
                    matcher: RuleMatcher::Contains("Red"),
                },
                ValidationRule {
                    label: "variant Yellow",
                    matcher: RuleMatcher::Contains("Yellow"),
                },
                ValidationRule {
                    label: "variant Green",
                    matcher: RuleMatcher::Contains("Green"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("enum Light {\n    Red,\n    Yellow,\n    Green,\n}"),
            hints: &[
                "Simple unit variants: `Red, Yellow, Green`",
            ],
        },
        success_message: "States defined!",
    },
    TutorialModule {
        id: "fstate-3-practice",
        title: "3. Practice: Transition Function",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `next()` Transition

### Task:
Add a `next(self) -> Light` method that transitions: Red→Green, Green→Yellow, Yellow→Red. Use `match self`.
        "#,
        initial_code: "enum Light { Red, Yellow, Green }\n\nimpl Light {\n    // Add fn next(self) -> Light\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "fn next(self) -> Light",
                    matcher: RuleMatcher::Contains("fn next(self) -> Light"),
                },
                ValidationRule {
                    label: "match self",
                    matcher: RuleMatcher::Contains("match self"),
                },
                ValidationRule {
                    label: "Red => Green",
                    matcher: RuleMatcher::OrderedContains(&["Red", "Green"]),
                },
                ValidationRule {
                    label: "Yellow => Red",
                    matcher: RuleMatcher::OrderedContains(&["Yellow", "Red"]),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl Light {\n    fn next(self) -> Light {\n        match self {\n            Light::Red => Light::Green,\n            Light::Green => Light::Yellow,\n            Light::Yellow => Light::Red,\n        }\n    }\n}"),
            hints: &[
                "Match each variant and return the next state.",
                "The compiler ensures you handle all cases!",
            ],
        },
        success_message: "Exhaustive state transitions — no invalid states possible!",
    },
];
