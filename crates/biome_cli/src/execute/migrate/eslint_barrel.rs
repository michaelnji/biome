use super::eslint;

pub(crate) const RECOMMENDED: [(&str, eslint::Severity); 3] = [
    ("barrel-files/avoid-barrel-files", eslint::Severity::Error),
    (
        "barrel-files/avoid-namespace-import",
        eslint::Severity::Error,
    ),
    ("barrel-files/avoid-re-export-all", eslint::Severity::Error),
];
