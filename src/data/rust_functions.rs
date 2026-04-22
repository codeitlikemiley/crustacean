use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fn-1-concept",
        title: "1. Concept: Functions",
        module_type: ModuleType::Concept,
        content: r#"
# Defining Functions

In Rust, you define functions using the `fn` keyword.

### Key Rules:
- **Parameters** must have explicit types (e.g., `x: i32`).
- **Return types** are specified after an arrow `->` (e.g., `-> i32`).
- The function body is enclosed in `{}`.

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}
```

**Mental Hook**: `fn name(args: Type) -> ReturnType`
        "#,
        initial_code: "// Read about functions, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fn-2-practice",
        title: "2. Practice: Write `add`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Define an `add` function

### Task:
Write a function named `add` that takes two `i32` parameters named `a` and `b`, and returns an `i32`. You can leave the body empty or return `0` for now.
        "#,
        initial_code: "// Define the `add` function here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn add`",
                    matcher: RuleMatcher::Regex(r#"fn\s+add"#),
                },
                ValidationRule {
                    label: "parameter `a: i32`",
                    matcher: RuleMatcher::Regex(r#"a\s*:\s*i32"#),
                },
                ValidationRule {
                    label: "parameter `b: i32`",
                    matcher: RuleMatcher::Regex(r#"b\s*:\s*i32"#),
                },
                ValidationRule {
                    label: "return type `-> i32`",
                    matcher: RuleMatcher::Regex(r#"->\s*i32"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn add(a: i32, b: i32) -> i32 {\n    0\n}"),
            hints: &[
                "Start with `fn add(`",
                "Ensure parameters are separated by commas and have the `: i32` type.",
            ],
        },
        success_message: "Great! You've defined a function signature.",
    },
    TutorialModule {
        id: "fn-3-concept",
        title: "3. Concept: Expressions vs Statements",
        module_type: ModuleType::Concept,
        content: r#"
# Expressions vs. Statements

Rust is an **expression-based** language. This is a crucial concept!

- **Statements** perform actions but *do not* return a value. (e.g., `let y = 6;`)
- **Expressions** evaluate to a resulting value. (e.g., `5 + 6`, or calling a function)

### The Semicolon Rule
If you add a semicolon to the end of an expression, you turn it into a statement, and it will no longer return its value (it returns `()` instead).

```rust
fn give_five() -> i32 {
    5 // Expression: returns 5
}

fn wrong_five() -> i32 {
    5; // Statement: returns (), causes a type error!
}
```
        "#,
        initial_code: "// Understand the semicolon rule, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fn-4-practice",
        title: "4. Practice: Implicit Return",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Return without `return`

### Task:
Fix the `multiply` function so it returns the result of `a * b` using an implicit return (no `return` keyword, no semicolon).
        "#,
        initial_code: "fn multiply(a: i32, b: i32) -> i32 {\n    let result = a * b;\n    // Return result here\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "return `result` implicitly",
                    matcher: RuleMatcher::Regex(r#"result\s*\}"#),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not use the `return` keyword",
                    matcher: RuleMatcher::Contains("return"),
                },
                ValidationRule {
                    label: "do not use a semicolon after result",
                    matcher: RuleMatcher::Contains("result;"),
                },
            ],
            canonical_solution: Some("fn multiply(a: i32, b: i32) -> i32 {\n    let result = a * b;\n    result\n}"),
            hints: &[
                "Just place the variable name `result` on the last line of the function.",
                "Make sure there is no semicolon `;` after `result`.",
            ],
        },
        success_message: "Perfect! Implicit returns make Rust code concise.",
    },
    TutorialModule {
        id: "fn-5-concept",
        title: "5. Concept: The Unit Type `()`",
        module_type: ModuleType::Concept,
        content: r#"
# The Unit Type `()`

What if a function doesn't return anything useful?

In Rust, functions that don't specify a return type implicitly return the **unit type**, written as `()`. It represents an empty value or "nothing".

```rust
// These two are exactly the same:
fn print_hello() {
    println!("Hello!");
}

fn print_hello_explicit() -> () {
    println!("Hello!");
}
```
        "#,
        initial_code: "// Study the unit type, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fn-6-practice",
        title: "6. Practice: Return `()`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Function returning `()`

### Task:
Write a function `log_message` that takes a `msg: &str` and prints it using `println!`. Do not specify a return type.
        "#,
        initial_code: "// Define `log_message` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn log_message`",
                    matcher: RuleMatcher::Regex(r#"fn\s+log_message"#),
                },
                ValidationRule {
                    label: "parameter `msg: &str`",
                    matcher: RuleMatcher::Regex(r#"msg\s*:\s*&str"#),
                },
                ValidationRule {
                    label: "use `println!`",
                    matcher: RuleMatcher::Contains("println!("),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not specify a return type",
                    matcher: RuleMatcher::Contains("->"),
                },
            ],
            canonical_solution: Some("fn log_message(msg: &str) {\n    println!(\"{}\", msg);\n}"),
            hints: &[
                "The signature should be `fn log_message(msg: &str)`",
                "Inside the body, use `println!(\"{}\", msg);`",
            ],
        },
        success_message: "Excellent! You understand functions that perform actions without returning data.",
    },
    TutorialModule {
        id: "fn-7-concept",
        title: "7. Concept: Nested Functions and Scope",
        module_type: ModuleType::Concept,
        content: r#"
# Nested Functions

In Rust, you can define functions *inside* other functions!

This is useful for helper functions that you only need in one specific place, keeping your outer scope clean.

```rust
fn main() {
    fn helper() {
        println!("I'm inside main!");
    }
    
    helper(); // Call it
}
```

Note: Nested functions do *not* capture variables from their enclosing scope (unlike closures, which we'll learn later).
        "#,
        initial_code: "// Note nested functions, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fn-8-practice",
        title: "8. Practice: Conversion Function",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Celsius to Fahrenheit

### Task:
Write a function `celsius_to_fahrenheit` that takes `c: f64` and returns an `f64`.
The formula is: `(c * 1.8) + 32.0`
Use implicit return.
        "#,
        initial_code: "// Write the converter function\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn celsius_to_fahrenheit`",
                    matcher: RuleMatcher::Regex(r#"fn\s+celsius_to_fahrenheit"#),
                },
                ValidationRule {
                    label: "parameter `c: f64`",
                    matcher: RuleMatcher::Regex(r#"c\s*:\s*f64"#),
                },
                ValidationRule {
                    label: "return type `-> f64`",
                    matcher: RuleMatcher::Regex(r#"->\s*f64"#),
                },
                ValidationRule {
                    label: "correct formula",
                    matcher: RuleMatcher::Regex(r#"\(\s*c\s*\*\s*1\.8\s*\)\s*\+\s*32\.0|c\s*\*\s*1\.8\s*\+\s*32\.0"#),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not use the `return` keyword",
                    matcher: RuleMatcher::Contains("return"),
                },
                ValidationRule {
                    label: "no semicolon on the last line",
                    matcher: RuleMatcher::Regex(r#"32\.0\s*;\s*\}"#),
                },
            ],
            canonical_solution: Some("fn celsius_to_fahrenheit(c: f64) -> f64 {\n    (c * 1.8) + 32.0\n}"),
            hints: &[
                "Signature: `fn celsius_to_fahrenheit(c: f64) -> f64`",
                "Body: just `(c * 1.8) + 32.0` without a semicolon at the end.",
            ],
        },
        success_message: "Fantastic! You've completed the Functions & Expressions course.",
    },
];
