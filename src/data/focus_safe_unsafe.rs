use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{RuleMatcher, ValidationRule, ValidationSpec, NormalizeOptions};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "unsafe_concept_1",
        module_type: ModuleType::Concept,
        title: "Unsafe Superpowers",
        content: "Rust's strict borrowing rules sometimes prevent valid, safe code from compiling. To bypass this, Rust offers the `unsafe` keyword.

`unsafe` gives you four superpowers:
1. Dereference raw pointers (`*const T` and `*mut T`).
2. Call unsafe functions.
3. Access or modify mutable static variables.
4. Implement unsafe traits.

`unsafe` doesn't turn off the borrow checker! It just allows these four operations, telling the compiler: *\"Trust me, I have verified this is sound.\"*",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's see an unsafe block in action.",
    },
    TutorialModule {
        id: "unsafe_practice_1",
        module_type: ModuleType::Practice,
        title: "Using Unsafe Blocks",
        content: "To use an unsafe superpower, you must wrap the operation in an `unsafe {}` block.

We have a raw pointer `ptr`. Dereferencing it requires `unsafe`.

1. Add an `unsafe { ... }` block.
2. Dereference the pointer `*ptr` and return its value.",
        initial_code: "pub fn read_ptr(ptr: *const u32) -> u32 {
    // 1. Return *ptr inside an unsafe block
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Use unsafe block",
                    matcher: RuleMatcher::Regex(r#"unsafe\s*\{"#),
                },
                ValidationRule {
                    label: "Dereference pointer",
                    matcher: RuleMatcher::Contains(r#"*ptr"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("pub fn read_ptr(ptr: *const u32) -> u32 {\n    unsafe { *ptr }\n}"),
            hints: &[
                "You need to wrap `*ptr` inside an `unsafe { }` block.",
                "Make sure there is no semicolon if you are returning the value directly."
            ],
        },
        success_message: "Perfect! You've used `unsafe` to dereference a raw pointer.",
    },
    TutorialModule {
        id: "unsafe_concept_2",
        module_type: ModuleType::Concept,
        title: "Safe Abstractions",
        content: "The main purpose of `unsafe` in Rust is to build **safe abstractions**. 

You wrap unsafe code in a safe function boundary, ensuring that callers can never cause undefined behavior, no matter what arguments they pass. 

A classic example is `Vec`. Internally, it uses raw pointers and manual memory allocation, but its public API is 100% safe to use.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's look at the classic `split_at_mut` pattern.",
    },
    TutorialModule {
        id: "unsafe_practice_2",
        module_type: ModuleType::Practice,
        title: "Building split_at_mut",
        content: "Let's build a simplified version of `split_at_mut` for a slice.

The borrow checker won't let you return two mutable references to the same slice, even if they don't overlap! We have to use `unsafe` to bypass this, while guaranteeing safety through our API.

1. Use `slice::from_raw_parts_mut` to create the left slice.
2. Use `slice::from_raw_parts_mut` to create the right slice.

*Assume we already asserted that `mid <= slice.len()`.*",
        initial_code: "use std::slice;

pub fn split_at_mut(slice: &mut [u8], mid: usize) -> (&mut [u8], &mut [u8]) {
    let len = slice.len();
    assert!(mid <= len);
    
    let ptr = slice.as_mut_ptr();
    
    unsafe {
        (
            // 1. left: slice::from_raw_parts_mut(ptr, mid)
            // 2. right: slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Create left slice",
                    matcher: RuleMatcher::Contains(r#"slice::from_raw_parts_mut(ptr, mid)"#),
                },
                ValidationRule {
                    label: "Create right slice",
                    matcher: RuleMatcher::Regex(r#"slice::from_raw_parts_mut\s*\(\s*ptr\.add\(\s*mid\s*\)\s*,\s*len\s*-\s*mid\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::slice;\n\npub fn split_at_mut(slice: &mut [u8], mid: usize) -> (&mut [u8], &mut [u8]) {\n    let len = slice.len();\n    assert!(mid <= len);\n    let ptr = slice.as_mut_ptr();\n    unsafe {\n        (\n            slice::from_raw_parts_mut(ptr, mid),\n            slice::from_raw_parts_mut(ptr.add(mid), len - mid)\n        )\n    }\n}"),
            hints: &[
                "For the left slice, use `slice::from_raw_parts_mut(ptr, mid)`.",
                "For the right slice, use `slice::from_raw_parts_mut(ptr.add(mid), len - mid)`."
            ],
        },
        success_message: "Brilliant! You've just implemented one of the most famous safe abstractions over unsafe code in Rust.",
    },
];
