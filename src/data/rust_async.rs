use crate::data::model::{TutorialModule, ModuleType};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "async-concept-1",
        title: "1. What is Async Rust?",
        module_type: ModuleType::Concept,
        content: r#"
# Async Rust: Concurrent Execution

Async Rust lets you run many tasks concurrently without threads.

### Key Concepts:
- `async fn` — returns a **Future** (a computation that can be paused)
- `.await` — yields control until the Future completes
- **Executor** — runtime that polls Futures (e.g., tokio, async-std)
- **Non-blocking** — other tasks run while waiting for I/O

### The Big Picture:
```rust
async fn fetch_data() -> String {
    // This doesn't block the thread!
    reqwest::get("https://api.example.com").await
}
```

**Mental Hook**: Async is like a chef managing multiple pots — they stir one, then check another, never wasting time waiting.
        "#,
        initial_code: "// Concept: Study the theory on the left.\n// Click \"ACKNOWLEDGE\" to continue!",
        solution: None,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "async-practice-1",
        title: "2. Practice: Async Function",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Define an Async Function

### Task:
Write an async function called `greet` that takes a `name: &str` parameter and returns `String`.
        "#,
        initial_code: "// Write an async function `greet`\n// that returns a String\n",
        solution: Some(r"async\s+fn\s+greet\s*\(\s*name\s*:\s*&str\s*\)\s*->\s*String"),
        success_message: "Great! You've written your first async function.",
    },
    TutorialModule {
        id: "async-concept-2",
        title: "3. Futures and Polling",
        module_type: ModuleType::Concept,
        content: r#"
# How Futures Work

A Future is a state machine. Each `.await` is a suspension point.

### The Poll Pattern:
1. Executor calls `poll()` on the Future
2. Future runs until it hits an `.await`
3. Future returns `Poll::Pending` and yields
4. When the awaited task completes, executor re-polls

### Pinning:
- Futures must be **pinned** in memory
- `Pin<&mut F>` ensures the Future won't move
- This is why `async fn` returns an impl Future

**Mental Hook**: A Future is like a bookmark — it remembers exactly where it left off.
        "#,
        initial_code: "// Concept: Study the theory on the left.\n// Click \"ACKNOWLEDGE\" to continue!",
        solution: None,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "async-concept-3",
        title: "4. Send and Sync Bounds",
        module_type: ModuleType::Concept,
        content: r#"
# Thread Safety in Async Rust

When Futures run on thread-pool executors, they must be `Send`.

### The Rules:
- `Send` — can be transferred across threads
- `Sync` — can be referenced from multiple threads
- `!Send` types (like `Rc`) cannot be `.await`ed across threads
- Use `Arc` instead of `Rc` for async code
- Use `Mutex` or `RwLock` for shared mutable state

### The Trap:
```rust
// This fails: Rc is !Send
let rc = Rc::new(5);
tokio::spawn(async move {
    println!("{}", *rc); // Error!
});
```

**Mental Hook**: In async land, `Arc` is your friend, `Rc` is your enemy.
        "#,
        initial_code: "// Concept: Study the theory on the left.\n// Click \"ACKNOWLEDGE\" to continue!",
        solution: None,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "async-practice-2",
        title: "5. Practice: Spawn a Task",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Spawn an Async Task

### Task:
Use `tokio::spawn` with an `async move` block that prints "hello".
        "#,
        initial_code: "// Spawn an async task\n// that prints \"hello\"\n",
        solution: Some(r"tokio\s*::\s*spawn\s*\(\s*async\s+move"),
        success_message: "Task spawning mastered!",
    },
];
