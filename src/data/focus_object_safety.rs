use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fobj-1-concept",
        title: "1. What is Object Safety?",
        module_type: ModuleType::Concept,
        content: r#"
# Object Safety

Not all traits can be used as `dyn Trait`. A trait is **object-safe** only if:
1. All methods have a receiver (`self`, `&self`, `&mut self`, etc.)
2. No method returns `Self`
3. No method has generic type parameters
4. The trait does not require `Self: Sized`

```rust
// Object-safe:
trait Draw { fn draw(&self); }

// NOT object-safe (returns Self):
trait Clone { fn clone(&self) -> Self; }
```
        "#,
        initial_code: "// Study object safety, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fobj-2-concept",
        title: "2. Fixing Unsafe Traits",
        module_type: ModuleType::Concept,
        content: r#"
# Making Traits Object-Safe

If a trait has a method that breaks object safety, you can add a `where Self: Sized` bound to *exclude* that method from the vtable:

```rust
trait MyTrait {
    fn draw(&self);

    // This method breaks object safety, but we opt it out:
    fn clone_box(&self) -> Box<Self> where Self: Sized;
}

// Now `dyn MyTrait` is valid, but you can't call
// `clone_box` through a trait object.
```
        "#,
        initial_code: "// Study the Sized fix, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fobj-3-practice",
        title: "3. Practice: Fix Object Safety",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Add `where Self: Sized`

### Task:
The trait `Animal` has a `clone_self` method that prevents it from being object-safe. Fix it by adding `where Self: Sized` to that method.
        "#,
        initial_code: "trait Animal {\n    fn speak(&self) -> &str;\n    fn clone_self(&self) -> Self;\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "add `where Self: Sized` to clone_self",
                    matcher: RuleMatcher::OrderedContains(&["clone_self", "where", "Self: Sized"]),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("trait Animal {\n    fn speak(&self) -> &str;\n    fn clone_self(&self) -> Self where Self: Sized;\n}"),
            hints: &[
                "Add `where Self: Sized` after the return type of `clone_self`.",
                "This opts the method out of the vtable.",
            ],
        },
        success_message: "Now `dyn Animal` works! The problematic method is simply unavailable via trait objects.",
    },
];
