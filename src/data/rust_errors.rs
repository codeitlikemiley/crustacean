use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "errors-1-concept",
        title: "1. Concept: Two Types of Errors",
        module_type: ModuleType::Concept,
        content: r#"
# Unrecoverable vs Recoverable Errors

Rust groups errors into two major categories:

### Unrecoverable Errors (`panic!`)
When something goes wrong and the program cannot (or should not) continue, Rust uses the `panic!` macro. This prints a failure message, unwinds the stack, and exits.
- Examples: accessing an array out of bounds, dividing by zero.

### Recoverable Errors (`Result<T, E>`)
For expected problems (like a file missing), Rust uses the `Result` enum. This forces the programmer to handle the failure gracefully.

```rust
// Panic explicitly
panic!("Crash and burn!");
```
        "#,
        initial_code: "// Learn about error types, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "errors-2-practice",
        title: "2. Practice: Trigger a Panic",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `panic!`

### Task:
Inside the `crash` function, use the `panic!` macro with the message `"Something went wrong"`.
        "#,
        initial_code: "fn crash() {\n    // trigger a panic here\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "use `panic!`",
                    matcher: RuleMatcher::Contains("panic!("),
                },
                ValidationRule {
                    label: "with the correct message",
                    matcher: RuleMatcher::Contains("\"Something went wrong\""),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn crash() {\n    panic!(\"Something went wrong\");\n}"),
            hints: &[
                "Call the macro: `panic!(\"Something went wrong\");`",
            ],
        },
        success_message: "Kaboom! You successfully aborted the program.",
    },
    TutorialModule {
        id: "errors-3-concept",
        title: "3. Concept: Deep Dive into `Result`",
        module_type: ModuleType::Concept,
        content: r#"
# Handling `Result<T, E>`

The `Result` enum is defined as:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

When a function can fail, it returns a `Result`. You typically use `match` to handle the variants.

```rust
use std::fs::File;

let f = File::open("hello.txt");

let f = match f {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error),
};
```
        "#,
        initial_code: "// Read about Result, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "errors-4-practice",
        title: "4. Practice: Match a Result",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Handling File Open

### Task:
Use `match` on `File::open("data.txt")`.
- On `Ok(f)`, return `f`
- On `Err(e)`, call `panic!("Failed to open file")`
        "#,
        initial_code: "use std::fs::File;\n\nfn get_file() -> File {\n    // Match on File::open(\"data.txt\") here\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "match `File::open`",
                    matcher: RuleMatcher::Regex(r#"match\s+File::open\(\s*"data\.txt"\s*\)"#),
                },
                ValidationRule {
                    label: "Ok arm",
                    matcher: RuleMatcher::Regex(r#"Ok\s*\(\s*\w+\s*\)\s*=>"#),
                },
                ValidationRule {
                    label: "Err arm",
                    matcher: RuleMatcher::Regex(r#"Err\s*\(\s*_\s*\)\s*=>|Err\s*\(\s*\w+\s*\)\s*=>"#),
                },
                ValidationRule {
                    label: "panic inside Err",
                    matcher: RuleMatcher::Contains("panic!(\"Failed to open file\")"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use std::fs::File;\n\nfn get_file() -> File {\n    match File::open(\"data.txt\") {\n        Ok(f) => f,\n        Err(e) => panic!(\"Failed to open file\"),\n    }\n}"),
            hints: &[
                "Write `match File::open(\"data.txt\") { ... }`",
                "The Err arm should look like `Err(_) => panic!(\"Failed to open file\"),`",
            ],
        },
        success_message: "Great! However, writing `match` every time gets tedious...",
    },
    TutorialModule {
        id: "errors-5-concept",
        title: "5. Concept: The `?` Operator",
        module_type: ModuleType::Concept,
        content: r#"
# Propagating Errors with `?`

Writing `match` for every `Result` is verbose. Rust provides the `?` operator for this exact pattern: propagating errors up to the caller.

If the value of the `Result` is an `Ok`, the value inside the `Ok` will be extracted and the program continues. If the value is an `Err`, the `Err` will be returned from the whole function early.

```rust
use std::fs::File;
use std::io::{self, Read};

// Notice the return type is Result
fn read_username() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // Early return if Err
    let mut s = String::new();
    f.read_to_string(&mut s)?; // Early return if Err
    Ok(s) // Wrap the success value in Ok
}
```
        "#,
        initial_code: "// Study the ? operator, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "errors-6-practice",
        title: "6. Practice: Chain with `?`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Use `?`

### Task:
Refactor the function to use the `?` operator.
1. Add `?` to the end of `File::open(...)`.
2. Wrap the final `f` in `Ok()`.
        "#,
        initial_code: "use std::fs::File;\nuse std::io;\n\nfn get_file() -> Result<File, io::Error> {\n    let f = File::open(\"data.txt\");\n    f\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "append `?` to `File::open`",
                    matcher: RuleMatcher::Regex(r#"File::open\(\s*"data\.txt"\s*\)\?"#),
                },
                ValidationRule {
                    label: "return `Ok(f)`",
                    matcher: RuleMatcher::Regex(r#"Ok\s*\(\s*f\s*\)"#),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not use match",
                    matcher: RuleMatcher::Contains("match "),
                },
            ],
            canonical_solution: Some("use std::fs::File;\nuse std::io;\n\nfn get_file() -> Result<File, io::Error> {\n    let f = File::open(\"data.txt\")?;\n    Ok(f)\n}"),
            hints: &[
                "Change the assignment to `let f = File::open(\"data.txt\")?;`",
                "The last line should be `Ok(f)` to match the `Result<File, io::Error>` return type.",
            ],
        },
        success_message: "Awesome! The `?` operator makes error handling incredibly clean.",
    },
    TutorialModule {
        id: "errors-7-concept",
        title: "7. Concept: Custom Error Types",
        module_type: ModuleType::Concept,
        content: r#"
# Designing Error Enums

When building an application, you often have multiple types of errors (e.g., IO errors, Parsing errors, Network errors).

It is idiomatic to define your own error enum that encapsulates all the ways your app can fail.

```rust
enum AppError {
    IoError,
    ParseError,
    NotFound,
}

fn do_work() -> Result<(), AppError> {
    // ...
    Err(AppError::NotFound)
}
```
        "#,
        initial_code: "// Study custom error enums, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "errors-8-practice",
        title: "8. Practice: Define `AppError`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Custom Error

### Task:
Define an enum named `AppError` with two variants:
- `DatabaseError`
- `AuthError`
        "#,
        initial_code: "// Define `enum AppError` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `enum AppError`",
                    matcher: RuleMatcher::Regex(r#"enum\s+AppError"#),
                },
                ValidationRule {
                    label: "variant `DatabaseError`",
                    matcher: RuleMatcher::Contains("DatabaseError"),
                },
                ValidationRule {
                    label: "variant `AuthError`",
                    matcher: RuleMatcher::Contains("AuthError"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("enum AppError {\n    DatabaseError,\n    AuthError,\n}"),
            hints: &[
                "It's just a standard enum with two variants.",
            ],
        },
        success_message: "Great! This is the foundation of robust Rust applications.",
    },
    TutorialModule {
        id: "errors-9-concept",
        title: "9. Concept: The `From` Trait",
        module_type: ModuleType::Concept,
        content: r#"
# Auto-Converting Errors

The real magic of the `?` operator is that it automatically converts errors using the `From` trait.

If `File::open` returns an `io::Error`, but your function returns `Result<T, AppError>`, the `?` operator will look for an implementation of `From<io::Error> for AppError`.

If it finds one, it automatically converts the error!

```rust
enum AppError {
    Io(std::io::Error),
}

// Implement conversion
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}
```
        "#,
        initial_code: "// Read about the From trait, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "errors-10-practice",
        title: "10. Practice: Implement `From`",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Error Conversion

### Task:
Implement the `From` trait for `AppError`. Convert a `std::io::Error` into the `AppError::Io` variant.
        "#,
        initial_code: "enum AppError {\n    Io(std::io::Error),\n}\n\n// Implement `From<std::io::Error> for AppError`\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "implement `From`",
                    matcher: RuleMatcher::Regex(r#"impl\s+From<\s*std::io::Error\s*>\s+for\s+AppError"#),
                },
                ValidationRule {
                    label: "define `fn from(err: std::io::Error) -> Self`",
                    matcher: RuleMatcher::Regex(r#"fn\s+from\(\s*\w+\s*:\s*std::io::Error\s*\)\s*->\s*(?:Self|AppError)"#),
                },
                ValidationRule {
                    label: "wrap in `AppError::Io`",
                    matcher: RuleMatcher::AnyContains(&["AppError::Io(", "Self::Io("]),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl From<std::io::Error> for AppError {\n    fn from(err: std::io::Error) -> Self {\n        AppError::Io(err)\n    }\n}"),
            hints: &[
                "The block should be `impl From<std::io::Error> for AppError { ... }`",
                "Inside, write `fn from(err: std::io::Error) -> Self { AppError::Io(err) }`",
            ],
        },
        success_message: "Incredible! Your app can now magically convert `io::Error` into `AppError` whenever you use the `?` operator.",
    },
];
