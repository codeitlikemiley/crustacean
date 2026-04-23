use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fnever-1-concept",
        title: "1. The Never Type `!`",
        module_type: ModuleType::Concept,
        content: r#"
# Diverging Functions

The never type `!` represents computations that never complete. Functions returning `!` never return to the caller.

```rust
fn forever() -> ! {
    loop {
        // This loop runs forever
    }
}

fn crash() -> ! {
    panic!("Fatal error");
}
```

`!` can be coerced to any type, which is why `panic!` works inside `match` arms that expect specific types.
        "#,
        initial_code: "// Study the never type, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fnever-2-practice",
        title: "2. Practice: Diverging Function",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Return `!`

### Task:
Write a function `exit_program` that returns `!` and calls `std::process::exit(1)`.
        "#,
        initial_code: "// Write fn exit_program() -> !\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "return `-> !`",
                    matcher: RuleMatcher::Contains("-> !"),
                },
                ValidationRule {
                    label: "call exit",
                    matcher: RuleMatcher::Contains("std::process::exit("),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn exit_program() -> ! {\n    std::process::exit(1)\n}"),
            hints: &[
                "Return type is `-> !`",
                "Body: `std::process::exit(1)`",
            ],
        },
        success_message: "This function provably never returns!",
    },
];
