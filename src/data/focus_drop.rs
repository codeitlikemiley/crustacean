use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fdrop-1-concept",
        title: "1. The Drop Trait",
        module_type: ModuleType::Concept,
        content: r#"
# Custom Destructors

When a value goes out of scope, Rust calls `Drop::drop()` automatically. This is the RAII pattern.

```rust
struct FileHandle { name: String }

impl Drop for FileHandle {
    fn drop(&mut self) {
        println!("Closing {}", self.name);
    }
}
// FileHandle is "closed" automatically when it leaves scope.
```

You cannot call `drop()` manually on a value. Use `std::mem::drop(value)` instead, which moves the value and drops it early.
        "#,
        initial_code: "// Study Drop, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fdrop-2-practice",
        title: "2. Practice: Implement Drop",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Custom Cleanup

### Task:
Implement `Drop` for `Connection`. In the `drop` method, print `"Disconnected"`.
        "#,
        initial_code: "struct Connection;\n\n// Implement Drop for Connection\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "impl Drop for Connection",
                    matcher: RuleMatcher::OrderedContains(&["impl", "Drop", "for Connection"]),
                },
                ValidationRule {
                    label: "fn drop(&mut self)",
                    matcher: RuleMatcher::Contains("fn drop(&mut self)"),
                },
                ValidationRule {
                    label: "print Disconnected",
                    matcher: RuleMatcher::Contains("Disconnected"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl Drop for Connection {\n    fn drop(&mut self) {\n        println!(\"Disconnected\");\n    }\n}"),
            hints: &[
                "`impl Drop for Connection { fn drop(&mut self) { ... } }`",
                "Print the message inside drop.",
            ],
        },
        success_message: "Automatic cleanup — no garbage collector needed!",
    },
    TutorialModule {
        id: "fdrop-3-practice",
        title: "3. Practice: Early Drop",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `std::mem::drop`

### Task:
Use `drop(conn);` to explicitly drop the connection before the function ends.
        "#,
        initial_code: "struct Connection;\nimpl Drop for Connection {\n    fn drop(&mut self) { println!(\"Disconnected\"); }\n}\n\nfn main() {\n    let conn = Connection;\n    // Drop conn early here\n    println!(\"Connection already closed\");\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "call drop(conn)",
                    matcher: RuleMatcher::Contains("drop(conn)"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn main() {\n    let conn = Connection;\n    drop(conn);\n    println!(\"Connection already closed\");\n}"),
            hints: &[
                "Use `drop(conn);` — this moves the value, triggering Drop.",
            ],
        },
        success_message: "You released the resource before scope end!",
    },
];
