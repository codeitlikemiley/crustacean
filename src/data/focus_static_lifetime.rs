use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fstatic-1-concept",
        title: "1. The `'static` Lifetime",
        module_type: ModuleType::Concept,
        content: r#"
# What `'static` Actually Means

`'static` is the longest possible lifetime — the entire program duration.

**Two distinct meanings:**
1. **Reference `&'static str`**: A reference that is valid for the entire program. String literals have this: `let s: &'static str = "hello";`.
2. **Bound `T: 'static`**: The type `T` contains no non-static references. It *owns* all its data. `String` is `'static`, but `&str` (with a non-static lifetime) is not.

```rust
// This does NOT mean the value lives forever!
// It means the TYPE contains no borrowed data.
fn spawn<T: Send + 'static>(val: T) { ... }

// String: 'static ✓ (owns its data)
// &str: 'static ✗ (borrows data — unless it's a string literal)
```
        "#,
        initial_code: "// Study 'static, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fstatic-2-concept",
        title: "2. String Literals are `'static`",
        module_type: ModuleType::Concept,
        content: r#"
# Embedded in the Binary

String literals are baked directly into the compiled binary. They exist for the entire runtime of the program — hence `&'static str`.

```rust
let s: &'static str = "I live forever!";

fn get_greeting() -> &'static str {
    "Hello, world!" // This is valid because the data is in the binary
}
```

This is different from `String`, which is heap-allocated and dropped when it goes out of scope.
        "#,
        initial_code: "// Study string literals, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fstatic-3-practice",
        title: "3. Practice: `T: 'static` Bound",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Owned Data Only

### Task:
Write a function `store` that accepts `data: T` where `T: 'static + Send`. This is the typical bound for sending data to another thread.
        "#,
        initial_code: "// Write fn store<T>(...) with 'static + Send bound\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define fn store",
                    matcher: RuleMatcher::Regex(r#"fn\s+store"#),
                },
                ValidationRule {
                    label: "'static bound",
                    matcher: RuleMatcher::Contains("'static"),
                },
                ValidationRule {
                    label: "Send bound",
                    matcher: RuleMatcher::Contains("Send"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn store<T: 'static + Send>(data: T) {\n    // data is guaranteed to own all its data\n}"),
            hints: &[
                "Syntax: `fn store<T: 'static + Send>(data: T)`",
                "This ensures T has no borrowed references.",
            ],
        },
        success_message: "You ensured the data can safely outlive any scope!",
    },
];
