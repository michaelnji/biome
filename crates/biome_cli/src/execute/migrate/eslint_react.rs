use super::eslint;

/// See https://github.com/jsx-eslint/eslint-plugin-react/blob/master/configs/jsx-runtime.js
pub(crate) const JSX_RUNTIME: [(&str, eslint::Severity); 2] = [
    ("react/react-in-jsx-scope", eslint::Severity::Off),
    ("react/jsx-uses-react", eslint::Severity::Off),
];

/// See https://github.com/jsx-eslint/eslint-plugin-react/blob/master/configs/recommended.js
pub(crate) const RECOMMENDED: [(&str, eslint::Severity); 22] = [
    ("react/display-name", eslint::Severity::Error),
    ("react/jsx-key", eslint::Severity::Error),
    ("react/jsx-no-comment-textnodes", eslint::Severity::Error),
    ("react/jsx-no-duplicate-props", eslint::Severity::Error),
    ("react/jsx-no-target-blank", eslint::Severity::Error),
    ("react/jsx-no-undef", eslint::Severity::Error),
    ("react/jsx-uses-react", eslint::Severity::Error),
    ("react/jsx-uses-vars", eslint::Severity::Error),
    ("react/no-children-prop", eslint::Severity::Error),
    ("react/no-danger-with-children", eslint::Severity::Error),
    ("react/no-deprecated", eslint::Severity::Error),
    ("react/no-direct-mutation-state", eslint::Severity::Error),
    ("react/no-find-dom-node", eslint::Severity::Error),
    ("react/no-is-mounted", eslint::Severity::Error),
    ("react/no-render-return-value", eslint::Severity::Error),
    ("react/no-string-refs", eslint::Severity::Error),
    ("react/no-unescaped-entities", eslint::Severity::Error),
    ("react/no-unknown-property", eslint::Severity::Error),
    ("react/no-unsafe", eslint::Severity::Error),
    ("react/prop-types", eslint::Severity::Error),
    ("react/react-in-jsx-scope", eslint::Severity::Error),
    ("react/require-render-return", eslint::Severity::Error),
];

