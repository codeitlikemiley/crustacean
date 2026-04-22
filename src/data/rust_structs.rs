use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "structs-1-concept",
        title: "1. Concept: Structs",
        module_type: ModuleType::Concept,
        content: r#"
# Defining Structs

A struct (structure) is a custom data type that lets you package together related values into a single entity. 

Unlike a tuple, you name each piece of data (called a **field**) so you don't have to rely on order.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

To create an instance, you provide the values in curly braces:
```rust
let user1 = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    sign_in_count: 1,
    active: true,
};
```
        "#,
        initial_code: "// Learn about structs, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "structs-2-practice",
        title: "2. Practice: Define a Struct",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `Rectangle` Struct

### Task:
Define a struct named `Rectangle` that has two fields:
- `width` of type `u32`
- `height` of type `u32`
        "#,
        initial_code: "// Define the `Rectangle` struct here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `struct Rectangle`",
                    matcher: RuleMatcher::Regex(r#"struct\s+Rectangle"#),
                },
                ValidationRule {
                    label: "field `width: u32`",
                    matcher: RuleMatcher::Regex(r#"width\s*:\s*u32"#),
                },
                ValidationRule {
                    label: "field `height: u32`",
                    matcher: RuleMatcher::Regex(r#"height\s*:\s*u32"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("struct Rectangle {\n    width: u32,\n    height: u32,\n}"),
            hints: &[
                "Start with `struct Rectangle {`",
                "Fields inside should look like `width: u32,` (don't forget the comma!).",
            ],
        },
        success_message: "Great! You defined a data shape.",
    },
    TutorialModule {
        id: "structs-3-concept",
        title: "3. Concept: Methods and `impl`",
        module_type: ModuleType::Concept,
        content: r#"
# Methods in `impl` Blocks

Methods are similar to functions, but they are tied to a specific struct and their first parameter is always `self`.

You define them inside an `impl` (implementation) block.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self is short for `self: &Self` (a reference to the struct)
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
```

You call methods using dot notation: `rect.is_square()`
        "#,
        initial_code: "// Read about methods, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "structs-4-practice",
        title: "4. Practice: Add a Method",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: The `area` Method

### Task:
Add an `area` method to the `Rectangle` implementation. It should take `&self` and return a `u32` representing `width * height`.
        "#,
        initial_code: "struct Rectangle {\n    width: u32,\n    height: u32,\n}\n\nimpl Rectangle {\n    // Define `fn area(&self) -> u32` here\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn area`",
                    matcher: RuleMatcher::Regex(r#"fn\s+area"#),
                },
                ValidationRule {
                    label: "accept `&self`",
                    matcher: RuleMatcher::Regex(r#"\(&self\)"#),
                },
                ValidationRule {
                    label: "return type `-> u32`",
                    matcher: RuleMatcher::Regex(r#"->\s*u32"#),
                },
                ValidationRule {
                    label: "calculate `self.width * self.height`",
                    matcher: RuleMatcher::Regex(r#"self\.width\s*\*\s*self\.height"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl Rectangle {\n    fn area(&self) -> u32 {\n        self.width * self.height\n    }\n}"),
            hints: &[
                "The signature should be `fn area(&self) -> u32`.",
                "Return `self.width * self.height` implicitly (no semicolon).",
            ],
        },
        success_message: "Perfect! You've attached behavior to your data.",
    },
    TutorialModule {
        id: "structs-5-concept",
        title: "5. Concept: Associated Functions",
        module_type: ModuleType::Concept,
        content: r#"
# Associated Functions (Constructors)

Functions inside an `impl` block that do **not** take `self` as a parameter are called associated functions. 

They are often used for constructors that will return a new instance of the struct, similar to `String::from()`.

```rust
impl Rectangle {
    // Self (capital S) refers to the type (Rectangle)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

You call them using `::` syntax: `let sq = Rectangle::square(3);`
        "#,
        initial_code: "// Study associated functions, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "structs-6-practice",
        title: "6. Practice: Write a Constructor",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `new` Constructor

### Task:
Write an associated function named `new` inside the `impl Rectangle` block. 
It should take `width: u32` and `height: u32`, and return `Self`.
*Hint:* You can use field init shorthand (just `width, height,` instead of `width: width, height: height,`).
        "#,
        initial_code: "struct Rectangle {\n    width: u32,\n    height: u32,\n}\n\nimpl Rectangle {\n    // Define `fn new(width: u32, height: u32) -> Self` here\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn new`",
                    matcher: RuleMatcher::Regex(r#"fn\s+new"#),
                },
                ValidationRule {
                    label: "return type `-> Self` or `-> Rectangle`",
                    matcher: RuleMatcher::AnyContains(&["-> Self", "-> Rectangle"]),
                },
                ValidationRule {
                    label: "instantiate the struct",
                    matcher: RuleMatcher::AnyContains(&["Self {", "Rectangle {"]),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "must not take `&self`",
                    matcher: RuleMatcher::Regex(r#"fn\s+new\s*\(\s*&self"#),
                },
            ],
            canonical_solution: Some("impl Rectangle {\n    fn new(width: u32, height: u32) -> Self {\n        Self {\n            width,\n            height,\n        }\n    }\n}"),
            hints: &[
                "Signature: `fn new(width: u32, height: u32) -> Self`",
                "Return `Self { width, height }`",
            ],
        },
        success_message: "Great! You built a constructor function.",
    },
    TutorialModule {
        id: "structs-7-concept",
        title: "7. Concept: Tuple Structs",
        module_type: ModuleType::Concept,
        content: r#"
# Tuple Structs

Tuple structs look like a hybrid between a tuple and a struct. They have a name, but their fields do not.

They are useful when you want to give a tuple a distinct type identity without the verbosity of naming every field.

```rust
// Two distinct types, even though both hold three i32s!
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// Access fields using indices
let r = black.0;
```
        "#,
        initial_code: "// Read about tuple structs, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "structs-8-practice",
        title: "8. Practice: Define a Tuple Struct",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `Color` Type

### Task:
Define a tuple struct named `Color` that holds three `u8` values (representing RGB).
        "#,
        initial_code: "// Define `struct Color` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `struct Color`",
                    matcher: RuleMatcher::Regex(r#"struct\s+Color"#),
                },
                ValidationRule {
                    label: "three `u8` fields",
                    matcher: RuleMatcher::Regex(r#"\(\s*u8\s*,\s*u8\s*,\s*u8\s*\)"#),
                },
                ValidationRule {
                    label: "end with a semicolon",
                    matcher: RuleMatcher::Regex(r#"\)\s*;"#),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not use curly braces",
                    matcher: RuleMatcher::Contains("{"),
                },
            ],
            canonical_solution: Some("struct Color(u8, u8, u8);"),
            hints: &[
                "It should look like `struct Name(Type, Type);`",
                "Don't forget the semicolon at the end of a tuple struct definition!",
            ],
        },
        success_message: "Nice! Tuple structs are great for creating strong wrapper types.",
    },
    TutorialModule {
        id: "structs-9-concept",
        title: "9. Concept: Struct Update Syntax",
        module_type: ModuleType::Concept,
        content: r#"
# Struct Update Syntax

Often, you want to create a new instance of a struct that uses most of the values from an old instance, but changes just one or two fields.

Rust provides the `..` syntax for this.

```rust
let user1 = User {
    email: String::from("alice@example.com"),
    username: String::from("alice"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
    email: String::from("bob@example.com"),
    // Copy the remaining fields from user1
    ..user1
};
```
        "#,
        initial_code: "// Study struct update syntax, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "structs-10-practice",
        title: "10. Practice: Update a Profile",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Update Syntax

### Task:
Create `user2` by changing the `username` to `String::from("bob")` but keeping `email` and `active` the same as `user1` using struct update syntax (`..user1`).
        "#,
        initial_code: "struct User {\n    username: String,\n    email: String,\n    active: bool,\n}\n\nfn make_bob(user1: User) -> User {\n    // Return a new User named bob, copying other fields from user1\n    User {\n        \n    }\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "set `username`",
                    matcher: RuleMatcher::AnyContains(&["username: String::from(\"bob\")", "username: \"bob\".to_string()"]),
                },
                ValidationRule {
                    label: "use update syntax `..user1`",
                    matcher: RuleMatcher::Contains("..user1"),
                },
            ],
            forbidden: &[
                ValidationRule {
                    label: "do not manually copy email or active",
                    matcher: RuleMatcher::Contains("email:"),
                },
            ],
            canonical_solution: Some("fn make_bob(user1: User) -> User {\n    User {\n        username: String::from(\"bob\"),\n        ..user1\n    }\n}"),
            hints: &[
                "Inside the `User { }` block, define the `username` field first.",
                "After the comma, write `..user1` to fill in the rest.",
            ],
        },
        success_message: "Perfect! You've completed the Structs & Methods course.",
    },
];
