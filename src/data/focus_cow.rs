use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fcow-1-concept",
        title: "1. What is Cow?",
        module_type: ModuleType::Concept,
        content: r#"
# Clone on Write

`Cow<'a, T>` is an enum that holds either a borrowed reference *or* an owned value. It only clones when you need to mutate.

```rust
use std::borrow::Cow;

enum Cow<'a, B: ToOwned + ?Sized> {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

For strings: `Cow<'a, str>` can hold either `&'a str` or `String`.

**When to use:** Functions that *usually* return input unchanged, but *sometimes* need to modify it.
        "#,
        initial_code: "// Study Cow, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fcow-2-practice",
        title: "2. Practice: Conditional Clone",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Sanitize Input

### Task:
Write a function `sanitize` that takes `input: &str` and returns `Cow<str>`. If the input contains `<`, return `Cow::Owned(input.replace("<", "&lt;"))`. Otherwise return `Cow::Borrowed(input)`.
        "#,
        initial_code: "use std::borrow::Cow;\n\n// Write `fn sanitize` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "return type `Cow<str>`",
                    matcher: RuleMatcher::Contains("-> Cow<str>"),
                },
                ValidationRule {
                    label: "check for `<`",
                    matcher: RuleMatcher::Contains("contains("),
                },
                ValidationRule {
                    label: "return Cow::Owned",
                    matcher: RuleMatcher::Contains("Cow::Owned("),
                },
                ValidationRule {
                    label: "return Cow::Borrowed",
                    matcher: RuleMatcher::Contains("Cow::Borrowed("),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn sanitize(input: &str) -> Cow<str> {\n    if input.contains('<') {\n        Cow::Owned(input.replace(\"<\", \"&lt;\"))\n    } else {\n        Cow::Borrowed(input)\n    }\n}"),
            hints: &[
                "Signature: `fn sanitize(input: &str) -> Cow<str>`",
                "Only allocate when the input actually needs modification.",
            ],
        },
        success_message: "Zero allocations for clean input, allocation only when needed!",
    },
    TutorialModule {
        id: "fcow-3-concept",
        title: "3. Cow in APIs",
        module_type: ModuleType::Concept,
        content: r#"
# Accepting Cow in Function Parameters

`Cow` implements `Deref`, so you can use it wherever `&str` or `&[T]` is expected. This makes it great for function return values.

For parameters, prefer `&str` or `impl Into<String>` instead. `Cow` shines as a **return type** from functions that conditionally allocate.

```rust
fn process(input: &str) -> Cow<str> {
    if input.is_empty() {
        Cow::Borrowed("default")
    } else {
        Cow::Owned(input.to_uppercase())
    }
}

// Cow<str> auto-derefs to &str
let result = process("hello");
println!("{}", result); // "HELLO"
```
        "#,
        initial_code: "// Study Cow in APIs, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fcow-4-practice",
        title: "4. Practice: to_mut()",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Lazy Mutation

### Task:
Use `cow.to_mut()` to get a mutable reference and push `"!!!"` onto the string. `to_mut()` will clone if currently borrowed.
        "#,
        initial_code: "use std::borrow::Cow;\n\nfn shout(cow: &mut Cow<str>) {\n    // Use to_mut() to push_str \"!!!\"\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "call `cow.to_mut()`",
                    matcher: RuleMatcher::Contains("cow.to_mut()"),
                },
                ValidationRule {
                    label: "push_str",
                    matcher: RuleMatcher::Contains(".push_str("),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn shout(cow: &mut Cow<str>) {\n    cow.to_mut().push_str(\"!!!\");\n}"),
            hints: &[
                "`cow.to_mut()` returns `&mut String`.",
                "Chain it: `cow.to_mut().push_str(\"!!!\");`",
            ],
        },
        success_message: "Lazy cloning — only allocates when mutation actually happens!",
    },
];
