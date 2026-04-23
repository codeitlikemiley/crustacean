use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{RuleMatcher, ValidationRule, ValidationSpec, NormalizeOptions};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "thiserror_concept_1",
        module_type: ModuleType::Concept,
        title: "The thiserror Crate",
        content: "When writing Rust libraries, you should implement the `std::error::Error` trait for your custom error types. However, writing `Display` and `Error::source` by hand is tedious. The `thiserror` crate provides a convenient derive macro that automatically implements `Display` and `Error` for your types.

Use `thiserror` for library errors (where you need to be highly specific), and `anyhow` for application errors (where you just want to pass the error up and log it).

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error(\"io error: {0}\")]
    Io(std::io::Error),
    #[error(\"invalid format\")]
    FormatError,
}
```",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Awesome! Let's practice writing `thiserror` enums.",
    },
    TutorialModule {
        id: "thiserror_practice_1",
        module_type: ModuleType::Practice,
        title: "Derive Error and Display",
        content: "Let's create a custom error enum for a network client.

Use `thiserror` to derive `Error` and add `#[error(\"...\")]` attributes to define the `Display` output.

1. Derive `Error` and `Debug` on `NetworkError`.
2. For the `Timeout` variant, output `\"connection timed out after {0} ms\"`.
3. For the `Unreachable` variant, output `\"host unreachable\"`.",
        initial_code: "use thiserror::Error;

pub enum NetworkError {
    Timeout(u64),
    Unreachable,
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Derive Error and Debug",
                    matcher: RuleMatcher::Regex(r#"#\[derive\([^)]*(Debug[^)]*Error|Error[^)]*Debug)\)\]"#),
                },
                ValidationRule {
                    label: "Add #[error] for Timeout",
                    matcher: RuleMatcher::Contains(r#"#[error("connection timed out after {0} ms")]"#),
                },
                ValidationRule {
                    label: "Add #[error] for Unreachable",
                    matcher: RuleMatcher::Contains(r#"#[error("host unreachable")]"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use thiserror::Error;\n\n#[derive(Debug, Error)]\npub enum NetworkError {\n    #[error(\"connection timed out after {0} ms\")]\n    Timeout(u64),\n    #[error(\"host unreachable\")]\n    Unreachable,\n}"),
            hints: &[
                "Make sure to `#[derive(Debug, Error)]` on the enum.",
                "Use `#[error(\"connection timed out after {0} ms\")]` on the `Timeout` variant."
            ],
        },
        success_message: "Great! `thiserror` generates the `Display` implementation using your format strings.",
    },
    TutorialModule {
        id: "thiserror_concept_2",
        module_type: ModuleType::Concept,
        title: "Transparent & From Errors",
        content: "Often, your error is just wrapping another underlying error (like `std::io::Error`). `thiserror` provides two powerful attributes for this:

- `#[from]` automatically generates a `From<T>` implementation, so you can use the `?` operator to implicitly convert the underlying error into your enum.
- `#[error(transparent)]` delegates both `Display` and `Error::source` directly to the underlying error.

```rust
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

This means `DatabaseError::Io` will print exactly the same message as `std::io::Error`, and `?` will work automatically!",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's put this into practice.",
    },
    TutorialModule {
        id: "thiserror_practice_2",
        module_type: ModuleType::Practice,
        title: "Wrapping Underlying Errors",
        content: "Update the `AppError` enum to wrap `std::io::Error`.

1. Use `#[error(transparent)]` to delegate the error output.
2. Use `#[from]` on the inner `std::io::Error` field so that `?` works.",
        initial_code: "use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    IoError(std::io::Error),
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Use transparent error",
                    matcher: RuleMatcher::Contains(r#"#[error(transparent)]"#),
                },
                ValidationRule {
                    label: "Use #[from] attribute",
                    matcher: RuleMatcher::Contains(r#"#[from]"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use thiserror::Error;\n\n#[derive(Debug, Error)]\npub enum AppError {\n    #[error(transparent)]\n    IoError(#[from] std::io::Error),\n}"),
            hints: &[
                "Place `#[error(transparent)]` above the `IoError` variant.",
                "Place `#[from]` right before `std::io::Error` inside the tuple variant."
            ],
        },
        success_message: "Excellent! You can now use `thiserror` to build ergonomic and robust custom error types for your libraries.",
    },
];
