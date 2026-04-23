use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{RuleMatcher, ValidationRule, ValidationSpec, NormalizeOptions};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "atomics_concept_1",
        module_type: ModuleType::Concept,
        title: "Atomics and Memory Ordering",
        content: "When multiple threads access the same memory location concurrently, you get a data race (Undefined Behavior!). Mutexes prevent this, but they are heavy.

**Atomics** (`AtomicUsize`, `AtomicBool`, etc.) provide lock-free, hardware-level synchronized access. 

Every atomic operation takes a `std::sync::atomic::Ordering`:
- `Relaxed`: No ordering guarantees, just atomic execution. Good for simple counters.
- `Acquire` / `Release`: Used for synchronization. `Release` ensures prior writes are visible to another thread doing an `Acquire`.
- `SeqCst` (Sequentially Consistent): Strongest ordering, guarantees a global total order of all `SeqCst` operations.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's use a simple AtomicBool.",
    },
    TutorialModule {
        id: "atomics_practice_1",
        module_type: ModuleType::Practice,
        title: "Using AtomicBool",
        content: "Let's use an `AtomicBool` as a simple stop flag for a background worker.

Since the stop flag doesn't synchronize other data (the worker just checks it and exits), we can use `Ordering::Relaxed`.

1. Store `true` into `flag` using `Ordering::Relaxed`.
2. Load the value of `flag` using `Ordering::Relaxed`.",
        initial_code: "use std::sync::atomic::{AtomicBool, Ordering};

pub fn stop_worker(flag: &AtomicBool) {
    // 1. Store true into flag
}

pub fn is_stopped(flag: &AtomicBool) -> bool {
    // 2. Load and return the flag
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Store true with Relaxed",
                    matcher: RuleMatcher::Contains(r#"flag.store(true, Ordering::Relaxed)"#),
                },
                ValidationRule {
                    label: "Load with Relaxed",
                    matcher: RuleMatcher::Contains(r#"flag.load(Ordering::Relaxed)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::sync::atomic::{AtomicBool, Ordering};\n\npub fn stop_worker(flag: &AtomicBool) {\n    flag.store(true, Ordering::Relaxed);\n}\n\npub fn is_stopped(flag: &AtomicBool) -> bool {\n    flag.load(Ordering::Relaxed)\n}"),
            hints: &[
                "Use `flag.store(true, Ordering::Relaxed);` in `stop_worker`.",
                "Use `flag.load(Ordering::Relaxed)` in `is_stopped`."
            ],
        },
        success_message: "Great! `Relaxed` is the fastest ordering, but only safe when you aren't synchronizing other memory accesses.",
    },
    TutorialModule {
        id: "atomics_concept_2",
        module_type: ModuleType::Concept,
        title: "Compare and Exchange",
        content: "A core atomic operation is **Compare and Exchange (CAS)**. 

`compare_exchange(current, new, success_order, failure_order)` checks if the atomic value equals `current`. If it does, it updates it to `new` and returns `Ok(current)`. If it doesn't, it does nothing and returns `Err(actual_value)`.

This is done atomically as a single hardware instruction, making it the foundation of lock-free data structures.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's build a Spinlock using CAS.",
    },
    TutorialModule {
        id: "atomics_practice_2",
        module_type: ModuleType::Practice,
        title: "Building a Spinlock",
        content: "A spinlock is a lock that busy-waits until it can be acquired. We can build one using an `AtomicBool` representing whether the lock is currently `locked`.

To acquire the lock, we want to atomically change the state from `false` (unlocked) to `true` (locked). We will spin in a `while` loop until this succeeds!

1. In the `while` loop, use `compare_exchange`. We expect the current value to be `false`, and we want to change it to `true`.
2. Use `Ordering::Acquire` for success, and `Ordering::Relaxed` for failure.",
        initial_code: "use std::sync::atomic::{AtomicBool, Ordering};

pub struct Spinlock {
    locked: AtomicBool,
}

impl Spinlock {
    pub fn lock(&self) {
        // Spin while we cannot change `false` to `true`
        while self.locked
            // 1. Call compare_exchange(false, true, ...)
            .is_err()
        {
            std::hint::spin_loop();
        }
    }
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "compare_exchange with Acquire/Relaxed",
                    matcher: RuleMatcher::Regex(r#"compare_exchange\s*\(\s*false\s*,\s*true\s*,\s*Ordering::Acquire\s*,\s*Ordering::Relaxed\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::sync::atomic::{AtomicBool, Ordering};\n\npub struct Spinlock {\n    locked: AtomicBool,\n}\n\nimpl Spinlock {\n    pub fn lock(&self) {\n        while self.locked\n            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)\n            .is_err()\n        {\n            std::hint::spin_loop();\n        }\n    }\n}"),
            hints: &[
                "The method call should be `.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)`"
            ],
        },
        success_message: "Brilliant! You've successfully built a basic Spinlock using atomic instructions.",
    },
];
