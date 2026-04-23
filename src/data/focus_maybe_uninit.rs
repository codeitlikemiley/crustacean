use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{RuleMatcher, ValidationRule, ValidationSpec, NormalizeOptions};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "maybe_uninit_concept_1",
        module_type: ModuleType::Concept,
        title: "Uninitialized Memory",
        content: "Rust requires all variables to be initialized before they are read. However, sometimes you want to allocate memory without initializing it immediately (e.g., for performance when filling a large buffer).

In the past, people used `mem::uninitialized()`, but this is now deprecated because it causes immediate undefined behavior for types that cannot be safely uninitialized (like references or Enums with non-zero layouts).

The safe way to represent potentially uninitialized memory is `std::mem::MaybeUninit<T>`.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's see how to use MaybeUninit.",
    },
    TutorialModule {
        id: "maybe_uninit_practice_1",
        module_type: ModuleType::Practice,
        title: "Creating MaybeUninit",
        content: "To create uninitialized memory, you use `MaybeUninit::uninit()`.

1. Import `std::mem::MaybeUninit`.
2. Create an uninitialized integer and assign it to `val`.
3. Do not return anything.",
        initial_code: "pub fn allocate_uninit() {
    // 1. Import MaybeUninit
    // 2. let val: MaybeUninit<u32> = ...
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Import MaybeUninit",
                    matcher: RuleMatcher::Contains(r#"use std::mem::MaybeUninit"#),
                },
                ValidationRule {
                    label: "Create uninit",
                    matcher: RuleMatcher::Contains(r#"MaybeUninit::uninit()"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::mem::MaybeUninit;\n\npub fn allocate_uninit() {\n    let val: MaybeUninit<u32> = MaybeUninit::uninit();\n}"),
            hints: &[
                "Add `use std::mem::MaybeUninit;` at the top or inside the function.",
                "Use `let val: MaybeUninit<u32> = MaybeUninit::uninit();`."
            ],
        },
        success_message: "Awesome! You now have a chunk of memory that Rust knows might be uninitialized.",
    },
    TutorialModule {
        id: "maybe_uninit_concept_2",
        module_type: ModuleType::Concept,
        title: "Assuming Initialization",
        content: "Once you have a `MaybeUninit`, you can safely write to it using the `.write(val)` method.

After you have written to it, the memory is fully initialized. To actually use the inner `T`, you must call `.assume_init()`.

```rust
let mut val = MaybeUninit::<u32>::uninit();
val.write(42);

// UNSAFE: We are telling the compiler \"Trust me, I initialized this.\"
let initialized: u32 = unsafe { val.assume_init() };
```

If you call `assume_init` on memory that hasn't actually been initialized, you trigger Undefined Behavior.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's initialize an array step by step.",
    },
    TutorialModule {
        id: "maybe_uninit_practice_2",
        module_type: ModuleType::Practice,
        title: "Initializing an Array",
        content: "We have an array of `MaybeUninit<u32>`. We want to initialize it in a loop, and then return the fully initialized array.

1. Write `i as u32` into `buffer[i]`.
2. Use `unsafe { std::mem::transmute(buffer) }` to cast `[MaybeUninit<u32>; 5]` to `[u32; 5]` and return it.",
        initial_code: "use std::mem::MaybeUninit;

pub fn init_array() -> [u32; 5] {
    let mut buffer: [MaybeUninit<u32>; 5] = unsafe { MaybeUninit::uninit().assume_init() };
    
    for i in 0..5 {
        // 1. Write `i as u32` to buffer[i]
    }
    
    // 2. Transmute and return
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Write to buffer",
                    matcher: RuleMatcher::Contains(r#"buffer[i].write(i as u32)"#),
                },
                ValidationRule {
                    label: "Transmute buffer",
                    matcher: RuleMatcher::Regex(r#"unsafe\s*\{\s*std::mem::transmute\s*\(\s*buffer\s*\)\s*\}"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::mem::MaybeUninit;\n\npub fn init_array() -> [u32; 5] {\n    let mut buffer: [MaybeUninit<u32>; 5] = unsafe { MaybeUninit::uninit().assume_init() };\n    for i in 0..5 {\n        buffer[i].write(i as u32);\n    }\n    unsafe { std::mem::transmute(buffer) }\n}"),
            hints: &[
                "Inside the loop, do `buffer[i].write(i as u32);`",
                "At the end, return `unsafe { std::mem::transmute(buffer) }`"
            ],
        },
        success_message: "Brilliant! You've successfully initialized memory without zeroing it first. `MaybeUninit::array_assume_init` exists in nightly to do this without `transmute`.",
    },
];
