use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fderef-1-concept",
        title: "1. Deref Coercion",
        module_type: ModuleType::Concept,
        content: r#"
# How `&String` Becomes `&str`

When you pass a `&String` to a function that expects `&str`, Rust automatically calls `Deref::deref()`. This is **deref coercion**.

```rust
use std::ops::Deref;

impl Deref for String {
    type Target = str;
    fn deref(&self) -> &str { ... }
}

fn greet(name: &str) { println!("Hi, {}", name); }

let s = String::from("Alice");
greet(&s); // &String → &str automatically!
```

This applies to any type that implements `Deref`.
        "#,
        initial_code: "// Study deref coercion, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fderef-2-concept",
        title: "2. Deref Chains",
        module_type: ModuleType::Concept,
        content: r#"
# Chained Coercions

Deref coercion can chain: `&Box<String>` → `&String` → `&str`.

The compiler follows the `Deref` chain until it finds a matching type:

```rust
let boxed: Box<String> = Box::new(String::from("hello"));

// All of these work thanks to deref chaining:
let a: &String = &boxed;  // Box<String> → String
let b: &str = &boxed;     // Box<String> → String → str
```
        "#,
        initial_code: "// Study deref chains, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fderef-3-practice",
        title: "3. Practice: Implement Deref",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Custom Smart Pointer

### Task:
Implement `Deref` for `MyBox<T>`. The `type Target = T;` and `deref` should return `&self.0`.
        "#,
        initial_code: "use std::ops::Deref;\n\nstruct MyBox<T>(T);\n\n// Implement Deref for MyBox<T>\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "impl Deref for MyBox<T>",
                    matcher: RuleMatcher::OrderedContains(&["impl", "Deref", "for MyBox"]),
                },
                ValidationRule {
                    label: "type Target = T",
                    matcher: RuleMatcher::Contains("type Target = T;"),
                },
                ValidationRule {
                    label: "fn deref returns &self.0",
                    matcher: RuleMatcher::Contains("&self.0"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl<T> Deref for MyBox<T> {\n    type Target = T;\n    fn deref(&self) -> &T {\n        &self.0\n    }\n}"),
            hints: &[
                "Add `impl<T> Deref for MyBox<T> { ... }`",
                "Return `&self.0` from deref.",
            ],
        },
        success_message: "Your type now coerces automatically!",
    },
    TutorialModule {
        id: "fderef-4-practice",
        title: "4. Practice: DerefMut",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Mutable Deref

### Task:
Implement `DerefMut` for `MyBox<T>`. The `deref_mut` method returns `&mut self.0`.
        "#,
        initial_code: "use std::ops::{Deref, DerefMut};\n\nstruct MyBox<T>(T);\n\nimpl<T> Deref for MyBox<T> {\n    type Target = T;\n    fn deref(&self) -> &T { &self.0 }\n}\n\n// Implement DerefMut for MyBox<T>\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "impl DerefMut for MyBox<T>",
                    matcher: RuleMatcher::OrderedContains(&["impl", "DerefMut", "for MyBox"]),
                },
                ValidationRule {
                    label: "fn deref_mut returns &mut self.0",
                    matcher: RuleMatcher::Contains("&mut self.0"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl<T> DerefMut for MyBox<T> {\n    fn deref_mut(&mut self) -> &mut T {\n        &mut self.0\n    }\n}"),
            hints: &[
                "`impl<T> DerefMut for MyBox<T> { ... }`",
                "Return `&mut self.0`.",
            ],
        },
        success_message: "Full deref coercion for both `&` and `&mut`!",
    },
];