pub(crate) const NON_RECOMMENDED: [(&str, eslint::Severity); 80] = [
    ("react/boolean-prop-naming", eslint::Severity::Error),
    ("react/button-has-type", eslint::Severity::Error),
    (
        "react/checked-requires-onchange-or-readonly",
        eslint::Severity::Error,
    ),
    (
        "react/default-props-match-prop-types",
        eslint::Severity::Error,
    ),
    ("react/destructuring-assignment", eslint::Severity::Error),
    ("react/forbid-component-props", eslint::Severity::Error),
    ("react/forbid-dom-props", eslint::Severity::Error),
    ("react/forbid-elements", eslint::Severity::Error),
    ("react/forbid-foreign-prop-types", eslint::Severity::Error),
    ("react/forbid-prop-types", eslint::Severity::Error),
    (
        "react/function-component-definition",
        eslint::Severity::Error,
    ),
    ("react/hook-use-state", eslint::Severity::Error),
    ("react/iframe-missing-sandbox", eslint::Severity::Error),
    ("react/index", eslint::Severity::Error),
    ("react/jsx-boolean-value", eslint::Severity::Error),
    ("react/jsx-child-element-spacing", eslint::Severity::Error),
    (
        "react/jsx-closing-bracket-location",
        eslint::Severity::Error,
    ),
    ("react/jsx-closing-tag-location", eslint::Severity::Error),
    ("react/jsx-curly-brace-presence", eslint::Severity::Error),
    ("react/jsx-curly-newline", eslint::Severity::Error),
    ("react/jsx-curly-spacing", eslint::Severity::Error),
    ("react/jsx-equals-spacing", eslint::Severity::Error),
    ("react/jsx-filename-extension", eslint::Severity::Error),
    ("react/jsx-first-prop-new-line", eslint::Severity::Error),
    ("react/jsx-fragments", eslint::Severity::Error),
    ("react/jsx-handler-names", eslint::Severity::Error),
    ("react/jsx-indent", eslint::Severity::Error),
    ("react/jsx-indent-props", eslint::Severity::Error),
    ("react/jsx-max-depth", eslint::Severity::Error),
    ("react/jsx-max-props-per-line", eslint::Severity::Error),
    ("react/jsx-newline", eslint::Severity::Error),
    ("react/jsx-no-bind", eslint::Severity::Error),
    (
        "react/jsx-no-constructed-context-values",
        eslint::Severity::Error,
    ),
    ("react/jsx-no-leaked-render", eslint::Severity::Error),
    ("react/jsx-no-literals", eslint::Severity::Error),
    ("react/jsx-no-script-url", eslint::Severity::Error),
    ("react/jsx-no-useless-fragment", eslint::Severity::Error),
    ("react/jsx-one-expression-per-line", eslint::Severity::Error),
    ("react/jsx-pascal-case", eslint::Severity::Error),
    ("react/jsx-props-no-multi-spaces", eslint::Severity::Error),
    ("react/jsx-props-no-spreading", eslint::Severity::Error),
    ("react/jsx-sort-default-props", eslint::Severity::Error),
    ("react/jsx-sort-props", eslint::Severity::Error),
    ("react/jsx-space-before-closing", eslint::Severity::Error),
    ("react/jsx-tag-spacing", eslint::Severity::Error),
    ("react/jsx-wrap-multilines", eslint::Severity::Error),
    ("react/no-access-state-in-setstate", eslint::Severity::Error),
    ("react/no-adjacent-inline-elements", eslint::Severity::Error),
    ("react/no-array-index-key", eslint::Severity::Error),
    ("react/no-arrow-function-lifecycle", eslint::Severity::Error),
    ("react/no-danger", eslint::Severity::Error),
    ("react/no-did-mount-set-state", eslint::Severity::Error),
    ("react/no-did-update-set-state", eslint::Severity::Error),
    ("react/no-invalid-html-attribute", eslint::Severity::Error),
    ("react/no-multi-comp", eslint::Severity::Error),
    ("react/no-namespace", eslint::Severity::Error),
    (
        "react/no-object-type-as-default-prop",
        eslint::Severity::Error,
    ),
    (
        "react/no-redundant-should-component-update",
        eslint::Severity::Error,
    ),
    ("react/no-set-state", eslint::Severity::Error),
    ("react/no-this-in-sfc", eslint::Severity::Error),
    ("react/no-typos", eslint::Severity::Error),
    (
        "react/no-unstable-nested-components",
        eslint::Severity::Error,
    ),
    (
        "react/no-unused-class-component-methods",
        eslint::Severity::Error,
    ),
    ("react/no-unused-prop-types", eslint::Severity::Error),
    ("react/no-unused-state", eslint::Severity::Error),
    ("react/no-will-update-set-state", eslint::Severity::Error),
    ("react/prefer-es6-class", eslint::Severity::Error),
    ("react/prefer-exact-props", eslint::Severity::Error),
    ("react/prefer-read-only-props", eslint::Severity::Error),
    ("react/prefer-stateless-function", eslint::Severity::Error),
    ("react/require-default-props", eslint::Severity::Error),
    ("react/require-optimization", eslint::Severity::Error),
    ("react/self-closing-comp", eslint::Severity::Error),
    ("react/sort-comp", eslint::Severity::Error),
    ("react/sort-default-props", eslint::Severity::Error),
    ("react/sort-prop-types", eslint::Severity::Error),
    ("react/state-in-constructor", eslint::Severity::Error),
    ("react/static-property-placement", eslint::Severity::Error),
    ("react/style-prop-object", eslint::Severity::Error),
    (
        "react/void-dom-elements-no-children",
        eslint::Severity::Error,
    ),
];

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashMap;

    use super::*;

    #[test]
    fn recommended_xor_non_recommended_() {
        let rules = FxHashMap::from_iter(RECOMMENDED);
        for (name, _) in NON_RECOMMENDED {
            assert!(rules.get(&name).is_none(), "{name}");
        }
    }
}
