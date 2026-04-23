use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fto-1-concept",
        title: "1. Static vs Dynamic Dispatch",
        module_type: ModuleType::Concept,
        content: r#"
# Static vs Dynamic Dispatch

When you write `fn draw<T: Shape>(s: &T)`, the compiler generates a separate copy for every concrete type. This is **static dispatch** — fast, but increases binary size.

When you write `fn draw(s: &dyn Shape)`, the compiler uses a **vtable** (virtual function table) at runtime to look up which `draw` to call. This is **dynamic dispatch** — slightly slower, but flexible.

```rust
trait Shape {
    fn area(&self) -> f64;
}

// Static: monomorphized at compile time
fn print_area_static<T: Shape>(s: &T) {
    println!("{}", s.area());
}

// Dynamic: resolved at runtime via vtable
fn print_area_dynamic(s: &dyn Shape) {
    println!("{}", s.area());
}
```

**When to use `dyn`:** When you need to store *different* types in the same collection, or when you want to reduce binary size.
        "#,
        initial_code: "// Study dispatch modes, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fto-2-concept",
        title: "2. The Vtable",
        module_type: ModuleType::Concept,
        content: r#"
# How the Vtable Works

A `&dyn Trait` is a **fat pointer**: it contains two pointers:
1. A pointer to the **data** (the actual struct).
2. A pointer to the **vtable** (a table of function pointers for that type's trait implementation).

```
&dyn Shape for Circle
┌──────────────────┐
│ data_ptr ─────────► Circle { radius: 5.0 }
│ vtable_ptr ───────► [ area_fn_ptr, drop_fn_ptr, size, align ]
└──────────────────┘
```

This is why `&dyn Trait` is **16 bytes** (two `usize` pointers) instead of the normal 8 bytes for a regular reference. You pay for the extra indirection, but gain the ability to erase the concrete type.
        "#,
        initial_code: "// Study vtable layout, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fto-3-practice",
        title: "3. Practice: Box<dyn Trait>",
        module_type: ModuleType::Practice,
        content: r#"
# Heterogeneous Collections

### Task:
Create a vector named `shapes` that holds `Box<dyn Shape>`. Push a `Circle` and a `Square` into it.
        "#,
        initial_code: "trait Shape {\n    fn area(&self) -> f64;\n}\n\nstruct Circle;\nimpl Shape for Circle {\n    fn area(&self) -> f64 { 3.14 }\n}\n\nstruct Square;\nimpl Shape for Square {\n    fn area(&self) -> f64 { 4.0 }\n}\n\nfn main() {\n    // Create `shapes` here\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "declare Vec<Box<dyn Shape>>",
                    matcher: RuleMatcher::Contains("Vec<Box<dyn Shape>>"),
                },
                ValidationRule {
                    label: "push a Circle",
                    matcher: RuleMatcher::Contains("Box::new(Circle)"),
                },
                ValidationRule {
                    label: "push a Square",
                    matcher: RuleMatcher::Contains("Box::new(Square)"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle), Box::new(Square)];"),
            hints: &[
                "The type annotation is `Vec<Box<dyn Shape>>`.",
                "Wrap each struct in `Box::new(...)` to create a trait object.",
            ],
        },
        success_message: "You stored different types in one collection!",
    },
    TutorialModule {
        id: "fto-4-practice",
        title: "4. Practice: Trait Object Parameter",
        module_type: ModuleType::Practice,
        content: r#"
# Accepting Trait Objects

### Task:
Write a function `describe` that takes `item: &dyn std::fmt::Display` and prints it using `println!("{}", item)`.
        "#,
        initial_code: "// Write `fn describe` here\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `fn describe`",
                    matcher: RuleMatcher::Regex(r#"fn\s+describe"#),
                },
                ValidationRule {
                    label: "accept `&dyn Display`",
                    matcher: RuleMatcher::Contains("&dyn std::fmt::Display"),
                },
                ValidationRule {
                    label: "use println!",
                    matcher: RuleMatcher::Contains("println!"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn describe(item: &dyn std::fmt::Display) {\n    println!(\"{}\", item);\n}"),
            hints: &[
                "Signature: `fn describe(item: &dyn std::fmt::Display)`",
                "Body: `println!(\"{}\", item);`",
            ],
        },
        success_message: "Dynamic dispatch in action!",
    },
    TutorialModule {
        id: "fto-5-practice",
        title: "5. Practice: Return a Trait Object",
        module_type: ModuleType::Practice,
        content: r#"
# Returning Trait Objects

You can't return `dyn Trait` directly (unknown size!). You must box it.

### Task:
Fix the function to return `Box<dyn Shape>`. Return a `Circle` when `kind` is `"circle"`, and a `Square` otherwise.
        "#,
        initial_code: "trait Shape {\n    fn name(&self) -> &str;\n}\nstruct Circle;\nimpl Shape for Circle { fn name(&self) -> &str { \"circle\" } }\nstruct Square;\nimpl Shape for Square { fn name(&self) -> &str { \"square\" } }\n\nfn make_shape(kind: &str) {\n    // Fix return type and body\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "return type `-> Box<dyn Shape>`",
                    matcher: RuleMatcher::Contains("-> Box<dyn Shape>"),
                },
                ValidationRule {
                    label: "return boxed Circle",
                    matcher: RuleMatcher::Contains("Box::new(Circle)"),
                },
                ValidationRule {
                    label: "return boxed Square",
                    matcher: RuleMatcher::Contains("Box::new(Square)"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("fn make_shape(kind: &str) -> Box<dyn Shape> {\n    if kind == \"circle\" {\n        Box::new(Circle)\n    } else {\n        Box::new(Square)\n    }\n}"),
            hints: &[
                "Change the return type to `-> Box<dyn Shape>`.",
                "Use `if kind == \"circle\" { Box::new(Circle) } else { Box::new(Square) }`.",
            ],
        },
        success_message: "Excellent! Boxing is the standard way to return trait objects.",
    },
];
