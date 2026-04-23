use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "ftypestate-1-concept",
        title: "1. The Type State Pattern",
        module_type: ModuleType::Concept,
        content: r#"
# Encoding State in the Type System

Instead of runtime checks (`if state == "locked"`), encode states as *types*. Invalid transitions become **compile errors**.

```rust
struct Locked;
struct Unlocked;

struct Door<State> {
    _state: std::marker::PhantomData<State>,
}

impl Door<Locked> {
    fn unlock(self) -> Door<Unlocked> { Door { _state: PhantomData } }
}

impl Door<Unlocked> {
    fn lock(self) -> Door<Locked> { Door { _state: PhantomData } }
    fn open(&self) { println!("Opening!"); }
}

// door.open() on a locked door? Compile error!
```
        "#,
        initial_code: "// Study type state, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "ftypestate-2-practice",
        title: "2. Practice: State Structs",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Define States

### Task:
Define two empty state marker structs: `Draft` and `Published`. Then define `struct Post<S> { title: String, _state: std::marker::PhantomData<S> }`.
        "#,
        initial_code: "use std::marker::PhantomData;\n\n// Define Draft, Published, and Post<S>\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `struct Draft`",
                    matcher: RuleMatcher::Contains("struct Draft;"),
                },
                ValidationRule {
                    label: "define `struct Published`",
                    matcher: RuleMatcher::Contains("struct Published;"),
                },
                ValidationRule {
                    label: "define `struct Post<S>`",
                    matcher: RuleMatcher::Contains("struct Post<S>"),
                },
                ValidationRule {
                    label: "use PhantomData<S>",
                    matcher: RuleMatcher::Contains("PhantomData<S>"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("struct Draft;\nstruct Published;\n\nstruct Post<S> {\n    title: String,\n    _state: PhantomData<S>,\n}"),
            hints: &[
                "State markers are just empty structs: `struct Draft;`",
                "Post is generic over S: `struct Post<S> { ... }`",
            ],
        },
        success_message: "States defined! Now add methods per-state.",
    },
    TutorialModule {
        id: "ftypestate-3-practice",
        title: "3. Practice: Transition Method",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `publish()` transition

### Task:
Add a `publish(self) -> Post<Published>` method to `impl Post<Draft>`. It should consume the draft and return a published post.
        "#,
        initial_code: "use std::marker::PhantomData;\nstruct Draft;\nstruct Published;\nstruct Post<S> { title: String, _state: PhantomData<S> }\n\nimpl Post<Draft> {\n    // Add `fn publish(self) -> Post<Published>`\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "fn publish(self)",
                    matcher: RuleMatcher::Contains("fn publish(self)"),
                },
                ValidationRule {
                    label: "return Post<Published>",
                    matcher: RuleMatcher::Contains("-> Post<Published>"),
                },
                ValidationRule {
                    label: "construct Post with PhantomData",
                    matcher: RuleMatcher::Contains("PhantomData"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl Post<Draft> {\n    fn publish(self) -> Post<Published> {\n        Post { title: self.title, _state: PhantomData }\n    }\n}"),
            hints: &[
                "Consume self and return a new `Post<Published>`.",
                "Transfer `self.title` to the new post.",
            ],
        },
        success_message: "Invalid state transitions are now impossible!",
    },
];
