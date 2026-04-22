# Validator Recipes

Use these patterns when authoring `ValidationSpec::Rules`.

## Default Normalize Options

Use:

```rust
const DEFAULT_NORMALIZE: NormalizeOptions = NormalizeOptions::new(false, false);
```

Only enable punctuation spacing normalization for narrow template-style exercises where formatting should not matter.

## Matcher Selection

### `Contains`

Use when one exact fragment should exist.

```rust
ValidationRule {
    label: "define a trait named Speak",
    matcher: RuleMatcher::Contains("trait Speak"),
}
```

### `AnyContains`

Use when a very small set of alternatives is acceptable.

```rust
ValidationRule {
    label: "return the Meow greeting",
    matcher: RuleMatcher::AnyContains(&[
        "\"Meow!\".to_string()",
        "String::from(\"Meow!\")",
    ]),
}
```

### `OrderedContains`

Use when fragments must appear in order, especially in bounds.

```rust
ValidationRule {
    label: "place for<'a> after the F bound",
    matcher: RuleMatcher::OrderedContains(&[
        "where",
        "F:for<'a>",
        "Fn(&'a str)",
    ]),
}
```

### `Regex`

Use only for syntax-sensitive declarations where spacing can vary.

```rust
ValidationRule {
    label: "add async fn download(&self) -> u64;",
    matcher: RuleMatcher::Regex(
        r"async\s+fn\s+download\s*\(\s*&self\s*\)\s*->\s*u64\s*;",
    ),
}
```

### `NormalizedExact`

Use for narrow template exercises only.

```rust
ValidationRule {
    label: "define the tuple struct exactly as MyList(Vec<String>)",
    matcher: RuleMatcher::NormalizedExact("struct MyList(Vec<String>);"),
}
```

## Authoring Checklist

Every practice should include:
- clear `required` rules with readable labels
- `forbidden` rules when the old form must be removed
- one `canonical_solution`
- at least two targeted `hints`
- one passing sample and one failing sample for manual or automated validation

## Non-Goals

Do not design rules for:
- broad semantic equivalence
- inferred type correctness
- borrow-check validity
- macro behavior
- multi-file project structure

Keep the exercise narrow enough that structural validation is honest and useful.
