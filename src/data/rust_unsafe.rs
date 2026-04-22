use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "unsafe-1-concept",
        title: "1. Concept: Unsafe Rust",
        module_type: ModuleType::Concept,
        content: r#"
# The `unsafe` Superpower

Rust guarantees memory safety at compile time. However, sometimes you need to do things the compiler can't verify (like talking to hardware or C libraries). 

For this, you use the `unsafe` keyword. It gives you 5 superpowers:
1. Dereference a raw pointer
2. Call an unsafe function or method
3. Access or modify a mutable static variable
4. Implement an unsafe trait
5. Access fields of `union`s

```rust
let mut num = 5;
// Raw pointers are created safely...
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

// ...but dereferencing them is unsafe!
unsafe {
    println!("r1 is: {}", *r1);
}
```

**Mental Hook**: `unsafe` doesn't turn off the borrow checker; it just lets you do five specific dangerous things. The programmer now takes responsibility for memory safety.
        "#,
        initial_code: "// Study unsafe Rust, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "unsafe-2-practice",
        title: "2. Practice: Dereference a Raw Pointer",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Raw Pointers

### Task:
Use an `unsafe { ... }` block to dereference the raw pointer `ptr` and return the value it points to.
        "#,
        initial_code: "fn read_raw(ptr: *const i32) -> i32 {\n    // Dereference `ptr` inside an unsafe block\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "use an `unsafe` block",
                    matcher: RuleMatcher::Regex(r#"unsafe\s*\{"#),
                },
                ValidationRule {
                    label: "dereference `ptr`",
                    matcher: RuleMatcher::Regex(r#"\*ptr"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn read_raw(ptr: *const i32) -> i32 {\n    unsafe { *ptr }\n}"),
            hints: &[
                "Wrap it in `unsafe { ... }`",
                "Dereference it using `*ptr`.",
            ],
        },
        success_message: "Great! You bypassed compile-time checks.",
    },
    TutorialModule {
        id: "unsafe-3-concept",
        title: "3. Concept: Unsafe Functions",
        module_type: ModuleType::Concept,
        content: r#"
# Calling Unsafe Functions

If a function is declared with `unsafe fn`, it means it has preconditions that the compiler cannot check. It is up to the caller to ensure those conditions are met.

Therefore, you can only call an `unsafe fn` from within an `unsafe` block (or another `unsafe fn`).

```rust
unsafe fn dangerous() {
    // ...
}

fn safe_wrapper() {
    // We must use an unsafe block here!
    unsafe {
        dangerous();
    }
}
```
        "#,
        initial_code: "// Study unsafe functions, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "unsafe-4-practice",
        title: "4. Practice: Call an Unsafe Function",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Call `dangerous`

### Task:
Call the `dangerous` function from within `main`. You must wrap the call in an `unsafe` block.
        "#,
        initial_code: "unsafe fn dangerous() {}\n\nfn main() {\n    // Call `dangerous()` here\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "use `unsafe` block",
                    matcher: RuleMatcher::Regex(r#"unsafe\s*\{"#),
                },
                ValidationRule {
                    label: "call `dangerous()`",
                    matcher: RuleMatcher::Contains("dangerous();"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("unsafe fn dangerous() {}\n\nfn main() {\n    unsafe {\n        dangerous();\n    }\n}"),
            hints: &[
                "Use `unsafe { dangerous(); }`",
            ],
        },
        success_message: "Well done! Writing safe wrappers around unsafe code is a core pattern in Rust.",
    },
    TutorialModule {
        id: "unsafe-5-concept",
        title: "5. Concept: Mutable Statics",
        module_type: ModuleType::Concept,
        content: r#"
# Global State

Rust supports global variables using the `static` keyword. 

Unlike constants (`const`), `static` variables have a fixed memory address.

If a `static` variable is `mut`, accessing or modifying it is **unsafe**, because multiple threads might access it at the same time, causing a data race!

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc; // Unsafe!
    }
}
```
        "#,
        initial_code: "// Study mutable statics, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "unsafe-6-practice",
        title: "6. Practice: Modifying Globals",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `static mut`

### Task:
Create an `unsafe` block inside `read_global` to return the current value of `GLOBAL_VAR`.
        "#,
        initial_code: "static mut GLOBAL_VAR: i32 = 42;\n\nfn read_global() -> i32 {\n    // Return GLOBAL_VAR safely\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "use `unsafe` block",
                    matcher: RuleMatcher::Regex(r#"unsafe\s*\{"#),
                },
                ValidationRule {
                    label: "return `GLOBAL_VAR`",
                    matcher: RuleMatcher::Contains("GLOBAL_VAR"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("static mut GLOBAL_VAR: i32 = 42;\n\nfn read_global() -> i32 {\n    unsafe { GLOBAL_VAR }\n}"),
            hints: &[
                "The body should just be `unsafe { GLOBAL_VAR }`",
            ],
        },
        success_message: "Perfect! (But remember, in real code, `Mutex` is much safer than `static mut`).",
    },
    TutorialModule {
        id: "unsafe-7-concept",
        title: "7. Concept: FFI (Extern)",
        module_type: ModuleType::Concept,
        content: r#"
# Foreign Function Interface

To call code written in another language (usually C), Rust uses the `extern` keyword.

Because the Rust compiler cannot check C code for memory safety or thread safety, every call to an `extern` function is **unsafe**.

```rust
// Declare the C function signature
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```
        "#,
        initial_code: "// Study FFI, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "unsafe-8-practice",
        title: "8. Practice: Call a C Function",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `extern`

### Task:
Use an `unsafe` block to call the external `sleep(1)` function from C.
        "#,
        initial_code: "extern \"C\" {\n    fn sleep(seconds: u32);\n}\n\nfn main() {\n    // Call sleep(1) here\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "use `unsafe` block",
                    matcher: RuleMatcher::Regex(r#"unsafe\s*\{"#),
                },
                ValidationRule {
                    label: "call `sleep(1)`",
                    matcher: RuleMatcher::Contains("sleep(1);"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("extern \"C\" {\n    fn sleep(seconds: u32);\n}\n\nfn main() {\n    unsafe {\n        sleep(1);\n    }\n}"),
            hints: &[
                "Write `unsafe { sleep(1); }`",
            ],
        },
        success_message: "Excellent! You've mastered the final frontier of Rust.",
    },
];
