use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "concurrency-1-concept",
        title: "1. Concept: Threads",
        module_type: ModuleType::Concept,
        content: r#"
# Spawning Threads

In most operating systems, an executed program's code is run in a **process**, and the operating system manages multiple processes at once. Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called **threads**.

Rust's standard library uses a 1:1 model of threading.

```rust
use std::thread;
use std::time::Duration;

// Spawn a new thread
let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
});

// The main thread continues running!
// If main finishes, all spawned threads are killed.
```
        "#,
        initial_code: "// Study thread::spawn, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "concurrency-2-practice",
        title: "2. Practice: Join a Thread",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `join()`

Because spawned threads are stopped when the main thread finishes, we need a way to wait for them to finish. We do this by calling `.join().unwrap()` on the handle returned by `spawn`.

### Task:
Spawn a thread that prints `"Hello from thread!"`. Assign the handle to `handle`.
Then call `handle.join().unwrap();` to wait for it.
        "#,
        initial_code: "use std::thread;\n\nfn main() {\n    // let handle = ...\n    // Wait for the thread to finish\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `let handle =`",
                    matcher: RuleMatcher::Regex(r#"let\s+handle\s*="#),
                },
                ValidationRule {
                    label: "use `thread::spawn`",
                    matcher: RuleMatcher::Regex(r#"thread::spawn\s*\(\s*\|\|"#),
                },
                ValidationRule {
                    label: "print from thread",
                    matcher: RuleMatcher::Contains("println!(\"Hello from thread!\")"),
                },
                ValidationRule {
                    label: "call `handle.join().unwrap()`",
                    matcher: RuleMatcher::Regex(r#"handle\.join\(\)\.unwrap\(\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::thread;\n\nfn main() {\n    let handle = thread::spawn(|| {\n        println!(\"Hello from thread!\");\n    });\n    handle.join().unwrap();\n}"),
            hints: &[
                "Spawn: `let handle = thread::spawn(|| { println!(\"Hello from thread!\"); });`",
                "Join: `handle.join().unwrap();`",
            ],
        },
        success_message: "Excellent! The main thread will now wait for the spawned thread.",
    },
    TutorialModule {
        id: "concurrency-3-concept",
        title: "3. Concept: `move` Closures",
        module_type: ModuleType::Concept,
        content: r#"
# Moving Data into Threads

Because a thread might outlive the function that spawned it, Rust won't let you just *borrow* variables from the environment inside a `thread::spawn` closure.

You must force the closure to take **ownership** of the variables it uses by adding the `move` keyword before the closure.

```rust
use std::thread;

let v = vec![1, 2, 3];

// We use `move` to transfer ownership of `v` into the thread.
let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});

// println!("{:?}", v); // Error! `v` has been moved.

handle.join().unwrap();
```
        "#,
        initial_code: "// Study the move keyword, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "concurrency-4-practice",
        title: "4. Practice: Move Data",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Use `move`

### Task:
Fix the `thread::spawn` call by adding the `move` keyword so it takes ownership of the `data` vector.
        "#,
        initial_code: "use std::thread;\n\nfn main() {\n    let data = vec![10, 20, 30];\n    \n    let handle = thread::spawn(|| {\n        println!(\"Data: {:?}\", data);\n    });\n    \n    handle.join().unwrap();\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "add `move` before the closure",
                    matcher: RuleMatcher::Regex(r#"thread::spawn\s*\(\s*move\s*\|\|"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::thread;\n\nfn main() {\n    let data = vec![10, 20, 30];\n    let handle = thread::spawn(move || {\n        println!(\"Data: {:?}\", data);\n    });\n    handle.join().unwrap();\n}"),
            hints: &[
                "Change `|| {` to `move || {`",
            ],
        },
        success_message: "Perfect! Rust guarantees you won't have dangling pointers in other threads.",
    },
    TutorialModule {
        id: "concurrency-5-concept",
        title: "5. Concept: Message Passing",
        module_type: ModuleType::Concept,
        content: r#"
# Channels (`mpsc`)

"Do not communicate by sharing memory; instead, share memory by communicating."

Rust's standard library provides channels for sending data between threads. `mpsc` stands for **Multi-Producer, Single-Consumer**.

```rust
use std::sync::mpsc;
use std::thread;

// Create a transmitter (tx) and receiver (rx)
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
});

// rx.recv() blocks the main thread until it gets a message
let received = rx.recv().unwrap();
println!("Got: {}", received);
```
        "#,
        initial_code: "// Study mpsc channels, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "concurrency-6-practice",
        title: "6. Practice: Send a Message",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Channels

### Task:
Use `tx.send(42).unwrap();` inside the spawned thread.
Then use `rx.recv().unwrap();` in the main thread to receive the value into `let answer`.
        "#,
        initial_code: "use std::sync::mpsc;\nuse std::thread;\n\nfn main() {\n    let (tx, rx) = mpsc::channel();\n    \n    thread::spawn(move || {\n        // send 42 here\n    });\n    \n    // let answer = ...\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "send 42",
                    matcher: RuleMatcher::Regex(r#"tx\.send\(\s*42\s*\)\.unwrap\(\)"#),
                },
                ValidationRule {
                    label: "receive answer",
                    matcher: RuleMatcher::Regex(r#"let\s+answer\s*=\s*rx\.recv\(\)\.unwrap\(\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::sync::mpsc;\nuse std::thread;\n\nfn main() {\n    let (tx, rx) = mpsc::channel();\n    thread::spawn(move || {\n        tx.send(42).unwrap();\n    });\n    let answer = rx.recv().unwrap();\n}"),
            hints: &[
                "Inside closure: `tx.send(42).unwrap();`",
                "Outside closure: `let answer = rx.recv().unwrap();`",
            ],
        },
        success_message: "Great! Data passed safely between threads.",
    },
    TutorialModule {
        id: "concurrency-7-concept",
        title: "7. Concept: Shared State (`Mutex`)",
        module_type: ModuleType::Concept,
        content: r#"
# Mutex: Mutual Exclusion

Sometimes you *do* want to share memory. A `Mutex<T>` ensures that only one thread can access data at a time.

To access the data, a thread must first acquire the lock. When the lock goes out of scope, it is automatically released!

```rust
use std::sync::Mutex;

let m = Mutex::new(5);

{
    // Acquire the lock (blocks until available)
    let mut num = m.lock().unwrap();
    *num = 6;
} // Lock is automatically dropped here

println!("m = {:?}", m);
```
        "#,
        initial_code: "// Study Mutex, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "concurrency-8-practice",
        title: "8. Practice: Lock a Mutex",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Mutate via Mutex

### Task:
Call `counter.lock().unwrap()` and assign it to `mut num`.
Then add `1` to `*num`.
        "#,
        initial_code: "use std::sync::Mutex;\n\nfn main() {\n    let counter = Mutex::new(0);\n    \n    // Lock the counter and add 1\n    \n    println!(\"{:?}\", counter);\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "acquire lock",
                    matcher: RuleMatcher::Regex(r#"let\s+mut\s+num\s*=\s*counter\.lock\(\)\.unwrap\(\)"#),
                },
                ValidationRule {
                    label: "increment",
                    matcher: RuleMatcher::Regex(r#"\*num\s*\+=\s*1|\*num\s*=\s*\*num\s*\+\s*1"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::sync::Mutex;\n\nfn main() {\n    let counter = Mutex::new(0);\n    let mut num = counter.lock().unwrap();\n    *num += 1;\n    println!(\"{:?}\", counter);\n}"),
            hints: &[
                "First line: `let mut num = counter.lock().unwrap();`",
                "Second line: `*num += 1;`",
            ],
        },
        success_message: "Awesome. The Mutex enforces safe concurrent access.",
    },
    TutorialModule {
        id: "concurrency-9-concept",
        title: "9. Concept: `Arc<T>`",
        module_type: ModuleType::Concept,
        content: r#"
# Atomic Reference Counting

If you want to share a `Mutex` between multiple threads, you can't just use `Rc<T>`. `Rc` is not thread-safe!

Instead, you use `Arc<T>` (Atomic Reference Counted). It works exactly like `Rc`, but uses atomic operations that are safe across threads.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Thread-safe shared mutable state!
let counter = Arc::new(Mutex::new(0));
let counter_clone = Arc::clone(&counter);

thread::spawn(move || {
    let mut num = counter_clone.lock().unwrap();
    *num += 1;
});
```
        "#,
        initial_code: "// Study Arc<Mutex<T>>, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "concurrency-10-practice",
        title: "10. Practice: `Arc` + `Mutex`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Thread-Safe State

### Task:
Wrap the `Mutex::new(0)` inside an `Arc::new(...)` so it can be shared across threads safely.
        "#,
        initial_code: "use std::sync::{Arc, Mutex};\nuse std::thread;\n\nfn main() {\n    let counter = Mutex::new(0); // Fix this line!\n    \n    let clone = Arc::clone(&counter);\n    let handle = thread::spawn(move || {\n        let mut num = clone.lock().unwrap();\n        *num += 1;\n    });\n    \n    handle.join().unwrap();\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "use `Arc::new(Mutex::new(0))`",
                    matcher: RuleMatcher::Regex(r#"let\s+counter\s*=\s*Arc::new\(\s*Mutex::new\(\s*0\s*\)\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::sync::{Arc, Mutex};\nuse std::thread;\n\nfn main() {\n    let counter = Arc::new(Mutex::new(0));\n    let clone = Arc::clone(&counter);\n    let handle = thread::spawn(move || {\n        let mut num = clone.lock().unwrap();\n        *num += 1;\n    });\n    handle.join().unwrap();\n}"),
            hints: &[
                "Change the initialization to `let counter = Arc::new(Mutex::new(0));`",
            ],
        },
        success_message: "Congratulations! You've learned how to write safe concurrent Rust code.",
    },
];
