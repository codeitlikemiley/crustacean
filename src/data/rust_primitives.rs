use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "prim-1-concept",
        title: "1. Concept: Scalar Types",
        module_type: ModuleType::Concept,
        content: r#"
# Scalar Types in Rust

A **scalar** type represents a single value. Rust has four primary scalar types:
- **Integers**: e.g., `i32`, `u8`, `isize`.
- **Floating-point numbers**: `f32` and `f64`.
- **Booleans**: `bool` (`true` or `false`).
- **Characters**: `char` (four bytes, represents a Unicode Scalar Value).

### Key Ideas
Rust is statically typed, which means it must know the types of all variables at compile time. However, it can usually infer the type based on the value!
        "#,
        initial_code: "// Read the lesson on the left.\n// Click ACKNOWLEDGE to continue.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "prim-2-practice",
        title: "2. Practice: Scalar Bindings",
        module_type: ModuleType::Practice,
        content: r#"
# Declare Scalar Values

### Task:
1. Create a variable `temperature` bound to the float `98.6`
2. Create a variable `is_fever` bound to the boolean `true`
        "#,
        initial_code: "// Declare temperature and is_fever here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "bind `temperature` to 98.6",
                    matcher: RuleMatcher::FlexContains("let temperature = 98.6;"),
                },
                ValidationRule {
                    label: "bind `is_fever` to true",
                    matcher: RuleMatcher::FlexContains("let is_fever = true;"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("let temperature = 98.6;\nlet is_fever = true;"),
            hints: &[
                "Use `let` to bind the variables.",
                "Booleans are lowercase `true` in Rust.",
            ],
        },
        success_message: "Perfect! You've declared a floating point and a boolean.",
    },
    TutorialModule {
        id: "prim-3-concept",
        title: "3. Concept: Compound Types",
        module_type: ModuleType::Concept,
        content: r#"
# Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types:

### Tuples
- Group values with potentially **different** types.
- Fixed length.
- Example: `let tup: (i32, f64, u8) = (500, 6.4, 1);`

### Arrays
- Group values of the **same** type.
- Fixed length (unlike vectors).
- Example: `let arr = [1, 2, 3, 4, 5];`
        "#,
        initial_code: "// Study Tuples and Arrays, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "prim-4-practice",
        title: "4. Practice: Arrays and Tuples",
        module_type: ModuleType::Practice,
        content: r#"
# Create Compound Types

### Task:
1. Create an array `months` containing two strings: `"Jan"` and `"Feb"`
2. Create a tuple `coordinates` containing two floats: `10.5` and `20.5`
        "#,
        initial_code: "// Create your array and tuple here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "bind `months` to an array with \"Jan\" and \"Feb\"",
                    matcher: RuleMatcher::FlexContains("let months = [\"Jan\", \"Feb\"];"),
                },
                ValidationRule {
                    label: "bind `coordinates` to a tuple with 10.5 and 20.5",
                    matcher: RuleMatcher::FlexContains("let coordinates = (10.5, 20.5);"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("let months = [\"Jan\", \"Feb\"];\nlet coordinates = (10.5, 20.5);"),
            hints: &[
                "Arrays use square brackets `[]`.",
                "Tuples use parentheses `()`.",
            ],
        },
        success_message: "Great! You understand how to group data safely in Rust.",
    },
];
