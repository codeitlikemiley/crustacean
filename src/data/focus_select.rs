use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{RuleMatcher, ValidationRule, ValidationSpec, NormalizeOptions};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "select_concept_1",
        module_type: ModuleType::Concept,
        title: "Racing Futures with select!",
        content: "Sometimes you want to run multiple async operations concurrently and only care about the one that finishes first. For example, fetching data from an API with a timeout.

The `tokio::select!` macro allows you to wait on multiple asynchronous branches. As soon as *one* branch completes, the others are immediately cancelled (dropped).

```rust
tokio::select! {
    res = fetch_data() => {
        println!(\"Data: {:?}\", res);
    }
    _ = tokio::time::sleep(Duration::from_secs(5)) => {
        println!(\"Timeout!\");
    }
}
```",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's try using select!.",
    },
    TutorialModule {
        id: "select_practice_1",
        module_type: ModuleType::Practice,
        title: "Using tokio::select!",
        content: "Write a `select!` block that races `job_1()` and `job_2()`.

1. Add a `tokio::select! { ... }` block.
2. If `job_1()` finishes first, assign its result to `val` and return `val * 2`.
3. If `job_2()` finishes first, assign its result to `val` and return `val * 3`.",
        initial_code: "pub async fn race_jobs() -> u32 {
    // Write tokio::select! here
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "tokio::select! macro",
                    matcher: RuleMatcher::Regex(r#"tokio::select!\s*\{"#),
                },
                ValidationRule {
                    label: "Branch for job_1",
                    matcher: RuleMatcher::Regex(r#"val\s*=\s*job_1\s*\(\s*\)\s*=>\s*\{\s*val\s*\*\s*2\s*\}|val\s*=\s*job_1\s*\(\s*\)\s*=>\s*val\s*\*\s*2"#),
                },
                ValidationRule {
                    label: "Branch for job_2",
                    matcher: RuleMatcher::Regex(r#"val\s*=\s*job_2\s*\(\s*\)\s*=>\s*\{\s*val\s*\*\s*3\s*\}|val\s*=\s*job_2\s*\(\s*\)\s*=>\s*val\s*\*\s*3"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("pub async fn race_jobs() -> u32 {\n    tokio::select! {\n        val = job_1() => val * 2,\n        val = job_2() => val * 3,\n    }\n}"),
            hints: &[
                "The syntax is `val = job_1() => val * 2,` inside the `tokio::select! { ... }` block.",
                "Ensure both `job_1()` and `job_2()` have branches."
            ],
        },
        success_message: "Awesome! The macro automatically cancels whichever future didn't finish.",
    },
    TutorialModule {
        id: "select_concept_2",
        module_type: ModuleType::Concept,
        title: "Cancellation Safety",
        content: "When `tokio::select!` cancels a branch, it literally drops the `Future`. 

If dropping that future loses important state (e.g., data that was read from a socket but hasn't been fully processed yet), the future is **NOT cancellation safe**.

For example, `tokio::io::AsyncReadExt::read` is safe because if it's dropped, nothing was read.
But `tokio::io::AsyncBufReadExt::read_line` is **NOT safe** because it might have read half a line into an internal buffer. If it's dropped, that half-line is gone forever!",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's fix a cancellation bug.",
    },
    TutorialModule {
        id: "select_practice_2",
        module_type: ModuleType::Practice,
        title: "Fixing Unsafe Cancellation",
        content: "We are reading frames from an `AsyncRead` stream. 
Currently, we use `stream.read_frame()` inside the `select!` block in a loop. But `read_frame` is NOT cancellation safe! If the heartbeat triggers, we lose a partially read frame.

To fix this, we should only call `select!` on the stream's `next()` method if it returns an optional frame, or handle the heartbeat. Wait, an easier fix is to put the `select!` *inside* the state machine or use a cancellation-safe channel.

Actually, let's fix the problem by extracting the `read_frame` out of the select, or by wrapping it in `tokio::spawn` and awaiting the `JoinHandle`, which IS cancellation safe!

1. Wrap `stream.read_frame()` in `tokio::spawn`.
2. Await the `JoinHandle` inside the `select!` block.",
        initial_code: "pub async fn handle_connection(mut stream: Stream) {
    // Fix this code by spawning read_frame
    tokio::select! {
        frame = stream.read_frame() => {
            process(frame);
        }
        _ = heartbeat() => {
            ping();
        }
    }
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Spawn read_frame",
                    matcher: RuleMatcher::Contains(r#"tokio::spawn"#),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "Don't put read_frame directly in select",
                    matcher: RuleMatcher::Contains(r#"frame = stream.read_frame() =>"#),
                },
            ],
            canonical_solution: Some("pub async fn handle_connection(mut stream: Stream) {\n    let handle = tokio::spawn(async move { stream.read_frame().await });\n    tokio::select! {\n        frame = handle => {\n            process(frame.unwrap());\n        }\n        _ = heartbeat() => {\n            ping();\n        }\n    }\n}"),
            hints: &[
                "Spawn a task: `let handle = tokio::spawn(async move { stream.read_frame().await });`",
                "Wait for the handle in the select: `res = handle => process(res.unwrap())`"
            ],
        },
        success_message: "Great job! A `JoinHandle` just waits for the background task, so dropping it doesn't cancel the actual background work. It's completely cancellation safe.",
    },
];
