use crate::data::TutorialModule;
use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NormalizeOptions {
    pub strip_line_comments: bool,
    pub normalize_punctuation_spacing: bool,
}

impl NormalizeOptions {
    pub const fn new(strip_line_comments: bool, normalize_punctuation_spacing: bool) -> Self {
        Self {
            strip_line_comments,
            normalize_punctuation_spacing,
        }
    }
}

impl Default for NormalizeOptions {
    fn default() -> Self {
        Self::new(false, false)
    }
}

#[derive(Clone, Debug)]
pub enum ValidationSpec {
    Acknowledge,
    Rules {
        normalize: NormalizeOptions,
        required: &'static [ValidationRule],
        forbidden: &'static [ValidationRule],
        canonical_solution: Option<&'static str>,
        hints: &'static [&'static str],
    },
}

#[derive(Clone, Debug)]
pub struct ValidationRule {
    pub label: &'static str,
    pub matcher: RuleMatcher,
}

#[derive(Clone, Debug)]
pub enum RuleMatcher {
    Regex(&'static str),
    Contains(&'static str),
    AnyContains(&'static [&'static str]),
    OrderedContains(&'static [&'static str]),
    NormalizedExact(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiffKind {
    Context,
    Missing,
    Extra,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiffLine {
    pub kind: DiffKind,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationResult {
    pub passed: bool,
    pub matched_checks: usize,
    pub total_checks: usize,
    pub summary: String,
    pub feedback_lines: Vec<String>,
    pub diff_lines: Vec<DiffLine>,
}

pub fn normalize_code(code: &str, options: NormalizeOptions) -> String {
    let normalized_line_endings = code.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines = Vec::new();
    let mut previous_blank = false;

    for line in normalized_line_endings.lines() {
        let stripped = if options.strip_line_comments {
            strip_line_comment(line)
        } else {
            line.to_string()
        };
        let trimmed = stripped.trim_end().to_string();
        let is_blank = trimmed.trim().is_empty();

        if is_blank {
            if !previous_blank {
                lines.push(String::new());
            }
            previous_blank = true;
        } else {
            lines.push(trimmed);
            previous_blank = false;
        }
    }

    while matches!(lines.first(), Some(line) if line.is_empty()) {
        lines.remove(0);
    }
    while matches!(lines.last(), Some(line) if line.is_empty()) {
        lines.pop();
    }

    let result = lines.join("\n");
    if options.normalize_punctuation_spacing {
        compact_punctuation_spacing(&result)
    } else {
        result
    }
}

pub fn validate_module(module: &TutorialModule, code: &str) -> ValidationResult {
    match &module.validation {
        ValidationSpec::Acknowledge => ValidationResult {
            passed: true,
            matched_checks: 1,
            total_checks: 1,
            summary: "Concept acknowledged.".to_string(),
            feedback_lines: Vec::new(),
            diff_lines: Vec::new(),
        },
        ValidationSpec::Rules {
            normalize,
            required,
            forbidden,
            canonical_solution,
            hints,
        } => {
            let normalized_code = normalize_code(code, *normalize);
            let mut matched_checks = 0;
            let mut feedback_lines = Vec::new();

            for rule in *required {
                if rule_matches(rule, &normalized_code, *normalize) {
                    matched_checks += 1;
                } else {
                    feedback_lines.push(format!("Missing: {}", rule.label));
                }
            }

            for rule in *forbidden {
                if rule_matches(rule, &normalized_code, *normalize) {
                    feedback_lines.push(format!("Forbidden: {}", rule.label));
                } else {
                    matched_checks += 1;
                }
            }

            if !feedback_lines.is_empty() {
                for hint in *hints {
                    feedback_lines.push(format!("Hint: {}", hint));
                }
            }

            let total_checks = required.len() + forbidden.len();
            let passed = feedback_lines
                .iter()
                .all(|line| !line.starts_with("Missing:") && !line.starts_with("Forbidden:"));

            let summary = if passed {
                format!(
                    "Validation passed ({}/{} checks matched).",
                    matched_checks,
                    total_checks
                )
            } else {
                format!(
                    "Validation failed ({}/{} checks matched).",
                    matched_checks,
                    total_checks
                )
            };

            let diff_lines = if passed {
                Vec::new()
            } else if let Some(expected) = canonical_solution {
                diff_lines(expected, code, *normalize)
            } else {
                Vec::new()
            };

            ValidationResult {
                passed,
                matched_checks,
                total_checks,
                summary,
                feedback_lines,
                diff_lines,
            }
        }
    }
}

pub fn diff_lines(expected: &str, actual: &str, normalize: NormalizeOptions) -> Vec<DiffLine> {
    let expected = normalize_code(expected, normalize);
    let actual = normalize_code(actual, normalize);
    let expected_lines: Vec<&str> = if expected.is_empty() {
        Vec::new()
    } else {
        expected.lines().collect()
    };
    let actual_lines: Vec<&str> = if actual.is_empty() {
        Vec::new()
    } else {
        actual.lines().collect()
    };

    let m = expected_lines.len();
    let n = actual_lines.len();
    let mut lcs = vec![vec![0usize; n + 1]; m + 1];

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            lcs[i][j] = if expected_lines[i] == actual_lines[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }

    let mut i = 0;
    let mut j = 0;
    let mut diff = Vec::new();

    while i < m && j < n {
        if expected_lines[i] == actual_lines[j] {
            diff.push(DiffLine {
                kind: DiffKind::Context,
                text: expected_lines[i].to_string(),
            });
            i += 1;
            j += 1;
        } else if lcs[i + 1][j] >= lcs[i][j + 1] {
            diff.push(DiffLine {
                kind: DiffKind::Missing,
                text: expected_lines[i].to_string(),
            });
            i += 1;
        } else {
            diff.push(DiffLine {
                kind: DiffKind::Extra,
                text: actual_lines[j].to_string(),
            });
            j += 1;
        }
    }

    while i < m {
        diff.push(DiffLine {
            kind: DiffKind::Missing,
            text: expected_lines[i].to_string(),
        });
        i += 1;
    }

    while j < n {
        diff.push(DiffLine {
            kind: DiffKind::Extra,
            text: actual_lines[j].to_string(),
        });
        j += 1;
    }

    const MAX_DIFF_LINES: usize = 18;
    if diff.len() > MAX_DIFF_LINES {
        let mut truncated = diff[..MAX_DIFF_LINES].to_vec();
        truncated.push(DiffLine {
            kind: DiffKind::Context,
            text: "...".to_string(),
        });
        truncated
    } else {
        diff
    }
}

fn rule_matches(rule: &ValidationRule, code: &str, normalize: NormalizeOptions) -> bool {
    match &rule.matcher {
        RuleMatcher::Regex(pattern) => Regex::new(pattern)
            .map(|regex| regex.is_match(code))
            .unwrap_or(false),
        RuleMatcher::Contains(fragment) => code.contains(fragment),
        RuleMatcher::AnyContains(fragments) => fragments.iter().any(|fragment| code.contains(fragment)),
        RuleMatcher::OrderedContains(fragments) => {
            let mut search_start = 0usize;
            for fragment in *fragments {
                if let Some(relative) = code[search_start..].find(fragment) {
                    search_start += relative + fragment.len();
                } else {
                    return false;
                }
            }
            true
        }
        RuleMatcher::NormalizedExact(expected) => {
            normalize_code(expected, normalize) == normalize_code(code, normalize)
        }
    }
}

fn strip_line_comment(line: &str) -> String {
    match line.find("//") {
        Some(index) => line[..index].to_string(),
        None => line.to_string(),
    }
}

fn compact_punctuation_spacing(code: &str) -> String {
    let mut output = String::new();
    let punctuation = "(){}[],:;<>?=&";
    let mut chars = code.chars();
    let mut pending_space = false;

    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            pending_space = true;
            continue;
        }

        if punctuation.contains(ch) {
            while output.ends_with(' ') {
                output.pop();
            }
            output.push(ch);
            pending_space = false;
            continue;
        }

        if pending_space && !output.is_empty() {
            output.push(' ');
        }

        output.push(ch);
        pending_space = false;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{RUST_OWNERSHIP_MODULES, RUST_TRAIT_MASTERY_MODULES, RUST_VARIABLES_MODULES};

    #[test]
    fn normalizes_line_endings_and_blank_lines() {
        let code = "\r\nfn main() {   \r\n    println!(\"hi\");\r\n\r\n\r\n}\r\n";
        assert_eq!(
            normalize_code(code, NormalizeOptions::default()),
            "fn main() {\n    println!(\"hi\");\n\n}"
        );
    }

    #[test]
    fn regex_rule_matches() {
        let rule = ValidationRule {
            label: "trait declaration",
            matcher: RuleMatcher::Regex(r"trait\s+Speak"),
        };
        assert!(rule_matches(&rule, "trait Speak {}", NormalizeOptions::default()));
    }

    #[test]
    fn any_contains_rule_matches_multiple_forms() {
        let rule = ValidationRule {
            label: "meow return",
            matcher: RuleMatcher::AnyContains(&["\"Meow!\".to_string()", "String::from(\"Meow!\")"]),
        };
        assert!(rule_matches(
            &rule,
            "String::from(\"Meow!\")",
            NormalizeOptions::default()
        ));
    }

    #[test]
    fn ordered_contains_rule_respects_sequence() {
        let rule = ValidationRule {
            label: "ordered bound",
            matcher: RuleMatcher::OrderedContains(&["where", "for<'a>", "Fn(&'a str)"]),
        };
        assert!(rule_matches(
            &rule,
            "fn call_anywhere<F>(f: F) where F: for<'a> Fn(&'a str) {}",
            NormalizeOptions::default()
        ));
        assert!(!rule_matches(
            &rule,
            "Fn(&'a str) where for<'a>",
            NormalizeOptions::default()
        ));
    }

    #[test]
    fn normalized_exact_rule_uses_normalized_input() {
        let rule = ValidationRule {
            label: "exact newtype",
            matcher: RuleMatcher::NormalizedExact("struct MyList(Vec<String>);"),
        };
        assert!(rule_matches(
            &rule,
            "struct MyList(Vec<String>);  \n",
            NormalizeOptions::default()
        ));
    }

    #[test]
    fn normalized_exact_rule_can_ignore_line_comments_when_requested() {
        let rule = ValidationRule {
            label: "exact newtype",
            matcher: RuleMatcher::NormalizedExact("struct MyList(Vec<String>);"),
        };
        assert!(rule_matches(
            &rule,
            "// starter comment\nstruct MyList(Vec<String>);\n",
            NormalizeOptions::new(true, false)
        ));
    }

    #[test]
    fn forbidden_rules_fail_validation() {
        let module = TutorialModule {
            id: "test",
            title: "Test",
            module_type: crate::data::ModuleType::Practice,
            content: "",
            initial_code: "",
            validation: ValidationSpec::Rules {
                normalize: NormalizeOptions::default(),
                required: &[ValidationRule {
                    label: "impl Trait argument",
                    matcher: RuleMatcher::Contains("impl Speak"),
                }],
                forbidden: &[ValidationRule {
                    label: "remove the generic parameter",
                    matcher: RuleMatcher::Contains("<T: Speak>"),
                }],
                canonical_solution: None,
                hints: &["Use impl Speak directly in the argument list."],
            },
            success_message: "",
        };

        let result = validate_module(&module, "fn greet<T: Speak>(subject: T) {}");
        assert!(!result.passed);
        assert!(result
            .feedback_lines
            .iter()
            .any(|line| line.contains("Forbidden: remove the generic parameter")));
    }

    #[test]
    fn diff_lines_marks_missing_and_extra_lines() {
        let diff = diff_lines(
            "line one\nline two",
            "line one\nline three",
            NormalizeOptions::default(),
        );
        assert!(diff.iter().any(|line| line.kind == DiffKind::Missing));
        assert!(diff.iter().any(|line| line.kind == DiffKind::Extra));
    }

    #[test]
    fn active_course_signature_lesson_passes() {
        let module = find_module("m2-practice");
        let result = validate_module(
            module,
            "trait Speak {\n    fn say_hello(&self) -> String;\n}",
        );
        assert!(result.passed);
    }

    #[test]
    fn active_course_allows_multiple_meow_forms() {
        let module = find_module("m4-practice");
        let result = validate_module(
            module,
            "trait Speak { fn say_hello(&self) -> String; }\nstruct Cat;\n\nimpl Speak for Cat {\n    fn say_hello(&self) -> String {\n        String::from(\"Meow!\")\n    }\n}",
        );
        assert!(result.passed);
    }

    #[test]
    fn active_course_hrtb_rule_requires_ordered_fragments() {
        let module = find_module("m28-practice");
        let result = validate_module(
            module,
            "fn call_anywhere<F>(f: F)\nwhere\n    F: for<'a> Fn(&'a str)\n{\n    let s = String::from(\"hi\");\n    f(&s);\n}",
        );
        assert!(result.passed);
    }

    #[test]
    fn active_course_exact_rule_rejects_incorrect_newtype() {
        let module = find_module("m12-practice");
        let result = validate_module(module, "struct MyList(Vec<&str>);");
        assert!(!result.passed);
    }

    #[test]
    fn active_course_newtype_allows_the_starter_comment_line() {
        let module = find_module("m12-practice");
        let result = validate_module(
            module,
            "// Create the Newtype wrapper here\nstruct MyList(Vec<String>);",
        );
        assert!(result.passed);
    }

    #[test]
    fn generated_variables_course_mutable_counter_passes() {
        let module = find_variables_module("vars-2-practice");
        let result = validate_module(module, "let mut counter = 0;\ncounter = 1;");
        assert!(result.passed);
    }

    #[test]
    fn generated_variables_course_constant_rule_passes() {
        let module = find_variables_module("vars-6-practice");
        let result = validate_module(module, "const MAX_POINTS: u32 = 100;");
        assert!(result.passed);
    }

    #[test]
    fn generated_ownership_course_borrow_signature_passes() {
        let module = find_ownership_module("own-8-practice");
        let result = validate_module(
            module,
            "fn print_len(value: &String) -> usize {\n    value.len()\n}",
        );
        assert!(result.passed);
    }

    #[test]
    fn generated_ownership_course_clone_rule_rejects_move() {
        let module = find_ownership_module("own-6-practice");
        let result = validate_module(
            module,
            "let name = String::from(\"Ferris\");\nlet copy = name;",
        );
        assert!(!result.passed);
    }

    fn find_module(id: &str) -> &'static TutorialModule {
        RUST_TRAIT_MASTERY_MODULES
            .iter()
            .find(|module| module.id == id)
            .expect("module should exist")
    }

    fn find_variables_module(id: &str) -> &'static TutorialModule {
        RUST_VARIABLES_MODULES
            .iter()
            .find(|module| module.id == id)
            .expect("module should exist")
    }

    fn find_ownership_module(id: &str) -> &'static TutorialModule {
        RUST_OWNERSHIP_MODULES
            .iter()
            .find(|module| module.id == id)
            .expect("module should exist")
    }
}
