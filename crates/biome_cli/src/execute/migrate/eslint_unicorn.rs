/// Configuration related to the
/// [Unicorn Eslint plugin](https://github.com/sindresorhus/eslint-plugin-unicorn).
///
/// ALso, the module includes implementation to convert rule options to Biome's rule options.
use biome_deserialize_macros::Deserializable;
use biome_js_analyze::lint::style::use_filenaming_convention;
use smallvec::SmallVec;

use super::eslint;

#[derive(Clone, Debug, Default, Deserializable)]
pub(crate) struct FilenameCaseOptions {
    case: FilenameCase,
    cases: FilenameCases,
    ignore: Vec<String>,
    multiple_file_extensions: bool,
}
impl From<FilenameCaseOptions> for use_filenaming_convention::FilenamingConventionOptions {
    fn from(val: FilenameCaseOptions) -> Self {
        let filename_cases: Option<use_filenaming_convention::FilenameCases> = val.cases.into();
        use_filenaming_convention::FilenamingConventionOptions {
            strict_case: true,
            require_ascii: true,
            filename_cases: filename_cases.unwrap_or_else(|| {
                use_filenaming_convention::FilenameCases::from_iter([val.case.into()])
            }),
        }
    }
}
#[derive(Clone, Debug, Default, Deserializable)]
pub(crate) enum FilenameCase {
    #[default]
    #[deserializable(rename = "kebabCase")]
    Kebab,
    #[deserializable(rename = "camelCase")]
    Camel,
    #[deserializable(rename = "snakeCase")]
    Snake,
    #[deserializable(rename = "pascalCase")]
    Pascal,
}
impl From<FilenameCase> for use_filenaming_convention::FilenameCase {
    fn from(val: FilenameCase) -> Self {
        match val {
            FilenameCase::Kebab => use_filenaming_convention::FilenameCase::Kebab,
            FilenameCase::Camel => use_filenaming_convention::FilenameCase::Camel,
            FilenameCase::Snake => use_filenaming_convention::FilenameCase::Snake,
            FilenameCase::Pascal => use_filenaming_convention::FilenameCase::Pascal,
        }
    }
}
#[derive(Clone, Debug, Default, Deserializable)]
pub(crate) struct FilenameCases {
    kebab_case: bool,
    camel_case: bool,
    snake_case: bool,
    pascal_case: bool,
}
impl From<FilenameCases> for Option<use_filenaming_convention::FilenameCases> {
    fn from(val: FilenameCases) -> Self {
        let mut cases: SmallVec<[use_filenaming_convention::FilenameCase; 4]> = SmallVec::new();
        if val.kebab_case {
            cases.push(use_filenaming_convention::FilenameCase::Kebab);
        }
        if val.camel_case {
            cases.push(use_filenaming_convention::FilenameCase::Camel);
        }
        if val.snake_case {
            cases.push(use_filenaming_convention::FilenameCase::Snake);
        }
        if val.pascal_case {
            cases.push(use_filenaming_convention::FilenameCase::Pascal);
        }
        if cases.is_empty() {
            None
        } else {
            Some(use_filenaming_convention::FilenameCases::from_iter(cases))
        }
    }
}

