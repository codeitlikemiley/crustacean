use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fassoc-1-concept",
        title: "1. Associated Types vs Generics",
        module_type: ModuleType::Concept,
        content: r#"
# When to Use Which?

**Generic parameter `<T>`**: The implementor *chooses* T, and a type can implement the trait multiple times with different T values.

**Associated type `type Item`**: Each type has exactly *one* implementation. The type is determined by the impl, not the caller.

```rust
// Generic: Vec can implement From<T> for many T values
impl<T> From<Vec<T>> for MyType { ... }

// Associated: Iterator has exactly one Item per type
trait Iterator {
    type Item;  // Not <T>!
    fn next(&mut self) -> Option<Self::Item>;
}
```
        "#,
        initial_code: "// Study associated types vs generics, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fassoc-2-practice",
        title: "2. Practice: Define Associated Type",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: `Converter` trait

### Task:
Define a trait `Converter` with an associated type `Output` and a method `convert(&self) -> Self::Output`.
        "#,
        initial_code: "// Define trait Converter\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define `trait Converter`",
                    matcher: RuleMatcher::Contains("trait Converter"),
                },
                ValidationRule {
                    label: "associated `type Output`",
                    matcher: RuleMatcher::Contains("type Output"),
                },
                ValidationRule {
                    label: "method returns `Self::Output`",
                    matcher: RuleMatcher::Contains("-> Self::Output"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("trait Converter {\n    type Output;\n    fn convert(&self) -> Self::Output;\n}"),
            hints: &[
                "Inside the trait: `type Output;`",
                "Method signature: `fn convert(&self) -> Self::Output;`",
            ],
        },
        success_message: "Each implementor specifies its own Output type!",
    },
    TutorialModule {
        id: "fassoc-3-practice",
        title: "3. Practice: Implement It",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Implement `Converter` for `Celsius`

### Task:
Implement `Converter` for `Celsius`. Set `type Output = f64;` and return `self.0 * 1.8 + 32.0` in `convert`.
        "#,
        initial_code: "trait Converter {\n    type Output;\n    fn convert(&self) -> Self::Output;\n}\n\nstruct Celsius(f64);\n\n// Implement Converter for Celsius\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "impl Converter for Celsius",
                    matcher: RuleMatcher::OrderedContains(&["impl", "Converter", "for Celsius"]),
                },
                ValidationRule {
                    label: "type Output = f64",
                    matcher: RuleMatcher::Contains("type Output = f64;"),
                },
                ValidationRule {
                    label: "conversion formula",
                    matcher: RuleMatcher::Contains("1.8"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl Converter for Celsius {\n    type Output = f64;\n    fn convert(&self) -> Self::Output {\n        self.0 * 1.8 + 32.0\n    }\n}"),
            hints: &[
                "Set `type Output = f64;`",
                "Return `self.0 * 1.8 + 32.0`",
            ],
        },
        success_message: "Associated type bound to a concrete implementation!",
    },
];
