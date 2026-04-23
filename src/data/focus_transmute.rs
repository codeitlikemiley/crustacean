use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{RuleMatcher, ValidationRule, ValidationSpec, NormalizeOptions};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "transmute_concept_1",
        module_type: ModuleType::Concept,
        title: "std::mem::transmute",
        content: "`std::mem::transmute` is the most dangerously powerful function in Rust.

It takes a value of one type and reinterprets its underlying bits as another type. The compiler only checks one thing: that the source and destination types have the **exact same size** in memory.

If you transmute into a type with invalid bit patterns (e.g., transmuting `3` into a `bool`, which must only be `0` or `1`), you immediately trigger Undefined Behavior.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's see transmute in action.",
    },
    TutorialModule {
        id: "transmute_practice_1",
        module_type: ModuleType::Practice,
        title: "Transmuting an Array",
        content: "Let's convert an array of four `u8` bytes into a single `u32` integer.

1. Import `std::mem::transmute`.
2. Use `transmute` to convert the `[u8; 4]` into a `u32` inside an `unsafe` block.",
        initial_code: "pub fn bytes_to_u32(bytes: [u8; 4]) -> u32 {
    // Return the transmuted bytes
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Use unsafe block",
                    matcher: RuleMatcher::Contains(r#"unsafe {"#),
                },
                ValidationRule {
                    label: "Call transmute",
                    matcher: RuleMatcher::Regex(r#"std::mem::transmute\s*\(\s*bytes\s*\)"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("pub fn bytes_to_u32(bytes: [u8; 4]) -> u32 {\n    unsafe { std::mem::transmute(bytes) }\n}"),
            hints: &[
                "Wrap the transmute in an `unsafe { }` block.",
                "Call `std::mem::transmute(bytes)`."
            ],
        },
        success_message: "Nice! Both `[u8; 4]` and `u32` are exactly 4 bytes, so the compiler allows it.",
    },
    TutorialModule {
        id: "transmute_concept_2",
        module_type: ModuleType::Concept,
        title: "The Dangers of Transmute",
        content: "While `transmute` is powerful, it is extremely dangerous. 

For example, transmuting `&T` to `&mut T` is ALWAYS undefined behavior, even if the underlying data is mutable.
Transmuting a generic `T` into another type often fails because the compiler doesn't know the size of `T` until monomorphization.

Because of this, you should almost never use `transmute` directly. Rust provides safer alternatives for almost every use case.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's see the safe alternative to transmute.",
    },
    TutorialModule {
        id: "transmute_practice_2",
        module_type: ModuleType::Practice,
        title: "Safe Bitcasting",
        content: "Instead of transmuting `f32` to `u32`, Rust provides safe, built-in methods that do the exact same thing without `unsafe` blocks.

Convert the `f32` into its raw bits (`u32`) using `.to_bits()`.
Then, convert it back using `f32::from_bits()`.

1. Let `bits = float.to_bits();`
2. Return `f32::from_bits(bits);`",
        initial_code: "pub fn round_trip(float: f32) -> f32 {
    // 1. let bits = float.to_bits();
    
    // 2. Return f32::from_bits(bits)
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "to_bits",
                    matcher: RuleMatcher::Contains(r#"float.to_bits()"#),
                },
                ValidationRule {
                    label: "from_bits",
                    matcher: RuleMatcher::Contains(r#"f32::from_bits(bits)"#),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "No transmute",
                    matcher: RuleMatcher::Contains(r#"transmute"#),
                },
            ],
            canonical_solution: Some("pub fn round_trip(float: f32) -> f32 {\n    let bits = float.to_bits();\n    f32::from_bits(bits)\n}"),
            hints: &[
                "Use `float.to_bits()`.",
                "Use `f32::from_bits(bits)`."
            ],
        },
        success_message: "Perfect! Always use safe alternatives like `to_bits()`, `to_ne_bytes()`, or the `bytemuck` crate instead of `transmute`.",
    },
];
