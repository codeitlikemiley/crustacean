use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);
const COMMENT_TOLERANT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(true, false);
const PUNCT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, true);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "m1-concept",
        title: "1. Mental Model: Traits as Contracts",
        module_type: ModuleType::Concept,
        content: r#"
# Traits: The "Capability" Label

In Rust, a trait is a **contract of capabilities**.

It defines *what* a type can do, without caring *how* it does it.
Think of a "Driver" license. The DMV doesn't care if you're a human or a robot; if you can steer and brake, you get the license.

### The Four Jobs of Traits:
- **Behavior**: Defining what a type can do.
- **Constraints**: Telling functions "Only accept types that can do X."
- **Static Dispatch**: Making code lightning fast at compile-time.
- **Dynamic Dispatch**: Allowing flexible "mixed-type" lists at runtime.

**Mental Hook**: Read `trait Speak` as "Something that has the capability to speak."
        "#,
        initial_code: "// Concept: Study the theory on the left.\n// Click \"Next Phase\" to try the exercise!",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m2-practice",
        title: "2. Practice: Defining a Trait",
        module_type: ModuleType::Practice,
        content: r#"
# Defining the Contract

Use the `trait` keyword. Inside, write **method signatures**.
No body (`{}`), just the semicolon.

### Task:
Define a trait named **`Speak`**.
It should have one method:
- Name: `say_hello`
- Parameters: `&self`
- Return type: `String`
        "#,
        initial_code: "// Define your trait here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define a trait named Speak",
                    matcher: RuleMatcher::Contains("trait Speak"),
                },
                ValidationRule {
                    label: "add the say_hello signature returning String",
                    matcher: RuleMatcher::Regex(
                        r"fn\s+say_hello\s*\(\s*&self\s*\)\s*->\s*String\s*;",
                    ),
                },
            ],
            forbidden: &[],
            canonical_solution: Some(
                "trait Speak {\n    fn say_hello(&self) -> String;\n}",
            ),
            hints: &[
                "Declare the trait first, then place the method signature inside braces.",
                "This lesson wants a signature only, so the method should end with a semicolon.",
            ],
        },
        success_message: "Perfect! You've defined a contract signature.",
    },
    TutorialModule {
        id: "m3-concept",
        title: "3. Concept: The \"impl\" Block",
        module_type: ModuleType::Concept,
        content: r#"
# Fulfilling the Promise

Defining a trait is useless unless a type **implements** it.

When you write `impl Trait for Type`, you are saying: "This type now officially has this capability."

### Syntax:
```rust
struct Dog;
impl Speak for Dog {
    fn say_hello(&self) -> String {
        "Woof".to_string()
    }
}
```
        "#,
        initial_code: "// Concept: Study the implementation syntax.",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m4-practice",
        title: "4. Practice: Attaching Capability",
        module_type: ModuleType::Practice,
        content: r#"
# Implementation Practice

### Task:
1. We've provided a struct `Cat`.
2. Implement the `Speak` trait for `Cat`.
3. Return `"Meow!".to_string()` in the method.
        "#,
        initial_code: "trait Speak { fn say_hello(&self) -> String; }\nstruct Cat;\n\n// Implement Speak for Cat here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "implement Speak for Cat",
                    matcher: RuleMatcher::Contains("impl Speak for Cat"),
                },
                ValidationRule {
                    label: "define say_hello with a String return type",
                    matcher: RuleMatcher::Regex(
                        r"fn\s+say_hello\s*\(\s*&self\s*\)\s*->\s*String",
                    ),
                },
                ValidationRule {
                    label: "return the Meow greeting",
                    matcher: RuleMatcher::AnyContains(&[
                        "\"Meow!\".to_string()",
                        "String::from(\"Meow!\")",
                    ]),
                },
            ],
            forbidden: &[],
            canonical_solution: Some(
                "trait Speak { fn say_hello(&self) -> String; }\nstruct Cat;\n\nimpl Speak for Cat {\n    fn say_hello(&self) -> String {\n        \"Meow!\".to_string()\n    }\n}",
            ),
            hints: &[
                "The impl header should read `impl Speak for Cat`.",
                "You can build the String with either `.to_string()` or `String::from(...)`.",
            ],
        },
        success_message: "Excellent! The Cat now fulfills the Speak contract.",
    },
    TutorialModule {
        id: "m5-concept",
        title: "5. Concept: Static Dispatch",
        module_type: ModuleType::Concept,
        content: r#"
# Generics & Monomorphization

When you use `fn f<T: Speak>(x: T)`, Rust uses **Static Dispatch**.

### The Compiler's Secret:
If you call `f(Dog)` and `f(Cat)`, the compiler writes **two separate versions** of the function for you at compile-time.
- Version 1: `f_for_dog(x: Dog)`
- Version 2: `f_for_cat(x: Cat)`

**Result**: Zero runtime cost. It's as fast as calling the method directly.
        "#,
        initial_code: "// fn talk<T: Speak>(item: T) { ... }",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m6-practice",
        title: "6. Practice: Trait Bounds",
        module_type: ModuleType::Practice,
        content: r#"
# Generic Constraints

Trait bounds tell the compiler: "I don't care what type `T` is, as long as it can `Speak`."

### Task:
Write a generic function `greet_it<T>`.
- Use a **trait bound** to ensure `T` implements `Speak`.
- Take one argument `item: T`.
- Call `item.say_hello()` inside.
        "#,
        initial_code: "trait Speak { fn say_hello(&self) -> String; }\n\n// Write greet_it<T: Speak> here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "write greet_it with a T: Speak bound",
                    matcher: RuleMatcher::Regex(
                        r"fn\s+greet_it\s*<\s*T\s*:\s*Speak\s*>\s*\(\s*item\s*:\s*T\s*\)",
                    ),
                },
                ValidationRule {
                    label: "call item.say_hello() inside the function",
                    matcher: RuleMatcher::Contains("item.say_hello()"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some(
                "trait Speak { fn say_hello(&self) -> String; }\n\nfn greet_it<T: Speak>(item: T) {\n    println!(\"{}\", item.say_hello());\n}",
            ),
            hints: &[
                "Place the trait bound directly on `T` inside the angle brackets.",
                "The body only needs to call `item.say_hello()` somewhere.",
            ],
        },
        success_message: "Static dispatch verified! The compiler will now generate specialized versions of this function.",
    },
    TutorialModule {
        id: "m7-concept",
        title: "7. Concept: Dynamic Dispatch (dyn)",
        module_type: ModuleType::Concept,
        content: r#"
# Erasing Types at Runtime

Sometimes you need a list of different types (e.g., `[Dog, Cat, Robot]`). Since they are different sizes, a standard `Vec` can't hold them.

We use **Trait Objects** (`dyn Trait`) behind a pointer (`&` or `Box`).

### The Vtable:
Rust stores a "vtable" (virtual method table) next to the data. It looks up which method to call at **runtime**.

**Syntax**: `&dyn Speak` or `Box<dyn Speak>`.
        "#,
        initial_code: "// Trait Objects: &dyn Speak",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m8-practice",
        title: "8. Practice: Using dyn",
        module_type: ModuleType::Practice,
        content: r#"
# Trait Object Practice

### Task:
Complete the function signature for `process_speakers`.
- It should take a slice of **trait object references**: `&[&dyn Speak]`.
        "#,
        initial_code: "trait Speak { fn say_hello(&self) -> String; }\n\nfn process_speakers(list: /* type here */) {\n    for item in list {\n        println!(\"{}\", item.say_hello());\n    }\n}",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[ValidationRule {
                label: "use &[&dyn Speak] for the list type",
                matcher: RuleMatcher::Regex(r#"list\s*:\s*&\[\s*&dyn\s+Speak\s*\]"#),
            }],
            forbidden: &[],
            canonical_solution: Some(
                "trait Speak { fn say_hello(&self) -> String; }\n\nfn process_speakers(list: &[&dyn Speak]) {\n    for item in list {\n        println!(\"{}\", item.say_hello());\n    }\n}",
            ),
            hints: &[
                "The outer `&[...]` makes the parameter a borrowed slice.",
                "Each item inside the slice should be a trait object reference: `&dyn Speak`.",
            ],
        },
        success_message: "Correct! This allows the function to handle mixed types at runtime.",
    },
    TutorialModule {
        id: "m9-concept",
        title: "9. Concept: FTC Anatomy",
        module_type: ModuleType::Concept,
        content: r#"
# Functions, Types, and Constants

Traits can contain more than just methods. Remember **FTC**:
1. **F**unctions: Standard methods.
2. **T**ypes: Associated types (`type Item`).
3. **C**onstants: Associated constants (`const ID: u32`).

Associated constants are perfect for embedding metadata directly into the capability contract.
        "#,
        initial_code: "// trait FTC { const C: i32; type T; fn f(); }",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m10-practice",
        title: "10. Practice: Associated Constants",
        module_type: ModuleType::Practice,
        content: r#"
# FTC Practice

### Task:
Add an associated constant to the `Identify` trait.
- Name: `VERSION`
- Type: `u32`
- Default Value: `1`
        "#,
        initial_code: "trait Identify {\n    // Add associated constant VERSION here\n}",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[ValidationRule {
                label: "add const VERSION: u32 = 1;",
                matcher: RuleMatcher::Regex(r#"const\s+VERSION\s*:\s*u32\s*=\s*1\s*;"#),
            }],
            forbidden: &[],
            canonical_solution: Some(
                "trait Identify {\n    const VERSION: u32 = 1;\n}",
            ),
            hints: &[
                "Associated constants live directly inside the trait body.",
                "This one needs both a type annotation and a default value.",
            ],
        },
        success_message: "Anatomy mastered! FTC makes traits very expressive.",
    },
    TutorialModule {
        id: "m11-concept",
        title: "11. Concept: The Orphan Rule",
        module_type: ModuleType::Concept,
        content: r#"
# Coherence & Ownership

Rust forbids you from implementing an **External Trait** for an **External Type**.

### The "No" Scenario:
You cannot write `impl Display for Vec<i32>`.
- You don't own `Display` (it's in `std`).
- You don't own `Vec` (it's in `std`).

