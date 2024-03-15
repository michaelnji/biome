use super::eslint;

/// See https://github.com/facebook/react/blob/main/packages/eslint-plugin-react-hooks/src/index.js
pub(crate) const RECOMMENDED: [(&str, eslint::Severity); 2] = [
    ("react-hooks/rules-of-hooks", eslint::Severity::Error),
    ("react-hooks/exhaustive-deps", eslint::Severity::Warn),
];