/// `plugin:unicorn/recommmended` preset.
/// See https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/configs/recommended.js
pub(crate) const RECOMMENDED: [(&str, eslint::Severity); 115] = [
    ("no-negated-condition", eslint::Severity::Off),
    ("no-nested-ternary", eslint::Severity::Off),
    ("unicorn/better-regex", eslint::Severity::Error),
    ("unicorn/catch-error-name", eslint::Severity::Error),
    ("unicorn/consistent-destructuring", eslint::Severity::Off),
    (
        "unicorn/consistent-function-scoping",
        eslint::Severity::Error,
    ),
    ("unicorn/custom-error-definition", eslint::Severity::Off),
    ("unicorn/empty-brace-spaces", eslint::Severity::Error),
    ("unicorn/error-message", eslint::Severity::Error),
    ("unicorn/escape-case", eslint::Severity::Error),
    ("unicorn/expiring-todo-comments", eslint::Severity::Error),
    ("unicorn/explicit-length-check", eslint::Severity::Error),
    ("unicorn/filename-case", eslint::Severity::Error),
    ("unicorn/import-style", eslint::Severity::Error),
    ("unicorn/new-for-builtins", eslint::Severity::Error),
    ("unicorn/no-abusive-eslint-disable", eslint::Severity::Error),
    (
        "unicorn/no-anonymous-default-export",
        eslint::Severity::Error,
    ),
    (
        "unicorn/no-array-callback-reference",
        eslint::Severity::Error,
    ),
    ("unicorn/no-array-for-each", eslint::Severity::Error),
    (
        "unicorn/no-array-method-this-argument",
        eslint::Severity::Error,
    ),
    ("unicorn/no-array-push-push", eslint::Severity::Error),
    ("unicorn/no-array-reduce", eslint::Severity::Error),
    (
        "unicorn/no-await-expression-member",
        eslint::Severity::Error,
    ),
    (
        "unicorn/no-await-in-promise-methods",
        eslint::Severity::Error,
    ),
    ("unicorn/no-console-spaces", eslint::Severity::Error),
    ("unicorn/no-document-cookie", eslint::Severity::Error),
    ("unicorn/no-empty-file", eslint::Severity::Error),
    ("unicorn/no-for-loop", eslint::Severity::Error),
    ("unicorn/no-hex-escape", eslint::Severity::Error),
    ("unicorn/no-instanceof-array", eslint::Severity::Error),
    (
        "unicorn/no-invalid-remove-event-listener",
        eslint::Severity::Error,
    ),
    ("unicorn/no-keyword-prefix", eslint::Severity::Off),
    ("unicorn/no-lonely-if", eslint::Severity::Error),
    ("unicorn/no-negated-condition", eslint::Severity::Error),
    ("unicorn/no-nested-ternary", eslint::Severity::Error),
    ("unicorn/no-new-array", eslint::Severity::Error),
    ("unicorn/no-new-buffer", eslint::Severity::Error),
    ("unicorn/no-null", eslint::Severity::Error),
    (
        "unicorn/no-object-as-default-parameter",
        eslint::Severity::Error,
    ),
    ("unicorn/no-process-exit", eslint::Severity::Error),
    (
        "unicorn/no-single-promise-in-promise-methods",
        eslint::Severity::Error,
    ),
    ("unicorn/no-static-only-class", eslint::Severity::Error),
    ("unicorn/no-thenable", eslint::Severity::Error),
    ("unicorn/no-this-assignment", eslint::Severity::Error),
    ("unicorn/no-typeof-undefined", eslint::Severity::Error),
    ("unicorn/no-unnecessary-await", eslint::Severity::Error),
    ("unicorn/no-unnecessary-polyfills", eslint::Severity::Error),
    (
        "unicorn/no-unreadable-array-destructuring",
        eslint::Severity::Error,
    ),
    ("unicorn/no-unreadable-iife", eslint::Severity::Error),
    ("unicorn/no-unused-properties", eslint::Severity::Off),
    (
        "unicorn/no-useless-fallback-in-spread",
        eslint::Severity::Error,
    ),
    ("unicorn/no-useless-length-check", eslint::Severity::Error),
    (
        "unicorn/no-useless-promise-resolve-reject",
        eslint::Severity::Error,
    ),
    ("unicorn/no-useless-spread", eslint::Severity::Error),
    ("unicorn/no-useless-switch-case", eslint::Severity::Error),
    ("unicorn/no-useless-undefined", eslint::Severity::Error),
    ("unicorn/no-zero-fractions", eslint::Severity::Error),
    ("unicorn/number-literal-case", eslint::Severity::Error),
    ("unicorn/numeric-separators-style", eslint::Severity::Error),
    ("unicorn/prefer-add-event-listener", eslint::Severity::Error),
    ("unicorn/prefer-array-find", eslint::Severity::Error),
    ("unicorn/prefer-array-flat-map", eslint::Severity::Error),
    ("unicorn/prefer-array-flat", eslint::Severity::Error),
    ("unicorn/prefer-array-index-of", eslint::Severity::Error),
    ("unicorn/prefer-array-some", eslint::Severity::Error),
    ("unicorn/prefer-at", eslint::Severity::Error),
    (
        "unicorn/prefer-blob-reading-methods",
        eslint::Severity::Error,
    ),
    ("unicorn/prefer-code-point", eslint::Severity::Error),
    ("unicorn/prefer-date-now", eslint::Severity::Error),
    ("unicorn/prefer-default-parameters", eslint::Severity::Error),
    ("unicorn/prefer-dom-node-append", eslint::Severity::Error),
    ("unicorn/prefer-dom-node-dataset", eslint::Severity::Error),
    ("unicorn/prefer-dom-node-remove", eslint::Severity::Error),
    (
        "unicorn/prefer-dom-node-text-content",
        eslint::Severity::Error,
    ),
    ("unicorn/prefer-event-target", eslint::Severity::Error),
    ("unicorn/prefer-export-from", eslint::Severity::Error),
    ("unicorn/prefer-includes", eslint::Severity::Error),
    ("unicorn/prefer-json-parse-buffer", eslint::Severity::Off),
    ("unicorn/prefer-keyboard-event-key", eslint::Severity::Error),
    (
        "unicorn/prefer-logical-operator-over-ternary",
        eslint::Severity::Error,
    ),
    ("unicorn/prefer-math-trunc", eslint::Severity::Error),
    ("unicorn/prefer-modern-dom-apis", eslint::Severity::Error),
    ("unicorn/prefer-modern-math-apis", eslint::Severity::Error),
    ("unicorn/prefer-module", eslint::Severity::Error),
    (
        "unicorn/prefer-native-coercion-functions",
        eslint::Severity::Error,
    ),
    ("unicorn/prefer-negative-index", eslint::Severity::Error),
    ("unicorn/prefer-node-protocol", eslint::Severity::Error),
    ("unicorn/prefer-number-properties", eslint::Severity::Error),
    (
        "unicorn/prefer-object-from-entries",
        eslint::Severity::Error,
    ),
    (
        "unicorn/prefer-optional-catch-binding",
        eslint::Severity::Error,
    ),
    ("unicorn/prefer-prototype-methods", eslint::Severity::Error),
    ("unicorn/prefer-query-selector", eslint::Severity::Error),
    ("unicorn/prefer-reflect-apply", eslint::Severity::Error),
    ("unicorn/prefer-regexp-test", eslint::Severity::Error),
    ("unicorn/prefer-set-has", eslint::Severity::Error),
    ("unicorn/prefer-set-size", eslint::Severity::Error),
    ("unicorn/prefer-spread", eslint::Severity::Error),
    ("unicorn/prefer-string-replace-all", eslint::Severity::Error),
    ("unicorn/prefer-string-slice", eslint::Severity::Error),
    (
        "unicorn/prefer-string-starts-ends-with",
        eslint::Severity::Error,
    ),
    (
        "unicorn/prefer-string-trim-start-end",
        eslint::Severity::Error,
    ),
    ("unicorn/prefer-switch", eslint::Severity::Error),
    ("unicorn/prefer-ternary", eslint::Severity::Error),
    ("unicorn/prefer-top-level-await", eslint::Severity::Error),
    ("unicorn/prefer-type-error", eslint::Severity::Error),
    ("unicorn/prevent-abbreviations", eslint::Severity::Error),
    ("unicorn/relative-url-style", eslint::Severity::Error),
    (
        "unicorn/require-array-join-separator",
        eslint::Severity::Error,
    ),
    (
        "unicorn/require-number-to-fixed-digits-argument",
        eslint::Severity::Error,
    ),
    (
        "unicorn/require-post-message-target-origin",
        eslint::Severity::Off,
    ),
    ("unicorn/string-content", eslint::Severity::Off),
    ("unicorn/switch-case-braces", eslint::Severity::Error),
    ("unicorn/template-indent", eslint::Severity::Error),
    (
        "unicorn/text-encoding-identifier-case",
        eslint::Severity::Error,
    ),
    ("unicorn/throw-new-error", eslint::Severity::Error),
];

/// See https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/configs/recommended.js
pub(crate) const NON_RECOMMENDED: [(&str, eslint::Severity); 7] = [
    ("unicorn/consistent-destructuring", eslint::Severity::Error),
    ("unicorn/custom-error-definition", eslint::Severity::Error),
    ("unicorn/no-keyword-prefix", eslint::Severity::Error),
    ("unicorn/no-unused-properties", eslint::Severity::Error),
    ("unicorn/prefer-json-parse-buffer", eslint::Severity::Error),
    (
        "unicorn/require-post-message-target-origin",
        eslint::Severity::Error,
    ),
    ("unicorn/string-content", eslint::Severity::Error),
];

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashMap;

    use super::*;

    #[test]
    fn non_recommended_disabled_in_recommended_() {
        let rules = FxHashMap::from_iter(RECOMMENDED);
        for (name, _) in NON_RECOMMENDED {
            if let Some(severity) = rules.get(name) {
                assert!(matches!(severity, eslint::Severity::Off), "{name}");
            }
        }
    }
}
