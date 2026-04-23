use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fbld-1-concept",
        title: "1. Why Builder Pattern?",
        module_type: ModuleType::Concept,
        content: r#"
# The Builder Pattern

When a struct has many fields (some optional), constructors become unwieldy. The Builder pattern provides a fluent API:

```rust
let server = ServerConfig::builder()
    .host("localhost")
    .port(8080)
    .max_connections(100)
    .build();
```

In Rust, builders are typically separate structs that accumulate configuration, then produce the final struct via a `build()` method.
        "#,
        initial_code: "// Study the builder pattern, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fbld-2-practice",
        title: "2. Practice: Builder Struct",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Define a Builder

### Task:
Define a struct `RequestBuilder` with two fields: `url: String` and `timeout: u64`. Give both default values using `Default` or a `new()` method.
        "#,
        initial_code: "// Define `RequestBuilder`\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `struct RequestBuilder`",
                    matcher: RuleMatcher::Contains("struct RequestBuilder"),
                },
                ValidationRule {
                    label: "field `url: String`",
                    matcher: RuleMatcher::Contains("url: String"),
                },
                ValidationRule {
                    label: "field `timeout: u64`",
                    matcher: RuleMatcher::Contains("timeout: u64"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("struct RequestBuilder {\n    url: String,\n    timeout: u64,\n}"),
            hints: &[
                "A simple struct with two named fields.",
                "The types are `String` and `u64`.",
            ],
        },
        success_message: "Builder struct defined!",
    },
    TutorialModule {
        id: "fbld-3-practice",
        title: "3. Practice: Fluent Setters",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Method Chaining

### Task:
Add a method `url(mut self, url: impl Into<String>) -> Self` to `RequestBuilder` that sets `self.url` and returns `self`. This enables chaining.
        "#,
        initial_code: "struct RequestBuilder {\n    url: String,\n    timeout: u64,\n}\n\nimpl RequestBuilder {\n    // Add `fn url(mut self, ...) -> Self` here\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn url`",
                    matcher: RuleMatcher::Regex(r#"fn\s+url\s*\("#),
                },
                ValidationRule {
                    label: "take `mut self`",
                    matcher: RuleMatcher::Contains("mut self"),
                },
                ValidationRule {
                    label: "return `-> Self`",
                    matcher: RuleMatcher::Contains("-> Self"),
                },
                ValidationRule {
                    label: "return self",
                    matcher: RuleMatcher::Regex(r#"self\s*\}"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl RequestBuilder {\n    fn url(mut self, url: impl Into<String>) -> Self {\n        self.url = url.into();\n        self\n    }\n}"),
            hints: &[
                "Take ownership: `fn url(mut self, url: impl Into<String>) -> Self`",
                "Set the field and return `self`.",
            ],
        },
        success_message: "Now you can chain: `.url(\"...\").timeout(30)`!",
    },
    TutorialModule {
        id: "fbld-4-practice",
        title: "4. Practice: build()",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: The `build()` Method

### Task:
Add a `build(self) -> Request` method that consumes the builder and returns a `Request` struct with `url` and `timeout` fields.
        "#,
        initial_code: "struct Request {\n    url: String,\n    timeout: u64,\n}\n\nstruct RequestBuilder {\n    url: String,\n    timeout: u64,\n}\n\nimpl RequestBuilder {\n    // Add `fn build(self) -> Request`\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn build`",
                    matcher: RuleMatcher::Contains("fn build(self)"),
                },
                ValidationRule {
                    label: "return `-> Request`",
                    matcher: RuleMatcher::Contains("-> Request"),
                },
                ValidationRule {
                    label: "construct Request",
                    matcher: RuleMatcher::Contains("Request {"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl RequestBuilder {\n    fn build(self) -> Request {\n        Request {\n            url: self.url,\n            timeout: self.timeout,\n        }\n    }\n}"),
            hints: &[
                "Signature: `fn build(self) -> Request`",
                "Transfer the builder fields into the `Request` struct.",
            ],
        },
        success_message: "Builder pattern complete! Fluent, safe, and idiomatic.",
    },
];
