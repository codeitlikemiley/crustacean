use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "async-1-concept",
        title: "1. Concept: What is Async Rust?",
        module_type: ModuleType::Concept,
        content: r#"
# Async Rust: Concurrent Execution

Async Rust allows you to write concurrent code that is highly efficient. Instead of using OS threads (which are heavy), it uses a cooperative model.

### Key Concepts:
- `async fn` — defines a function that returns a **Future** (a computation that hasn't finished yet).
- **Future** — a state machine representing an asynchronous operation.
- **Executor** — a runtime (like `tokio` or `async-std`) that polls Futures and actually drives them to completion.

```rust
// This function doesn't run immediately!
// It returns a Future.
async fn fetch_data() -> String {
    String::from("Data!")
}
```
        "#,
        initial_code: "// Study the theory of Async Rust, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "async-2-practice",
        title: "2. Practice: Define an Async Function",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `async fn`

### Task:
Write an asynchronous function called `greet` that takes a `name: &str` parameter and returns a `String`.
You can just use `format!("Hello, {}", name)` in the body.
        "#,
        initial_code: "// Write `async fn greet` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `async fn greet`",
                    matcher: RuleMatcher::Regex(r#"async\s+fn\s+greet"#),
                },
                ValidationRule {
                    label: "parameter `name: &str`",
                    matcher: RuleMatcher::Regex(r#"name\s*:\s*&str"#),
                },
                ValidationRule {
                    label: "return type `-> String`",
                    matcher: RuleMatcher::Regex(r#"->\s*String"#),
                },
                ValidationRule {
                    label: "return `format!(...)`",
                    matcher: RuleMatcher::Contains("format!("),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("async fn greet(name: &str) -> String {\n    format!(\"Hello, {}\", name)\n}"),
            hints: &[
                "The signature should be `async fn greet(name: &str) -> String`",
            ],
        },
        success_message: "Great! You've written a function that returns a Future.",
    },
    TutorialModule {
        id: "async-3-concept",
        title: "3. Concept: The `.await` Keyword",
        module_type: ModuleType::Concept,
        content: r#"
# Yielding Control

Because an `async fn` just returns a `Future`, calling it does nothing. To actually execute the Future and get its result, you use `.await`.

When a Future hits a point where it needs to wait (e.g., for a network response), the `.await` keyword yields control back to the executor, allowing it to run *other* tasks on the same thread!

```rust
async fn do_work() {
    // Calling fetch_data() does nothing on its own.
    // We MUST .await it to drive it forward.
    let data = fetch_data().await;
    println!("Got: {}", data);
}
```

*Note: You can only use `.await` inside another `async` function or block.*
        "#,
        initial_code: "// Study the .await keyword, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "async-4-practice",
        title: "4. Practice: `.await` a Future",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `.await`

### Task:
Inside the async `main` function, call `fetch_user_id()`. 
Assign the result to `let id` by appending `.await` to the call.
        "#,
        initial_code: "async fn fetch_user_id() -> u32 {\n    42\n}\n\nasync fn main() {\n    // let id = ...\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `let id =`",
                    matcher: RuleMatcher::Regex(r#"let\s+id\s*="#),
                },
                ValidationRule {
                    label: "call `fetch_user_id().await`",
                    matcher: RuleMatcher::Regex(r#"fetch_user_id\(\)\.await"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("async fn fetch_user_id() -> u32 {\n    42\n}\n\nasync fn main() {\n    let id = fetch_user_id().await;\n}"),
            hints: &[
                "It should look like `let id = fetch_user_id().await;`",
            ],
        },
        success_message: "Excellent! You are now driving Futures to completion.",
    },
    TutorialModule {
        id: "async-5-concept",
        title: "5. Concept: Concurrent Execution",
        module_type: ModuleType::Concept,
        content: r#"
# `join!` Macro

If you `.await` multiple futures one after the other, they run sequentially.

```rust
let a = fetch_a().await; // Waits for a
let b = fetch_b().await; // Then waits for b
```

If they are independent, you can run them concurrently using the `join!` macro from a runtime like Tokio or `futures`. It polls them all at the same time and returns a tuple of their results once they are all complete.

```rust
// Run concurrently!
let (a, b) = futures::join!(fetch_a(), fetch_b());
```
        "#,
        initial_code: "// Study join!, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "async-6-practice",
        title: "6. Practice: Run Concurrently",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `join!`

### Task:
Use `futures::join!` to run `task1()` and `task2()` at the same time. Assign the resulting tuple to `let (res1, res2)`.
        "#,
        initial_code: "async fn task1() -> i32 { 1 }\nasync fn task2() -> i32 { 2 }\n\nasync fn run_all() {\n    // let (res1, res2) = futures::join!(...)\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `let (res1, res2) =`",
                    matcher: RuleMatcher::Regex(r#"let\s*\(\s*res1\s*,\s*res2\s*\)\s*="#),
                },
                ValidationRule {
                    label: "call `futures::join!`",
                    matcher: RuleMatcher::Contains("futures::join!("),
                },
                ValidationRule {
                    label: "pass `task1(), task2()`",
                    matcher: RuleMatcher::Contains("task1(), task2()"),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not .await them individually",
                    matcher: RuleMatcher::Regex(r#"task1\(\)\.await"#),
                },
            ],
            canonical_solution: Some("async fn task1() -> i32 { 1 }\nasync fn task2() -> i32 { 2 }\n\nasync fn run_all() {\n    let (res1, res2) = futures::join!(task1(), task2());\n}"),
            hints: &[
                "The syntax is `let (res1, res2) = futures::join!(task1(), task2());`",
            ],
        },
        success_message: "Awesome! You are now writing truly concurrent code.",
    },
    TutorialModule {
        id: "async-7-concept",
        title: "7. Concept: Spawning Tasks",
        module_type: ModuleType::Concept,
        content: r#"
# Spawning Background Tasks

`join!` requires you to wait for all the futures to finish right there.

If you want to fire off a task in the background and keep doing other things, you can spawn an async task on the executor. This is the async equivalent of `thread::spawn`.

In Tokio, this is done with `tokio::spawn`.

```rust
tokio::spawn(async move {
    // This runs independently in the background
    let data = fetch_data().await;
    println!("Background data: {}", data);
});
```

Tasks spawned this way require the future to be `'static` (meaning it must own all its data), which is why we use `async move`.
        "#,
        initial_code: "// Study tokio::spawn, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "async-8-practice",
        title: "8. Practice: `tokio::spawn`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Background Task

### Task:
Use `tokio::spawn` with an `async move` block. Inside the block, print `"Background task running"`.
        "#,
        initial_code: "async fn main() {\n    // Spawn a background task here\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "call `tokio::spawn`",
                    matcher: RuleMatcher::Regex(r#"tokio::spawn\s*\("#),
                },
                ValidationRule {
                    label: "use `async move {`",
                    matcher: RuleMatcher::Regex(r#"async\s+move\s*\{"#),
                },
                ValidationRule {
                    label: "print the message",
                    matcher: RuleMatcher::Contains("println!(\"Background task running\")"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("async fn main() {\n    tokio::spawn(async move {\n        println!(\"Background task running\");\n    });\n}"),
            hints: &[
                "Wrap it like this: `tokio::spawn(async move { println!(\"Background task running\"); });`",
            ],
        },
        success_message: "Congratulations! You've completed the Async Rust module.",
    },
];
