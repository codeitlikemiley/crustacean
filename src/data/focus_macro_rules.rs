use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{RuleMatcher, ValidationRule, ValidationSpec, NormalizeOptions};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "macro_rules_concept_1",
        module_type: ModuleType::Concept,
        title: "Declarative Macros",
        content: "Rust's `macro_rules!` lets you write declarative macros. They are essentially powerful pattern-matching engines that operate on tokens, not strings.

A macro has one or more arms, just like a `match` expression:

```rust
macro_rules! say_hello {
    // Match empty input
    () => {
        println!(\"Hello!\");
    };
    // Match an expression and bind it to `$name`
    ($name:expr) => {
        println!(\"Hello, {}!\", $name);
    };
}
```

The `$name:expr` captures any valid Rust expression (like `\"Alice\"` or `1 + 1`) and assigns it to the meta-variable `$name`.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's build a simple macro.",
    },
    TutorialModule {
        id: "macro_rules_practice_1",
        module_type: ModuleType::Practice,
        title: "Building the ok! macro",
        content: "Let's write an `ok!` macro that wraps an expression in `Result::Ok`.

1. Add a pattern to match a single expression and bind it to `$val:expr`.
2. The expansion (right side of `=>`) should return `Ok($val)`.

*Note: Since it's a single expression, you can just use `Ok($val)` without braces.*",
        initial_code: "macro_rules! ok {
    // 1. Match an expression `$val:expr`
    // 2. Expand to `Ok($val)`
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Match $val:expr",
                    matcher: RuleMatcher::Regex(r#"\(\s*\$val\s*:\s*expr\s*\)\s*=>"#),
                },
                ValidationRule {
                    label: "Expand to Ok($val)",
                    matcher: RuleMatcher::Regex(r#"=>\s*\{\s*Ok\s*\(\s*\$val\s*\)\s*\}"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("macro_rules! ok {\n    ($val:expr) => { Ok($val) };\n}"),
            hints: &[
                "The arm should look like: `($val:expr) => { Ok($val) };`"
            ],
        },
        success_message: "Nice! You've written your first macro. Now let's handle repeating patterns.",
    },
    TutorialModule {
        id: "macro_rules_concept_2",
        module_type: ModuleType::Concept,
        title: "Repetition in Macros",
        content: "Macros can match repeating patterns using `$( ... )*` (zero or more) or `$( ... )+` (one or more). You can also specify a separator, like a comma: `$( ... ),*`.

```rust
macro_rules! sum {
    ( $( $x:expr ),* ) => {
        {
            let mut temp = 0;
            $(
                temp += $x;
            )*
            temp
        }
    };
}
```

In the expansion, `$( temp += $x; )*` will stamp out the inner code once for every `$x` that was matched. Notice how `sum!(1, 2, 3)` turns into `temp += 1; temp += 2; temp += 3;`.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's build the famous hashmap! macro.",
    },
    TutorialModule {
        id: "macro_rules_practice_2",
        module_type: ModuleType::Practice,
        title: "Building hashmap!",
        content: "Let's build a `hashmap!` macro that lets us create a map like this:
`hashmap!( \"A\" => 1, \"B\" => 2 )`

1. Match repeating pairs: `$key:expr => $val:expr` separated by commas. Wait, the `=>` is part of the syntax!
2. In the expansion, stamp out `map.insert($key, $val);` for each pair.

*The initialization code is already written for you.*",
        initial_code: "macro_rules! hashmap {
    (
        // 1. Match repeating key => value pairs separated by commas
    ) => {
        {
            let mut map = std::collections::HashMap::new();
            // 2. Repeat map.insert($key, $val);
            map
        }
    };
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Match repeating pairs",
                    matcher: RuleMatcher::Regex(r#"\(\s*\$\(\s*\$key\s*:\s*expr\s*=>\s*\$val\s*:\s*expr\s*\)\s*,\s*\*\s*\)"#),
                },
                ValidationRule {
                    label: "Stamp out inserts",
                    matcher: RuleMatcher::Regex(r#"\$\(\s*map\.insert\s*\(\s*\$key\s*,\s*\$val\s*\)\s*;\s*\)\*"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("macro_rules! hashmap {\n    ( $( $key:expr => $val:expr ),* ) => {\n        {\n            let mut map = std::collections::HashMap::new();\n            $( map.insert($key, $val); )*\n            map\n        }\n    };\n}"),
            hints: &[
                "The match pattern should be `( $( $key:expr => $val:expr ),* )`",
                "The expansion should use `$( map.insert($key, $val); )*`"
            ],
        },
        success_message: "Fantastic! You've mastered declarative macros and repetition. This is exactly how `vec!` is implemented in the standard library.",
    },
];
