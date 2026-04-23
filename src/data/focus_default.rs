use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{NormalizeOptions, RuleMatcher, ValidationRule, ValidationSpec};

const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "fdefault-1-concept",
        title: "1. The Default Trait",
        module_type: ModuleType::Concept,
        content: r#"
# Sensible Defaults

The `Default` trait provides a default value for a type. Many stdlib types implement it: `0` for numbers, `""` for String, `false` for bool.

```rust
let x: i32 = Default::default();     // 0
let s: String = Default::default();  // ""
let v: Vec<i32> = Default::default(); // []
```

You can derive it automatically if all fields implement `Default`:
```rust
#[derive(Default)]
struct Config {
    debug: bool,     // false
    port: u16,       // 0
    name: String,    // ""
}
```
        "#,
        initial_code: "// Study Default, then ACKNOWLEDGE.\n",
        validation: ValidationSpec::Acknowledge,
        success_message: "Concept acknowledged.",
    },
    TutorialModule {
        id: "fdefault-2-practice",
        title: "2. Practice: Custom Default",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Implement Default

### Task:
Implement `Default` for `Settings`. Set `volume` to `50` and `muted` to `false`.
        "#,
        initial_code: "struct Settings {\n    volume: u8,\n    muted: bool,\n}\n\n// Implement Default for Settings\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "impl Default for Settings",
                    matcher: RuleMatcher::OrderedContains(&["impl", "Default", "for Settings"]),
                },
                ValidationRule {
                    label: "fn default()",
                    matcher: RuleMatcher::Contains("fn default()"),
                },
                ValidationRule {
                    label: "volume: 50",
                    matcher: RuleMatcher::Contains("volume: 50"),
                },
                ValidationRule {
                    label: "muted: false",
                    matcher: RuleMatcher::Contains("muted: false"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("impl Default for Settings {\n    fn default() -> Self {\n        Settings {\n            volume: 50,\n            muted: false,\n        }\n    }\n}"),
            hints: &[
                "`impl Default for Settings { fn default() -> Self { ... } }`",
                "Return a Settings with your chosen defaults.",
            ],
        },
        success_message: "Now `Settings::default()` gives sensible values!",
    },
    TutorialModule {
        id: "fdefault-3-practice",
        title: "3. Practice: Struct Update with Default",
        module_type: ModuleType::Practice,
        content: r#"
# Practice: Partial Override

### Task:
Create a `Settings` where `volume` is `80` but everything else uses `Default`. Use the `..Default::default()` syntax.
        "#,
        initial_code: "#[derive(Default)]\nstruct Settings { volume: u8, muted: bool }\n\nfn main() {\n    // Create settings with volume = 80, rest default\n}\n",
        validation: ValidationSpec::Rules {
            normalize: DEFAULT_NORMALIZE,
            required: &[
                ValidationRule {
                    label: "set volume: 80",
                    matcher: RuleMatcher::Contains("volume: 80"),
                },
                ValidationRule {
                    label: "use ..Default::default()",
                    matcher: RuleMatcher::Contains("..Default::default()"),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("let s = Settings { volume: 80, ..Default::default() };"),
            hints: &[
                "Syntax: `Settings { volume: 80, ..Default::default() }`",
            ],
        },
        success_message: "Override only what you need — default the rest!",
    },
];
