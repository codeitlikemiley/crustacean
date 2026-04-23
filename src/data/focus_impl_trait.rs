use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fimpl-1-concept",
        title: "1. `impl Trait` in Arguments",
        module_type: ModuleType::Concept,
        content: r#"
# APIT: Argument Position Impl Trait

`impl Trait` in argument position is syntactic sugar for a generic:

```rust
fn print_it(item: impl Display) { ... }
// is equivalent to:
fn print_it<T: Display>(item: T) { ... }
```

The caller decides the concrete type. Each call can use a different type.
        "#,
        initial_code: "// Study APIT, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fimpl-2-concept",
        title: "2. `impl Trait` in Return",
        module_type: ModuleType::Concept,
        content: r#"
# RPIT: Return Position Impl Trait

`impl Trait` in return position means "I return some type that implements this trait, but I'm not telling you which":

```rust
fn make_iter() -> impl Iterator<Item = i32> {
    (0..10).filter(|x| x % 2 == 0)
}
```

The function *must* return exactly one concrete type (no branching between different types). The caller sees only the trait, not the concrete type.
        "#,
        initial_code: "// Study RPIT, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fimpl-3-practice",
        title: "3. Practice: Return impl Trait",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Return an Iterator

### Task:
Write a function `evens` that returns `impl Iterator<Item = i32>`. It should filter a range `0..n` to keep only even numbers.
        "#,
        initial_code: "// Write fn evens(n: i32) -> impl Iterator<Item = i32>\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "return impl Iterator",
                    matcher: RuleMatcher::Contains("-> impl Iterator<Item = i32>"),
                },
                ValidationRule {
                    label: "use filter",
                    matcher: RuleMatcher::Contains(".filter("),
                },
                ValidationRule {
                    label: "check even",
                    matcher: RuleMatcher::Contains("% 2 == 0"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn evens(n: i32) -> impl Iterator<Item = i32> {\n    (0..n).filter(|x| x % 2 == 0)\n}"),
            hints: &[
                "Return `(0..n).filter(|x| x % 2 == 0)`",
                "The return type hides the complex filter type behind `impl Iterator`.",
            ],
        },
        success_message: "The caller sees a simple Iterator without knowing the complex chain type!",
    },
];
