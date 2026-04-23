use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "forphan-1-concept",
        title: "1. The Orphan Rule",
        module_type: ModuleType::Concept,
        content: r#"
# Coherence & the Orphan Rule

To impl a trait, **at least one of** the trait or the type must be defined in your crate. This prevents two crates from providing conflicting impls.

```rust
// OK: Your trait, foreign type
impl MyTrait for Vec<i32> { ... }

// OK: Foreign trait, your type
impl Display for MyStruct { ... }

// ERROR: Both foreign
impl Display for Vec<i32> { ... }
```
        "#,
        initial_code: "// Study the orphan rule, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "forphan-2-concept",
        title: "2. Workarounds",
        module_type: ModuleType::Concept,
        content: r#"
# Escape Hatches

1. **Newtype**: Wrap the foreign type → now it's yours.
2. **Extension trait**: Define your own trait with the methods you want, implement it for the foreign type.
3. **Blanket impl**: `impl<T: ForeignTrait> MyTrait for T` — your trait, generic over any type meeting the bound.

```rust
// Extension trait pattern:
trait VecExt {
    fn sum_all(&self) -> i32;
}

impl VecExt for Vec<i32> {
    fn sum_all(&self) -> i32 {
        self.iter().sum()
    }
}
```
        "#,
        initial_code: "// Study workarounds, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "forphan-3-practice",
        title: "3. Practice: Extension Trait",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Add Method to Vec

### Task:
Define a trait `VecExt` with a method `first_or_default(&self) -> i32` and implement it for `Vec<i32>`. Return the first element or `0` if empty.
        "#,
        initial_code: "// Define trait VecExt and implement for Vec<i32>\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define trait VecExt",
                    matcher: RuleMatcher::Contains("trait VecExt"),
                },
                ValidationRule {
                    label: "method first_or_default",
                    matcher: RuleMatcher::Contains("fn first_or_default"),
                },
                ValidationRule {
                    label: "impl VecExt for Vec<i32>",
                    matcher: RuleMatcher::OrderedContains(&["impl", "VecExt", "for Vec<i32>"]),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("trait VecExt {\n    fn first_or_default(&self) -> i32;\n}\n\nimpl VecExt for Vec<i32> {\n    fn first_or_default(&self) -> i32 {\n        self.first().copied().unwrap_or(0)\n    }\n}"),
            hints: &[
                "Define the trait first, then implement it.",
                "Use `.first().copied().unwrap_or(0)` in the body.",
            ],
        },
        success_message: "You added methods to a stdlib type!",
    },
];
