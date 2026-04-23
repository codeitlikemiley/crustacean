use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{RuleMatcher, ValidationRule, ValidationSpec, NormalizeOptions};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "gats_concept_1",
        module_type: ModuleType::Concept,
        title: "Generic Associated Types (GATs)",
        content: "Before Rust 1.65, associated types in traits couldn't have their own generics or lifetimes. This made it impossible to express traits like a `LendingIterator`, where the item yielded by `next()` borrows from the iterator itself.

Generic Associated Types (GATs) solve this by allowing associated types to have generics:

```rust
trait StreamingIterator {
    type Item<'a> where Self: 'a;
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}
```

This ensures the `Item` cannot outlive the `StreamingIterator` it was borrowed from.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's build a LendingIterator trait.",
    },
    TutorialModule {
        id: "gats_practice_1",
        module_type: ModuleType::Practice,
        title: "Defining a LendingIterator",
        content: "Define a `LendingIterator` trait.
1. Add an associated type `Item<'a>` that restricts `Self: 'a`.
2. Add a `next` method that takes `&'a mut self` and returns `Option<Self::Item<'a>>`.",
        initial_code: "pub trait LendingIterator {
    // 1. Define type Item<'a> where Self: 'a;
    
    // 2. Define fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Define Item<'a>",
                    matcher: RuleMatcher::Regex(r#"type\s+Item\s*<\s*'a\s*>\s*where\s+Self\s*:\s*'a\s*;"#),
                },
                ValidationRule {
                    label: "Define next method",
                    matcher: RuleMatcher::Regex(r#"fn\s+next\s*<\s*'a\s*>\s*\(\s*&'a\s+mut\s+self\s*\)\s*->\s*Option\s*<\s*Self::Item\s*<\s*'a\s*>\s*>"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("pub trait LendingIterator {\n    type Item<'a> where Self: 'a;\n    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;\n}"),
            hints: &[
                "Make sure to include `where Self: 'a` for the `Item<'a>` definition.",
                "The `next` signature should be `fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;`."
            ],
        },
        success_message: "Awesome! GATs allow traits to describe complex lifetime relationships.",
    },
    TutorialModule {
        id: "gats_concept_2",
        module_type: ModuleType::Concept,
        title: "Implementing GATs",
        content: "Implementing a trait with a GAT requires you to define the associated type with the exact generic parameters declared in the trait.

```rust
struct MyStream(String);

impl StreamingIterator for MyStream {
    type Item<'a> = &'a str;
    
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        Some(self.0.as_str())
    }
}
```

Notice how `type Item<'a> = &'a str` connects the lifetime of the yielded item to the iterator.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's implement the LendingIterator.",
    },
    TutorialModule {
        id: "gats_practice_2",
        module_type: ModuleType::Practice,
        title: "Implement LendingIterator",
        content: "Implement `LendingIterator` for the `SliceMut` struct.

1. Define `type Item<'a> = &'a mut [u8];` inside the impl block.
2. Complete the `next` method to return `Some(&mut self.data[..])`.",
        initial_code: "pub trait LendingIterator {
    type Item<'a> where Self: 'a;
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

pub struct SliceMut {
    data: Vec<u8>,
}

impl LendingIterator for SliceMut {
    // 1. Define type Item<'a>
    
    // 2. Define next method
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Define type Item<'a>",
                    matcher: RuleMatcher::Contains(r#"type Item<'a> = &'a mut [u8];"#),
                },
                ValidationRule {
                    label: "Define next method",
                    matcher: RuleMatcher::Contains(r#"fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>"#),
                },
                ValidationRule {
                    label: "Return Some(&mut self.data[..])",
                    matcher: RuleMatcher::Contains(r#"Some(&mut self.data[..])"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("pub trait LendingIterator {\n    type Item<'a> where Self: 'a;\n    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;\n}\n\npub struct SliceMut {\n    data: Vec<u8>,\n}\n\nimpl LendingIterator for SliceMut {\n    type Item<'a> = &'a mut [u8];\n    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {\n        Some(&mut self.data[..])\n    }\n}"),
            hints: &[
                "Inside the impl block, add `type Item<'a> = &'a mut [u8];`",
                "The `next` method signature must match the trait exactly: `fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>`"
            ],
        },
        success_message: "You've successfully used Generic Associated Types! GATs are the foundation of many advanced patterns in Rust.",
    },
];