**Rule**: You must **Own One Side** of the `impl` block.
        "#,
        initial_code: "// Orphan Rule: Own the Trait OR the Type.",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m12-practice",
        title: "12. Practice: The Newtype Pattern",
        module_type: ModuleType::Practice,
        content: r#"
# Bypassing the Orphan Rule

If you *must* implement a foreign trait for a foreign type, wrap it in a local struct.

### Task:
1. Create a "Newtype" tuple struct named **`MyList`**.
2. It should wrap a `Vec<String>`.
        "#,
        initial_code: "// Create the Newtype wrapper here\n",
        validation: ValidationSpec::Rules {
            normalize: COMMENT_TOLERANT_NORMALIZE,
            required: &[ValidationRule {
                label: "define the tuple struct exactly as MyList(Vec<String>)",
                matcher: RuleMatcher::NormalizedExact("struct MyList(Vec<String>);"),
            }],
            forbidden: &[],
            canonical_solution: Some("struct MyList(Vec<String>);"),
            hints: &[
                "Use tuple-struct syntax, not a named-field struct.",
                "The wrapped type should stay `Vec<String>` exactly.",
            ],
        },
        success_message: "Brilliant. Now you own MyList, so the Orphan Rule no longer applies!",
    },
    TutorialModule {
        id: "m13-concept",
        title: "13. Concept: Associated Types",
        module_type: ModuleType::Concept,
        content: r#"
# The "Slot" Model

Associated types (`type Item`) are like "slots" that an implementor fills.

### Why not generics?
If you used `trait Iterator<T>`, a type could be an iterator of `String` AND an iterator of `i32` at the same time. This is usually confusing.

With `type Item`, a type picks **one** choice for the capability.
        "#,
        initial_code: "// trait Iterator { type Item; ... }",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m14-practice",
        title: "14. Practice: Associated Types",
        module_type: ModuleType::Practice,
        content: r#"
# Filling the Slot

### Task:
1. Add an associated type named **`Output`** to the `Producer` trait.
2. Update the `produce` method to return `Self::Output`.
        "#,
        initial_code: "trait Producer {\n    // 1. Add type slot\n\n    // 2. Change return to Self::Output\n    fn produce(&self) -> i32;\n}",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "add type Output;",
                    matcher: RuleMatcher::Contains("type Output;"),
                },
                ValidationRule {
                    label: "change produce to return Self::Output",
                    matcher: RuleMatcher::Regex(
                        r"fn\s+produce\s*\(\s*&self\s*\)\s*->\s*Self\s*::\s*Output\s*;",
                    ),
                },
            ],
            forbidden: &[ValidationRule {
                label: "remove the old i32 return type",
                matcher: RuleMatcher::Contains("-> i32"),
            }],
            canonical_solution: Some(
                "trait Producer {\n    type Output;\n\n    fn produce(&self) -> Self::Output;\n}",
            ),
            hints: &[
                "Add the associated type as its own line inside the trait body.",
                "The method should return `Self::Output`, not the original `i32`.",
            ],
        },
        success_message: "Correct! Associated types define a unique 'output' for a capability.",
    },
    TutorialModule {
        id: "m15-concept",
        title: "15. Concept: Supertraits",
        module_type: ModuleType::Concept,
        content: r#"
# Interface Dependencies

A Supertrait means: "To implement A, you must already implement B."

Syntax: `trait A: B {}`

This is how Rust expresses requirements. For example, `trait Copy: Clone {}` because you can't have a bitwise copy if you don't even know how to clone!
        "#,
        initial_code: "// trait Child: Parent { }",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m16-practice",
        title: "16. Practice: Supertraits",
        module_type: ModuleType::Practice,
        content: r#"
# Hierarchy Practice

### Task:
Make the **`SmartPhone`** trait require **`Battery`** as a supertrait.
        "#,
        initial_code: "trait Battery {\n    fn level(&self) -> u8;\n}\n\n// Add requirement here\ntrait SmartPhone {\n    fn call(&self);\n}",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[ValidationRule {
                label: "write SmartPhone: Battery",
                matcher: RuleMatcher::Contains("trait SmartPhone: Battery"),
            }],
            forbidden: &[],
            canonical_solution: Some(
                "trait Battery {\n    fn level(&self) -> u8;\n}\n\ntrait SmartPhone: Battery {\n    fn call(&self);\n}",
            ),
            hints: &[
                "Supertraits are declared after a colon in the trait header.",
                "Only the SmartPhone declaration needs to change for this lesson.",
            ],
        },
        success_message: "Exactly! SmartPhone now depends on the Battery capability.",
    },
    TutorialModule {
        id: "m17-concept",
        title: "17. Concept: impl Trait (APIT)",
        module_type: ModuleType::Concept,
        content: r#"
# Anonymous Generics

Writing `fn f<T: Speak>(x: T)` can be long.

**Argument Position impl Trait (APIT)** is just syntactic sugar:
`fn f(x: impl Speak)`

It means the same thing, but it's shorter and easier to read.
        "#,
        initial_code: "// fn f(x: impl Display) vs fn f<T: Display>(x: T)",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m18-practice",
        title: "18. Practice: APIT",
        module_type: ModuleType::Practice,
        content: r#"
# Shortening Generics

### Task:
Convert this function to use **`impl Speak`** syntax for the parameter `subject`.
        "#,
        initial_code: "trait Speak { fn say_hello(&self) -> String; }\n\nfn greet<T: Speak>(subject: T) {\n    println!(\"{}\", subject.say_hello());\n}",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "rewrite the signature to use subject: impl Speak",
                    matcher: RuleMatcher::Regex(
                        r"fn\s+greet\s*\(\s*subject\s*:\s*impl\s+Speak\s*\)",
                    ),
                },
                ValidationRule {
                    label: "keep the say_hello call inside the function",
                    matcher: RuleMatcher::Contains("subject.say_hello()"),
                },
            ],
            forbidden: &[ValidationRule {
                label: "remove the generic parameter version",
                matcher: RuleMatcher::Regex(r#"fn\s+greet\s*<\s*T\s*:\s*Speak\s*>"#),
            }],
            canonical_solution: Some(
                "trait Speak { fn say_hello(&self) -> String; }\n\nfn greet(subject: impl Speak) {\n    println!(\"{}\", subject.say_hello());\n}",
            ),
            hints: &[
                "Drop the `<T: Speak>` generic parameter entirely.",
                "Move the trait requirement onto the argument as `subject: impl Speak`.",
            ],
        },
        success_message: "Beautiful. Less noise, same power.",
    },
    TutorialModule {
        id: "m19-concept",
        title: "19. Concept: RPIT (Return impl Trait)",
        module_type: ModuleType::Concept,
        content: r#"
# Hiding the Concrete Type

Sometimes you return a type (like an iterator or a closure) that is impossible to write out manually.

**Return Position impl Trait (RPIT)** lets you hide it:
`fn f() -> impl Iterator<Item = i32>`

The compiler knows the type, but the caller only knows the trait.
        "#,
        initial_code: "// fn factory() -> impl Speak",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m20-practice",
        title: "20. Practice: RPIT",
        module_type: ModuleType::Practice,
        content: r#"
# Opaque Return Practice

### Task:
Change the return type to **`impl Iterator<Item = u32>`**.
        "#,
        initial_code: "fn evens() -> /* type here */ {\n    (0..10).filter(|x| x % 2 == 0)\n}",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[ValidationRule {
                label: "return impl Iterator<Item = u32>",
                matcher: RuleMatcher::Regex(
                    r"->\s*impl\s+Iterator\s*<\s*Item\s*=\s*u32\s*>",
                ),
            }],
            forbidden: &[],
            canonical_solution: Some(
                "fn evens() -> impl Iterator<Item = u32> {\n    (0..10).filter(|x| x % 2 == 0)\n}",
            ),
            hints: &[
                "Only the return type needs to change for this exercise.",
                "The `Item = u32` part belongs inside the `Iterator<...>` angle brackets.",
            ],
        },
        success_message: "Perfect. This is the standard way to return iterators in Rust.",
    },
    TutorialModule {
        id: "m21-concept",
        title: "21. Concept: Sized & ?Sized",
        module_type: ModuleType::Concept,
        content: r#"
# Memory Layout Knowledge

By default, every generic `T` has a hidden bound: `T: Sized`. This means Rust must know its size at compile-time to put it on the stack.

**Unsized types** (like `str` or `dyn Trait`) don't have a fixed size. To allow them, we use the **Relaxed Bound**: **`?Sized`**.
        "#,
        initial_code: "// fn f<T: ?Sized>(x: &T)",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m22-practice",
        title: "22. Practice: ?Sized",
        module_type: ModuleType::Practice,
        content: r#"
# Relaxing Bounds

### Task:
Update the `where` clause to allow `T` to be unsized.
        "#,
        initial_code: "fn process<T>(item: &T) \nwhere \n    T: \n{\n    // ...\n}",
        validation: ValidationSpec::Rules {
            normalize: PUNCT_NORMALIZE,
            required: &[ValidationRule {
                label: "replace the where clause with T: ?Sized",
                matcher: RuleMatcher::NormalizedExact(
                    "fn process<T>(item: &T)\nwhere\n    T: ?Sized\n{\n    // ...\n}",
                ),
            }],
            forbidden: &[],
            canonical_solution: Some(
                "fn process<T>(item: &T)\nwhere\n    T: ?Sized\n{\n    // ...\n}",
            ),
            hints: &[
                "Keep the existing function structure and only change the bound after `T:`.",
                "The relaxed bound is written exactly as `?Sized`.",
            ],
        },
        success_message: "Correct! This function can now work with slices and trait objects.",
    },
    TutorialModule {
        id: "m23-concept",
        title: "23. Concept: Auto Traits",
        module_type: ModuleType::Concept,
        content: r#"
# Compiler Stickers: Send & Sync

**Auto Traits** are implemented by the compiler automatically based on structure.

- **Send**: It's safe to move this type to another thread.
- **Sync**: It's safe to share references to this type across threads.

If all fields of your struct are `Send`, your struct is `Send`. You don't write the code; the compiler just knows.
        "#,
        initial_code: "// Send/Sync: The thread-safety markers.",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m24-practice",
        title: "24. Practice: Blanket Impls",
        module_type: ModuleType::Practice,
        content: r#"
# Implementing for Everyone

A **Blanket Implementation** gives a capability to *every* type that meets a certain criteria.

### Task:
Implement the `Notify` trait for **every type `T`** that already implements `ToString`.
        "#,
        initial_code: "trait Notify { fn notify(&self); }\n\n// Write: impl<T: ToString> Notify for T { ... }\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[ValidationRule {
                label: "write impl<T: ToString> Notify for T",
                matcher: RuleMatcher::Regex(
                    r"impl\s*<\s*T\s*:\s*ToString\s*>\s*Notify\s+for\s+T",
                ),
            }],
            forbidden: &[],
            canonical_solution: Some(
                "trait Notify { fn notify(&self); }\n\nimpl<T: ToString> Notify for T {\n    fn notify(&self) {}\n}",
            ),
            hints: &[
                "The generic parameter should be constrained as `T: ToString`.",
                "This lesson is about the impl header; the method body can stay minimal.",
            ],
        },
        success_message: "Masterful. You've just updated every ToString type in the world!",
    },
    TutorialModule {
        id: "m25-concept",
        title: "25. Concept: Async in Traits (AFIT)",
        module_type: ModuleType::Concept,
        content: r#"
# Modern Concurrency

Until recently, you needed a macro (`#[async_trait]`) for this. Now, you can use `async fn` directly!

**Note**: Async methods return a hidden Future, so they are **not dyn-compatible** by default yet.
        "#,
        initial_code: "// trait S { async fn run(&self); }",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m26-practice",
        title: "26. Practice: AFIT",
        module_type: ModuleType::Practice,
        content: r#"
# Async Trait Practice

### Task:
Add an **`async`** method named **`download`** that returns a `u64`.
        "#,
        initial_code: "trait Downloader {\n    // Add async method here\n}",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[ValidationRule {
                label: "add async fn download(&self) -> u64;",
                matcher: RuleMatcher::Regex(
                    r"async\s+fn\s+download\s*\(\s*&self\s*\)\s*->\s*u64\s*;",
                ),
            }],
            forbidden: &[],
            canonical_solution: Some(
                "trait Downloader {\n    async fn download(&self) -> u64;\n}",
            ),
            hints: &[
                "Use `async fn`, not a regular `fn` returning a future type.",
                "The signature still ends with a semicolon because this is a trait method declaration.",
            ],
        },
        success_message: "Native async traits verified!",
    },
    TutorialModule {
        id: "m27-concept",
        title: "27. Concept: HRTB (for<'a>)",
        module_type: ModuleType::Concept,
        content: r#"
# For All Lifetimes

Sometimes a closure must work with **any** lifetime the function provides.

Syntax: **`for<'a> Trait<'a>`**

**Mental Hook**: Read it as "For every possible lifetime 'a, this bound must hold."
        "#,
        initial_code: "// F: for<'a> Fn(&'a str)",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m28-practice",
        title: "28. Practice: HRTB",
        module_type: ModuleType::Practice,
        content: r#"
# Bound Practice

### Task:
Add an HRTB to the bound for `F`.
- It should be: **`for<'a> Fn(&'a str)`**.
        "#,
        initial_code: "fn call_anywhere<F>(f: F)\nwhere\n    F: \n{\n    let s = String::from(\"hi\");\n    f(&s);\n}",
        validation: ValidationSpec::Rules {
            normalize: PUNCT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "place for<'a> after the F bound",
                    matcher: RuleMatcher::OrderedContains(&[
                        "where",
                        "F: for<'a>",
                        "Fn(&'a str)",
                    ]),
                },
                ValidationRule {
                    label: "keep the call to f(&s);",
                    matcher: RuleMatcher::Contains("f(&s);"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some(
                "fn call_anywhere<F>(f: F)\nwhere\n    F: for<'a> Fn(&'a str)\n{\n    let s = String::from(\"hi\");\n    f(&s);\n}",
            ),
            hints: &[
                "The `for<'a>` belongs in the bound itself, immediately before `Fn(...)`.",
                "Keep the bound attached to `F` in the existing `where` clause.",
            ],
        },
        success_message: "Excellent! The closure is now flexible across any internal lifetime.",
    },
    TutorialModule {
        id: "m29-concept",
        title: "29. Concept: GATs",
        module_type: ModuleType::Concept,
        content: r#"
# Generic Associated Types

A GAT is an associated type that takes its own generic parameters.

This allows for **"Lending Iterators"**--iterators where the item produced borrows from the iterator itself. This was the "missing piece" of the trait system for years.
        "#,
        initial_code: "// type Item<'a>;",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m30-practice",
        title: "30. Practice: GATs",
        module_type: ModuleType::Practice,
        content: r#"
# GAT Practice

### Task:
Make the associated type **`Item`** take a lifetime parameter **`<'a>`**.
        "#,
        initial_code: "trait LendingIterator {\n    // Add <'a> to Item\n    type Item;\n\n    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;\n}",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[ValidationRule {
                label: "change Item to type Item<'a>;",
                matcher: RuleMatcher::Regex(r#"type\s+Item\s*<\s*'a\s*>\s*;"#),
            }],
            forbidden: &[],
            canonical_solution: Some(
                "trait LendingIterator {\n    type Item<'a>;\n\n    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;\n}",
            ),
            hints: &[
                "Only the associated type line needs the lifetime parameter.",
                "Use the same lifetime name shown in the task: `<'a>`.",
            ],
        },
        success_message: "You've mastered GATs--the peak of trait syntax!",
    },
    TutorialModule {
        id: "m31-concept",
        title: "31. Concept: Fully Qualified Syntax",
        module_type: ModuleType::Concept,
        content: r#"
# Tie-Breaking

When two traits have the same method name, Rust needs you to be explicit.

**Syntax**: `<Type as Trait>::method(&instance)`
        "#,
        initial_code: "// <S as A>::foo(&s)",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "m32-finish",
        title: "32. Full Mastery: Graduation",
        module_type: ModuleType::Concept,
        content: r#"
# Congratulations, Trait Master!

You have completed the entire 32-step journey from basic contracts to GATs and HRTBs.

### The 3-Lens Review:
1. **Behavior**: What can it do?
2. **Constraint**: What MUST it do?
3. **Dispatch**: How does it run?

**You are now ready to architect industrial-grade Rust abstractions.**
        "#,
        initial_code: "// GRADUATION COMPLETE.\n// Rust Traits Mastery: 100%",
        validation: ValidationSpec::Acknowledge,
        success_message: "You are now ready to architect industrial-grade Rust abstractions.",
    },
];
