use super::eslint;

/// See https://github.com/import-js/eslint-plugin-import/blob/main/config/errors.js
pub(crate) const ERRORS: [(&str, eslint::Severity); 4] = [
    ("import/named", eslint::Severity::Error),
    ("import/namespace", eslint::Severity::Error),
    ("import/default", eslint::Severity::Error),
    ("import/export", eslint::Severity::Error),
];

/// See https://github.com/import-js/eslint-plugin-import/blob/main/config/warnings.js
pub(crate) const WARNINGS: [(&str, eslint::Severity); 3] = [
    ("import/no-named-as-default", eslint::Severity::Warn),
    ("import/no-named-as-default-member", eslint::Severity::Warn),
    ("import/no-duplicates", eslint::Severity::Warn),
];
