use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fsealed-1-concept",
        title: "1. The Sealed Trait Pattern",
        module_type: ModuleType::Concept,
        content: r#"
# Preventing External Implementations

Sometimes you want a trait that only YOUR types can implement. The sealed trait pattern uses a private supertrait:

```rust
mod private {
    pub trait Sealed {}  // Public trait, but in a private module!
}

pub trait MyApi: private::Sealed {
    fn do_thing(&self);
}

// Only you can impl Sealed, so only you can impl MyApi.
impl private::Sealed for MyStruct {}
impl MyApi for MyStruct {
    fn do_thing(&self) { ... }
}
```

External users can *use* `MyApi` but can't *implement* it — they can't access `private::Sealed`.
        "#,
        initial_code: "// Study sealed traits, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fsealed-2-practice",
        title: "2. Practice: Seal a Trait",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Create a Sealed Trait

### Task:
Define a `mod private` with a `pub trait Sealed {}`. Then define a public trait `DatabaseDriver` that requires `: private::Sealed`.
        "#,
        initial_code: "// Create the sealed pattern\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "define private module",
                    matcher: RuleMatcher::Contains("mod private"),
                },
                ValidationRule {
                    label: "Sealed trait inside",
                    matcher: RuleMatcher::Contains("trait Sealed"),
                },
                ValidationRule {
                    label: "DatabaseDriver requires Sealed",
                    matcher: RuleMatcher::OrderedContains(&["trait DatabaseDriver", "Sealed"]),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("mod private {\n    pub trait Sealed {}\n}\n\npub trait DatabaseDriver: private::Sealed {\n    fn connect(&self);\n}"),
            hints: &[
                "The private module makes Sealed inaccessible from outside.",
                "DatabaseDriver uses it as a supertrait.",
            ],
        },
        success_message: "Your trait is now sealed — only you control who implements it!",
    },
];
