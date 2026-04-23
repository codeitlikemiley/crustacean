use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "ffi-1-concept",
        title: "1. The From Trait",
        module_type: ModuleType::Concept,
        content: r#"
# `From<T>` — Infallible Conversion

`From` is the standard way to convert one type into another when the conversion can never fail.

```rust
impl From<i32> for MyNumber {
    fn from(val: i32) -> Self {
        MyNumber(val)
    }
}

let n = MyNumber::from(42);
```

**Key insight:** When you implement `From<A> for B`, you automatically get `Into<B> for A` for free!
        "#,
        initial_code: "// Study From<T>, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "ffi-2-practice",
        title: "2. Practice: Implement From",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `From<&str>` for `Greeting`

### Task:
Implement `From<&str>` for `Greeting`. The `from` method should wrap the input in a `Greeting` using `String::from(s)`.
        "#,
        initial_code: "struct Greeting(String);\n\n// Implement From<&str> for Greeting\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "impl From<&str> for Greeting",
                    matcher: RuleMatcher::OrderedContains(&["impl", "From<&str>", "for Greeting"]),
                },
                ValidationRule {
                    label: "define fn from",
                    matcher: RuleMatcher::Contains("fn from("),
                },
                ValidationRule {
                    label: "wrap in Greeting",
                    matcher: RuleMatcher::AnyContains(&["Greeting(String::from(", "Greeting(s.to_string()", "Greeting(s.to_owned()"]),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl From<&str> for Greeting {\n    fn from(s: &str) -> Self {\n        Greeting(String::from(s))\n    }\n}"),
            hints: &[
                "Start with `impl From<&str> for Greeting {`",
                "The body should be `Greeting(String::from(s))`.",
            ],
        },
        success_message: "Now anyone can write `Greeting::from(\"hi\")` or `\"hi\".into()`!",
    },
    TutorialModule {
        id: "ffi-3-concept",
        title: "3. Into and Generic Bounds",
        module_type: ModuleType::Concept,
        content: r#"
# `Into<T>` — The Caller's Perspective

While `From` is implemented on the *target* type, `Into` is called from the *source* side.

The real power of `Into` shows up in **function signatures** that accept anything convertible:

```rust
// This function accepts &str, String, or anything else
// that implements Into<String>!
fn greet(name: impl Into<String>) {
    let name = name.into();
    println!("Hello, {}!", name);
}

greet("Alice");                    // &str → String
greet(String::from("Bob"));        // String → String (no-op)
```

**Rule:** Prefer implementing `From` (you get `Into` for free). Prefer *using* `Into` in function bounds for ergonomic APIs.
        "#,
        initial_code: "// Study Into<T> usage, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "ffi-4-practice",
        title: "4. Practice: impl Into<String>",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Flexible API

### Task:
Write a function `set_title` that accepts `title: impl Into<String>` and stores it in a `let _title: String = title.into();`.
        "#,
        initial_code: "// Write `fn set_title` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn set_title`",
                    matcher: RuleMatcher::Regex(r#"fn\s+set_title"#),
                },
                ValidationRule {
                    label: "accept `impl Into<String>`",
                    matcher: RuleMatcher::Contains("impl Into<String>"),
                },
                ValidationRule {
                    label: "call `.into()`",
                    matcher: RuleMatcher::Contains(".into()"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn set_title(title: impl Into<String>) {\n    let _title: String = title.into();\n}"),
            hints: &[
                "Signature: `fn set_title(title: impl Into<String>)`",
                "Inside: `let _title: String = title.into();`",
            ],
        },
        success_message: "Your API now accepts both &str and String seamlessly!",
    },
    TutorialModule {
        id: "ffi-5-concept",
        title: "5. TryFrom / TryInto",
        module_type: ModuleType::Concept,
        content: r#"
# Fallible Conversion

What if the conversion can fail? Use `TryFrom` / `TryInto`.

```rust
use std::convert::TryFrom;

struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(format!("{} is not even", value))
        }
    }
}

let ok = EvenNumber::try_from(8);   // Ok(EvenNumber(8))
let err = EvenNumber::try_from(5);  // Err("5 is not even")
```

`TryFrom` is used extensively for parsing, validation, and narrowing conversions.
        "#,
        initial_code: "// Study TryFrom, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "ffi-6-practice",
        title: "6. Practice: Implement TryFrom",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Positive Number

### Task:
Implement `TryFrom<i32>` for `Positive`. Return `Ok(Positive(value))` if value > 0, otherwise return `Err("must be positive")`.
        "#,
        initial_code: "struct Positive(i32);\n\n// Implement TryFrom<i32> for Positive\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "impl TryFrom<i32> for Positive",
                    matcher: RuleMatcher::OrderedContains(&["impl", "TryFrom<i32>", "for Positive"]),
                },
                ValidationRule {
                    label: "define `type Error`",
                    matcher: RuleMatcher::Contains("type Error"),
                },
                ValidationRule {
                    label: "return Ok(Positive(value))",
                    matcher: RuleMatcher::Contains("Ok(Positive("),
                },
                ValidationRule {
                    label: "return Err for invalid",
                    matcher: RuleMatcher::Contains("Err("),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl TryFrom<i32> for Positive {\n    type Error = &'static str;\n    fn try_from(value: i32) -> Result<Self, Self::Error> {\n        if value > 0 {\n            Ok(Positive(value))\n        } else {\n            Err(\"must be positive\")\n        }\n    }\n}"),
            hints: &[
                "Set `type Error = &'static str;`",
                "Use `if value > 0 { Ok(Positive(value)) } else { Err(\"must be positive\") }`.",
            ],
        },
        success_message: "Now callers get a clear error when conversion fails!",
    },
];
