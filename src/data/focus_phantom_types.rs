use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fphantom-1-concept",
        title: "1. PhantomData",
        module_type: ModuleType::Concept,
        content: r#"
# Zero-Cost Type Tags

`PhantomData<T>` tells the compiler your type logically "uses" `T` even though it stores nothing. It occupies zero bytes.

```rust
use std::marker::PhantomData;

struct Meters;
struct Feet;

struct Distance<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

// Distance<Meters> and Distance<Feet> are different types!
```
        "#,
        initial_code: "// Study PhantomData, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fphantom-2-practice",
        title: "2. Practice: Tagged Container",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Type-Safe IDs

### Task:
Define a struct `Id<Entity>` with fields `value: u64` and `_marker: PhantomData<Entity>`.
        "#,
        initial_code: "use std::marker::PhantomData;\n\n// Define struct Id<Entity>\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define struct Id<Entity>",
                    matcher: RuleMatcher::Contains("struct Id<Entity>"),
                },
                ValidationRule {
                    label: "field value: u64",
                    matcher: RuleMatcher::Contains("value: u64"),
                },
                ValidationRule {
                    label: "PhantomData<Entity>",
                    matcher: RuleMatcher::Contains("PhantomData<Entity>"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("struct Id<Entity> {\n    value: u64,\n    _marker: PhantomData<Entity>,\n}"),
            hints: &[
                "Use `PhantomData<Entity>` for the marker field.",
                "The underscore prefix (`_marker`) avoids unused field warnings.",
            ],
        },
        success_message: "Id<User> and Id<Order> are now incompatible types!",
    },
];
