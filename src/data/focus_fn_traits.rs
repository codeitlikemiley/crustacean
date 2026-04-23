use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "ffntr-1-concept",
        title: "1. Fn, FnMut, FnOnce",
        module_type: ModuleType::Concept,
        content: r#"
# The Three Closure Traits

Every closure implements one or more of these traits based on how it captures variables:

- **`FnOnce`**: Takes ownership of captured values. Can only be called once.
- **`FnMut`**: Borrows mutably. Can be called multiple times.
- **`Fn`**: Borrows immutably. Can be called many times concurrently.

Hierarchy: `Fn` ⊂ `FnMut` ⊂ `FnOnce`. Every `Fn` is also `FnMut`, every `FnMut` is also `FnOnce`.

```rust
let name = String::from("Alice");
let consume = move || drop(name);  // FnOnce
let mut count = 0;
let mut inc = || count += 1;       // FnMut
let greet = || println!("Hi");     // Fn
```
        "#,
        initial_code: "// Study closure traits, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "ffntr-2-practice",
        title: "2. Practice: Accept Fn",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Function accepting a closure

### Task:
Write a function `apply` that takes a closure `f: impl Fn(i32) -> i32` and an `x: i32`, and returns `f(x)`.
        "#,
        initial_code: "// Write `fn apply` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn apply`",
                    matcher: RuleMatcher::Regex(r#"fn\s+apply"#),
                },
                ValidationRule {
                    label: "accept `impl Fn(i32) -> i32`",
                    matcher: RuleMatcher::Contains("impl Fn(i32) -> i32"),
                },
                ValidationRule {
                    label: "call `f(x)`",
                    matcher: RuleMatcher::Contains("f(x)"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 {\n    f(x)\n}"),
            hints: &[
                "Signature: `fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32`",
                "Body: `f(x)`",
            ],
        },
        success_message: "Your function accepts any compatible closure!",
    },
    TutorialModule {
        id: "ffntr-3-concept",
        title: "3. Boxing Closures",
        module_type: ModuleType::Concept,
        content: r#"
# Returning Closures

Each closure has a unique, anonymous type. You can't write its type name. To return a closure from a function:

**Option 1**: `impl Fn` (when returning one specific closure)
```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}
```

**Option 2**: `Box<dyn Fn>` (when returning different closures)
```rust
fn pick(add: bool) -> Box<dyn Fn(i32) -> i32> {
    if add { Box::new(|x| x + 1) }
    else { Box::new(|x| x * 2) }
}
```
        "#,
        initial_code: "// Study returning closures, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "ffntr-4-practice",
        title: "4. Practice: Return a Closure",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `make_multiplier`

### Task:
Write `make_multiplier(factor: i32)` that returns `impl Fn(i32) -> i32`. The returned closure should multiply its argument by `factor`.
        "#,
        initial_code: "// Write `fn make_multiplier` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "return `impl Fn(i32) -> i32`",
                    matcher: RuleMatcher::Contains("-> impl Fn(i32) -> i32"),
                },
                ValidationRule {
                    label: "use `move`",
                    matcher: RuleMatcher::Contains("move"),
                },
                ValidationRule {
                    label: "multiply by factor",
                    matcher: RuleMatcher::AnyContains(&["x * factor", "factor * x"]),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {\n    move |x| x * factor\n}"),
            hints: &[
                "Signature: `fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32`",
                "Body: `move |x| x * factor`",
            ],
        },
        success_message: "You returned a closure that captures its environment!",
    },
];
