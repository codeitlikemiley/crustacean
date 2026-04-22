use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);
const PUNCT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, true);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "own-1-concept",
        title: "1. Concept: Ownership Means One Owner",
        module_type: ModuleType::Concept,
        content: r#"
# One Value, One Owner

Rust tracks which variable currently owns each value.

### Core Rule
- Each value has one owner at a time
- When the owner goes out of scope, the value is dropped
- Ownership prevents double-free bugs without a garbage collector

**Mental Hook**: Ownership is a title deed. Only one variable holds the deed at a time.
        "#,
        initial_code: "// Read the ownership model, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "own-2-practice",
        title: "2. Practice: Heap-Owned String",
        module_type: ModuleType::Practice,
        content: r#"
# Create an Owned String

### Task:
Create a variable named `message` using `String::from("hello")`.
        "#,
        initial_code: "// Create `message` with String::from(\"hello\")\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[ValidationRule {
                label: "bind `message` using String::from(\"hello\")",
                matcher: RuleMatcher::Contains("let message = String::from(\"hello\");"),
            }],
            forbidden: &[],
            canonical_solution: Some("let message = String::from(\"hello\");"),
            hints: &[
                "This lesson wants a heap-owned `String`, not a string slice like `\"hello\"`.",
                "Use the binding name `message` exactly as written in the task.",
            ],
        },
        success_message: "Correct. `message` now owns a heap-allocated String.",
    },
    TutorialModule {
        id: "own-3-concept",
        title: "3. Concept: Moves Transfer Ownership",
        module_type: ModuleType::Concept,
        content: r#"
# Moves Hand Off the Value

When you assign an owned value like `String` to another variable, Rust moves ownership.

### Example
```rust
let a = String::from("hi");
let b = a; // ownership moves to b
```

After the move, `a` is no longer valid.
        "#,
        initial_code: "// Review how moves transfer ownership.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "own-4-practice",
        title: "4. Practice: Show a Move",
        module_type: ModuleType::Practice,
        content: r#"
# Transfer Ownership

### Task:
1. Create `first` with `String::from("rust")`
2. Move it into a new variable named `second`
        "#,
        initial_code: "// Create `first`, then move it into `second`\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "create `first` as an owned String",
                    matcher: RuleMatcher::Contains("let first = String::from(\"rust\");"),
                },
                ValidationRule {
                    label: "move `first` into `second`",
                    matcher: RuleMatcher::Contains("let second = first;"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("let first = String::from(\"rust\");\nlet second = first;"),
            hints: &[
                "The second binding should reuse `first` directly, without cloning.",
                "Use the exact target name `second` so the move is easy to read.",
            ],
        },
        success_message: "Correct. Ownership has moved from `first` to `second`.",
    },
    TutorialModule {
        id: "own-5-concept",
        title: "5. Concept: Cloning Copies Heap Data",
        module_type: ModuleType::Concept,
        content: r#"
# Clone Makes a Deep Copy

If you want two owned Strings, call `.clone()`.

### Difference
- assignment moves ownership
- `.clone()` duplicates the heap data

This is explicit so expensive copies are visible in the code.
        "#,
        initial_code: "// Read the clone concept and ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "own-6-practice",
        title: "6. Practice: Clone Instead of Move",
        module_type: ModuleType::Practice,
        content: r#"
# Keep Both Variables Valid

### Task:
1. Create `name` with `String::from("Ferris")`
2. Create `copy` by cloning `name`
        "#,
        initial_code: "// Create `name`\n// then clone it into `copy`\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "create `name` as an owned String",
                    matcher: RuleMatcher::Contains("let name = String::from(\"Ferris\");"),
                },
                ValidationRule {
                    label: "clone `name` into `copy`",
                    matcher: RuleMatcher::Contains("let copy = name.clone();"),
                },
            ],
            forbidden: &[ValidationRule {
                label: "do not move `name` directly into `copy`",
                matcher: RuleMatcher::Contains("let copy = name;"),
            }],
            canonical_solution: Some("let name = String::from(\"Ferris\");\nlet copy = name.clone();"),
            hints: &[
                "Use `.clone()` on `name` so both bindings can own separate Strings.",
                "A plain assignment would move the value, which is not what this exercise asks for.",
            ],
        },
        success_message: "Exactly. `clone()` keeps both owned Strings available.",
    },
    TutorialModule {
        id: "own-7-concept",
        title: "7. Concept: Borrowing with References",
        module_type: ModuleType::Concept,
        content: r#"
# References Borrow Without Owning

A reference lets code look at a value without taking ownership.

### Syntax
- `&String` borrows immutably
- `&mut String` borrows mutably

Borrowing is how Rust shares access safely.
        "#,
        initial_code: "// Review borrowing, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "own-8-practice",
        title: "8. Practice: Borrow a String",
        module_type: ModuleType::Practice,
        content: r#"
# Pass by Reference

### Task:
Write a function `print_len` that:
- takes one parameter `value: &String`
- returns `usize`
- uses `value.len()`
        "#,
        initial_code: "// Write fn print_len(value: &String) -> usize { ... }\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "declare `print_len(value: &String) -> usize`",
                    matcher: RuleMatcher::Regex(
                        r"fn\s+print_len\s*\(\s*value\s*:\s*&String\s*\)\s*->\s*usize",
                    ),
                },
                ValidationRule {
                    label: "return `value.len()` from the body",
                    matcher: RuleMatcher::Contains("value.len()"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn print_len(value: &String) -> usize {\n    value.len()\n}"),
            hints: &[
                "This lesson is about borrowing, so the parameter type must start with `&`.",
                "You only need the length, so the body can return `value.len()` directly.",
            ],
        },
        success_message: "Correct. The function borrows the String instead of taking ownership.",
    },
    TutorialModule {
        id: "own-9-concept",
        title: "9. Concept: Mutable References",
        module_type: ModuleType::Concept,
        content: r#"
# Mutable Borrows Allow Change

Use `&mut` when a function needs permission to modify a value through a reference.

### Pattern
```rust
fn add_mark(s: &mut String) {
    s.push('!');
}
```

Mutable borrowing keeps mutation explicit and controlled.
        "#,
        initial_code: "// Study mutable borrowing, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "own-10-practice",
        title: "10. Practice: Mutable Borrow Parameter",
        module_type: ModuleType::Practice,
        content: r#"
# Borrow Mutably

### Task:
Write a function `append_world` that:
- takes `text: &mut String`
- calls `text.push_str(" world")`
        "#,
        initial_code: "// Write append_world(text: &mut String) { ... }\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "declare `append_world(text: &mut String)`",
                    matcher: RuleMatcher::Regex(
                        r"fn\s+append_world\s*\(\s*text\s*:\s*&mut\s+String\s*\)",
                    ),
                },
                ValidationRule {
                    label: "append ` world` with push_str",
                    matcher: RuleMatcher::Contains("text.push_str(\" world\")"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn append_world(text: &mut String) {\n    text.push_str(\" world\");\n}"),
            hints: &[
                "Use `&mut String` in the parameter so the function can modify the caller's String.",
                "The task asks for `push_str(\" world\")`, not `push` or concatenation.",
            ],
        },
        success_message: "Correct. The function mutates the borrowed String through a mutable reference.",
    },
    TutorialModule {
        id: "own-11-concept",
        title: "11. Concept: Slices Borrow Parts",
        module_type: ModuleType::Concept,
        content: r#"
# Slices Borrow a View into Data

A slice borrows part or all of a collection.

### Common Forms
- `&str` for string slices
- `&[T]` for slices of elements

Slices let you borrow data without copying it.
        "#,
        initial_code: "// Read about slices, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "own-12-practice",
        title: "12. Practice: Return a String Slice",
        module_type: ModuleType::Practice,
        content: r#"
# Borrow a Word Prefix

### Task:
Write a function `first_word` that:
- takes `text: &str`
- returns `&str`
- returns the slice `&text[..1]`
        "#,
        initial_code: "// Write fn first_word(text: &str) -> &str { ... }\n",
        validation: ValidationSpec::Rules {
            normalize: PUNCT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "declare `first_word(text: &str) -> &str`",
                    matcher: RuleMatcher::Regex(
                        r"fn\s+first_word\s*\(\s*text\s*:\s*&str\s*\)\s*->\s*&str",
                    ),
                },
                ValidationRule {
                    label: "return the slice `&text[..1]`",
                    matcher: RuleMatcher::Contains("&text[..1]"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn first_word(text: &str) -> &str {\n    &text[..1]\n}"),
            hints: &[
                "This lesson uses `&str`, not `&String`, to emphasize slice types.",
                "The return expression should be the slice `&text[..1]` exactly.",
            ],
        },
        success_message: "Correct. The function returns a borrowed string slice instead of an owned String.",
    },
    TutorialModule {
        id: "own-13-concept",
        title: "13. Concept: Ownership in Function Calls",
        module_type: ModuleType::Concept,
        content: r#"
# Functions Can Take or Borrow

Passing a value into a function follows the same ownership rules as assignment.

### Meaning
- pass `String` to transfer ownership
- pass `&String` or `&str` to borrow

Function signatures are how Rust makes that decision obvious.
        "#,
        initial_code: "// Read the function-call ownership rules.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "own-14-practice",
        title: "14. Practice: Take Ownership in a Parameter",
        module_type: ModuleType::Practice,
        content: r#"
# Consume the String

### Task:
Write a function `consume` that:
- takes `value: String`
- returns `usize`
- returns `value.len()`
        "#,
        initial_code: "// Write fn consume(value: String) -> usize { ... }\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "declare `consume(value: String) -> usize`",
                    matcher: RuleMatcher::Regex(
                        r"fn\s+consume\s*\(\s*value\s*:\s*String\s*\)\s*->\s*usize",
                    ),
                },
                ValidationRule {
                    label: "return `value.len()` from the body",
                    matcher: RuleMatcher::Contains("value.len()"),
                },
            ],
            forbidden: &[ValidationRule {
                label: "do not borrow the parameter with `&String`",
                matcher: RuleMatcher::Contains("value: &String"),
            }],
            canonical_solution: Some("fn consume(value: String) -> usize {\n    value.len()\n}"),
            hints: &[
                "This lesson is about taking ownership, so the parameter should be plain `String`.",
                "The body can simply return `value.len()`.",
            ],
        },
        success_message: "Correct. The function takes ownership of the String parameter.",
    },
    TutorialModule {
        id: "own-15-concept",
        title: "15. Concept: Ownership Review",
        module_type: ModuleType::Concept,
        content: r#"
# Ownership Review

You now have the three core moves:
- move ownership with assignment or by-value parameters
- clone when you need an explicit duplicate
- borrow with `&` or `&mut` when ownership should stay with the caller

These are the everyday tools of Rust API design.
        "#,
        initial_code: "// Review the ownership summary and ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "own-16-practice",
        title: "16. Practice: Pick the Right Parameter Types",
        module_type: ModuleType::Practice,
        content: r#"
# Mixed Ownership Signals

### Task:
Complete both function signatures:
- `inspect` should take `name: &String`
- `rename` should take `name: &mut String`
        "#,
        initial_code: "// Write inspect(name: ...)\n// Write rename(name: ...)\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "write `inspect(name: &String)`",
                    matcher: RuleMatcher::Regex(
                        r"fn\s+inspect\s*\(\s*name\s*:\s*&String\s*\)",
                    ),
                },
                ValidationRule {
                    label: "write `rename(name: &mut String)`",
                    matcher: RuleMatcher::Regex(
                        r"fn\s+rename\s*\(\s*name\s*:\s*&mut\s+String\s*\)",
                    ),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn inspect(name: &String) {}\nfn rename(name: &mut String) {}"),
            hints: &[
                "Use `&String` for read-only borrowing in `inspect`.",
                "Use `&mut String` for mutable borrowing in `rename`.",
            ],
        },
        success_message: "Well done. You matched immutable and mutable borrows to the right API shape.",
    },
];
