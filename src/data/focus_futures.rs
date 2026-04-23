use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{RuleMatcher, ValidationRule, ValidationSpec, NormalizeOptions};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "futures_concept_1",
        module_type: ModuleType::Concept,
        title: "Anatomy of a Future",
        content: "In Rust, `async` blocks and functions return a type that implements the `Future` trait.

A `Future` is basically a state machine that can be polled to check if it's finished.

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait SimpleFuture {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

When you call `poll`, the future either returns `Poll::Ready(value)` if it's done, or `Poll::Pending` if it's still waiting. If it returns `Pending`, it must arrange for the `Waker` (inside `Context`) to be called when it's ready to be polled again.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's build our own simple future.",
    },
    TutorialModule {
        id: "futures_practice_1",
        module_type: ModuleType::Practice,
        title: "Implement a Future",
        content: "Let's implement a future that immediately resolves to `42`.

Implement the `Future` trait for `ImmediateAnswer`.

1. Set `type Output = u32;`.
2. Implement the `poll` method to return `Poll::Ready(42)`. You don't need to worry about `Pin` or `Context` since we're immediately ready.",
        initial_code: "use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct ImmediateAnswer;

impl Future for ImmediateAnswer {
    // 1. type Output = u32;
    
    // 2. fn poll(...) -> Poll<Self::Output>
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Define type Output = u32",
                    matcher: RuleMatcher::Regex(r#"type\s+Output\s*=\s*u32\s*;"#),
                },
                ValidationRule {
                    label: "Define poll method signature",
                    matcher: RuleMatcher::Regex(r#"fn\s+poll\s*\(\s*self\s*:\s*Pin\s*<\s*&mut\s+Self\s*>\s*,\s*_?cx\s*:\s*&mut\s+Context\s*<\s*'_\s*>\s*\)\s*->\s*Poll\s*<\s*Self::Output\s*>"#),
                },
                ValidationRule {
                    label: "Return Poll::Ready(42)",
                    matcher: RuleMatcher::Regex(r#"Poll::Ready\s*\(\s*42\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::future::Future;\nuse std::pin::Pin;\nuse std::task::{Context, Poll};\n\npub struct ImmediateAnswer;\n\nimpl Future for ImmediateAnswer {\n    type Output = u32;\n    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {\n        Poll::Ready(42)\n    }\n}"),
            hints: &[
                "The `poll` signature is: `fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>`",
                "Inside `poll`, just return `Poll::Ready(42)`."
            ],
        },
        success_message: "Awesome! You've implemented a future from scratch.",
    },
    TutorialModule {
        id: "futures_concept_2",
        module_type: ModuleType::Concept,
        title: "The Waker",
        content: "What if the future is NOT ready? It returns `Poll::Pending`. 

But executors (like Tokio) don't busy-wait by polling in an infinite loop. That would waste CPU! Instead, when a future returns `Poll::Pending`, it is responsible for calling `cx.waker().wake_by_ref()` when it is *finally* ready.

The executor park itself until the `Waker` is called, and then it polls the future again.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's see how to use the Waker.",
    },
    TutorialModule {
        id: "futures_practice_2",
        module_type: ModuleType::Practice,
        title: "Waking a Future",
        content: "We have a `Delay` future that needs to notify the executor when a timer completes.

If the timer is not finished, we must clone the `Waker` and hand it to the background thread.

1. Call `cx.waker().clone()` and store it in `self.waker`.
2. Return `Poll::Pending`.",
        initial_code: "use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

pub struct Delay {
    pub is_ready: bool,
    pub waker: Option<Waker>,
}

impl Future for Delay {
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.is_ready {
            Poll::Ready(())
        } else {
            // 1. self.waker = Some(cx.waker().clone());
            
            // 2. Return Poll::Pending
        }
    }
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Clone the waker",
                    matcher: RuleMatcher::Contains(r#"self.waker = Some(cx.waker().clone())"#),
                },
                ValidationRule {
                    label: "Return Poll::Pending",
                    matcher: RuleMatcher::Contains(r#"Poll::Pending"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::future::Future;\nuse std::pin::Pin;\nuse std::task::{Context, Poll, Waker};\n\npub struct Delay {\n    pub is_ready: bool,\n    pub waker: Option<Waker>,\n}\n\nimpl Future for Delay {\n    type Output = ();\n    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {\n        if self.is_ready {\n            Poll::Ready(())\n        } else {\n            self.waker = Some(cx.waker().clone());\n            Poll::Pending\n        }\n    }\n}"),
            hints: &[
                "Inside the `else` block, set `self.waker = Some(cx.waker().clone());`",
                "Don't forget to return `Poll::Pending` after storing the waker."
            ],
        },
        success_message: "Perfect! Now the background thread can call `waker.wake()` when the delay finishes, telling the executor to poll this future again.",
    },
];
