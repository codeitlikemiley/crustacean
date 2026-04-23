use crate::data::model::{ModuleType, TutorialModule};
use crate::validation::{RuleMatcher, ValidationRule, ValidationSpec, NormalizeOptions};

pub const MODULES: &[TutorialModule] = &[
    TutorialModule {
        id: "derive_macros_concept_1",
        module_type: ModuleType::Concept,
        title: "Procedural Macros",
        content: "While `macro_rules!` matches syntax patterns, **Procedural Macros** run actual Rust code at compile time!

They take a `TokenStream` (the source code AST) as input, and produce a new `TokenStream` as output.

There are three types:
1. **Custom `#[derive]`**: `#[derive(MyTrait)]`
2. **Attribute-like**: `#[route(\"/get\")] fn run() {}`
3. **Function-like**: `sql!(\"SELECT * FROM users\")`

Proc-macros must be defined in a special crate with `proc-macro = true` in its `Cargo.toml`.",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's write a derive macro signature.",
    },
    TutorialModule {
        id: "derive_macros_practice_1",
        module_type: ModuleType::Practice,
        title: "The Derive Signature",
        content: "To create a custom derive, we use the `#[proc_macro_derive(Name)]` attribute and define a function that maps a `TokenStream` to a `TokenStream`.

We want to create a `#[derive(Loggable)]` macro.

1. Add the `#[proc_macro_derive(Loggable)]` attribute above the function.
2. Complete the function signature: it takes `input: TokenStream` and returns `TokenStream`.",
        initial_code: "extern crate proc_macro;
use proc_macro::TokenStream;

// 1. Add #[proc_macro_derive(Loggable)]
pub fn loggable_derive(input: /* 2. TokenStream */) /* -> TokenStream */ {
    input
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "Add #[proc_macro_derive(Loggable)]",
                    matcher: RuleMatcher::Contains(r#"#[proc_macro_derive(Loggable)]"#),
                },
                ValidationRule {
                    label: "Signature takes and returns TokenStream",
                    matcher: RuleMatcher::Regex(r#"fn\s+loggable_derive\s*\(\s*input\s*:\s*TokenStream\s*\)\s*->\s*TokenStream"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("extern crate proc_macro;\nuse proc_macro::TokenStream;\n\n#[proc_macro_derive(Loggable)]\npub fn loggable_derive(input: TokenStream) -> TokenStream {\n    input\n}"),
            hints: &[
                "The attribute must be exact: `#[proc_macro_derive(Loggable)]`",
                "The function signature should be `pub fn loggable_derive(input: TokenStream) -> TokenStream`."
            ],
        },
        success_message: "Great! That's the basic entry point for any derive macro.",
    },
    TutorialModule {
        id: "derive_macros_concept_2",
        module_type: ModuleType::Concept,
        title: "syn and quote",
        content: "Parsing raw tokens is incredibly hard. Instead, we use two standard crates:
- `syn`: Parses the `TokenStream` into a structured syntax tree (AST).
- `quote`: Converts Rust code back into a `TokenStream`.

```rust
let ast = syn::parse_macro_input!(input as syn::DeriveInput);
let name = &ast.ident;

let expanded = quote::quote! {
    impl Loggable for #name {
        fn log(&self) { println!(\"logging\"); }
    }
};

TokenStream::from(expanded)
```
The `#name` syntax in `quote!` interpolates variables from your macro code directly into the generated Rust code!",
        initial_code: "",
        validation: ValidationSpec::Acknowledge,
        success_message: "Let's practice using quote!.",
    },
    TutorialModule {
        id: "derive_macros_practice_2",
        module_type: ModuleType::Practice,
        title: "Using quote!",
        content: "Let's generate the actual implementation.

We have the struct's name in the `name` variable. Use `quote!` to generate an `impl Hello for #name` block.

1. Create an `impl Hello for #name` block.
2. Inside it, define `fn say_hello() { println!(\"Hello!\"); }`.",
        initial_code: "use quote::quote;

pub fn generate_impl(name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        // 1. impl Hello for #name { ... }
        // 2. fn say_hello() { println!(\"Hello!\"); }
    }
}",
        validation: ValidationSpec::Rules {
            normalize: NormalizeOptions::new(false, false),
            required: &[
                ValidationRule {
                    label: "impl Hello for #name",
                    matcher: RuleMatcher::Regex(r#"impl\s+Hello\s+for\s+#name"#),
                },
                ValidationRule {
                    label: "define say_hello",
                    matcher: RuleMatcher::Contains(r#"fn say_hello() { println!("Hello!"); }"#),
                },
            ],
            forbidden: &[],
            canonical_solution: Some("use quote::quote;\n\npub fn generate_impl(name: &syn::Ident) -> proc_macro2::TokenStream {\n    quote! {\n        impl Hello for #name {\n            fn say_hello() { println!(\"Hello!\"); }\n        }\n    }\n}"),
            hints: &[
                "Write `impl Hello for #name { ... }` inside the quote! block.",
                "Inside the impl block, write exactly `fn say_hello() { println!(\"Hello!\"); }`."
            ],
        },
        success_message: "Brilliant! You can now write procedural macros to eliminate boilerplate in your codebases.",
    },
];
