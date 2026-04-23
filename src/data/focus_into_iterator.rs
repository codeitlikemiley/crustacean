use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "finto-1-concept",
        title: "1. IntoIterator",
        module_type: ModuleType::Concept,
        content: r#"
# Making `for` Loops Work

`for x in collection` calls `collection.into_iter()`. This works via the `IntoIterator` trait. `Vec<T>` implements it three ways: owned (`T`), shared (`&T`), mutable (`&mut T`).
        "#,
        initial_code: "// Study IntoIterator, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "finto-2-practice",
        title: "2. Practice: Implement IntoIterator",
        module_type: ModuleType::Practice,
        content: r#"
### Task:
Implement `IntoIterator` for `NumberList`. Set `type Item = i32`, `type IntoIter = std::vec::IntoIter<i32>`, return `self.0.into_iter()`.
        "#,
        initial_code: "struct NumberList(Vec<i32>);\n\n// Implement IntoIterator for NumberList\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "impl IntoIterator for NumberList",
                    matcher: RuleMatcher::OrderedContains(&["impl", "IntoIterator", "for NumberList"]),
                },
                ValidationRule {
                    label: "type Item = i32",
                    matcher: RuleMatcher::Contains("type Item = i32"),
                },
                ValidationRule {
                    label: "return self.0.into_iter()",
                    matcher: RuleMatcher::Contains("self.0.into_iter()"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl IntoIterator for NumberList {\n    type Item = i32;\n    type IntoIter = std::vec::IntoIter<i32>;\n    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }\n}"),
            hints: &[
                "Delegate to the inner Vec's into_iter.",
                "IntoIter type is `std::vec::IntoIter<i32>`.",
            ],
        },
        success_message: "Now `for n in my_list { ... }` works!",
    },
];
