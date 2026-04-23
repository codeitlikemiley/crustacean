use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fpin-1-concept",
        title: "1. Why Pinning Exists",
        module_type: ModuleType::Concept,
        content: r#"
# Self-Referential Structs

Some types contain internal pointers to their own data. If the struct moves in memory, those pointers become dangling!

`Pin<P>` guarantees that the pointed-to value **will not move**. This is critical for `async` blocks which become self-referential state machines.

```rust
use std::pin::Pin;

// A pinned reference: the value behind it won't move
let mut val = 42;
let pinned: Pin<&mut i32> = Pin::new(&mut val);
```
        "#,
        initial_code: "// Study Pin, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fpin-2-concept",
        title: "2. Unpin Marker Trait",
        module_type: ModuleType::Concept,
        content: r#"
# The `Unpin` Auto-Trait

Most types are `Unpin` — they are safe to move even after being pinned. `Pin` only provides guarantees for `!Unpin` types.

- `i32`, `String`, `Vec<T>`: all `Unpin` (can be freely moved).
- `async` blocks: `!Unpin` (self-referential, must stay put).

```rust
// For Unpin types, Pin is a no-op:
let mut x = 5;
let pinned = Pin::new(&mut x); // Allowed because i32: Unpin

// For !Unpin types, you need Box::pin:
let fut = async { 42 };
let pinned = Box::pin(fut); // Heap-allocates and pins
```
        "#,
        initial_code: "// Study Unpin, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fpin-3-concept",
        title: "3. Pin in Async",
        module_type: ModuleType::Concept,
        content: r#"
# Why async Needs Pin

When you write `async { ... }`, the compiler generates a state machine struct. Between `.await` points, local variables are stored as fields. If a variable references another field, the struct is self-referential.

`Future::poll` requires `Pin<&mut Self>` to ensure the future doesn't move between polls:

```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

This is why you see `Box::pin()` when manually working with futures.
        "#,
        initial_code: "// Study Pin in async, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fpin-4-practice",
        title: "4. Practice: Box::pin",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Pin a Future

### Task:
Use `Box::pin()` to pin an async block that returns `42`. Assign it to `let pinned`.
        "#,
        initial_code: "fn main() {\n    // let pinned = Box::pin(async { ... });\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `let pinned`",
                    matcher: RuleMatcher::Regex(r#"let\s+pinned"#),
                },
                ValidationRule {
                    label: "use Box::pin",
                    matcher: RuleMatcher::Contains("Box::pin("),
                },
                ValidationRule {
                    label: "async block",
                    matcher: RuleMatcher::Contains("async"),
                },
                ValidationRule {
                    label: "return 42",
                    matcher: RuleMatcher::Contains("42"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("let pinned = Box::pin(async { 42 });"),
            hints: &[
                "Syntax: `let pinned = Box::pin(async { 42 });`",
            ],
        },
        success_message: "The future is pinned to the heap and safe to poll!",
    },
];
