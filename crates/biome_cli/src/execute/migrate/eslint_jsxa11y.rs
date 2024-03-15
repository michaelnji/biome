/// Configuration related to the
/// [JSX A11y Eslint plugin](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y).
///
/// Also, the module includes implementation to convert rule options to Biome's rule options.
use biome_deserialize_macros::Deserializable;
use biome_js_analyze::lint::a11y::use_valid_aria_role;

use super::eslint;

#[derive(Debug, Default, Deserializable)]
pub(crate) struct AriaRoleOptions {
    allow_invalid_roles: Vec<String>,
    ignore_non_dom: bool,
}
impl From<AriaRoleOptions> for use_valid_aria_role::ValidAriaRoleOptions {
    fn from(val: AriaRoleOptions) -> Self {
        use_valid_aria_role::ValidAriaRoleOptions {
            allow_invalid_roles: val.allow_invalid_roles,
            ignore_non_dom: val.ignore_non_dom,
        }
    }
}

/// See https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/src/index.js
pub(crate) const DISABLED_IN_RECOMMENDED: [(&str, eslint::Severity); 1] =
    [("jsx-a11y/anchor-ambiguous-text", eslint::Severity::Off)];

/// See https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/src/index.js
pub(crate) const STRICT: [(&str, eslint::Severity); 33] = [
    ("jsx-a11y/alt-text", eslint::Severity::Error),
    ("jsx-a11y/anchor-has-content", eslint::Severity::Error),
    ("jsx-a11y/anchor-is-valid", eslint::Severity::Error),
    (
        "jsx-a11y/aria-activedescendant-has-tabindex",
        eslint::Severity::Error,
    ),
    ("jsx-a11y/aria-props", eslint::Severity::Error),
    ("jsx-a11y/aria-proptypes", eslint::Severity::Error),
    ("jsx-a11y/aria-role", eslint::Severity::Error),
    (
        "jsx-a11y/aria-unsupported-elements",
        eslint::Severity::Error,
    ),
    ("jsx-a11y/autocomplete-valid", eslint::Severity::Error),
    (
        "jsx-a11y/click-events-have-key-events",
        eslint::Severity::Error,
    ),
    (
        "jsx-a11y/control-has-associated-label",
        eslint::Severity::Off,
    ),
    ("jsx-a11y/heading-has-content", eslint::Severity::Error),
    ("jsx-a11y/html-has-lang", eslint::Severity::Error),
    ("jsx-a11y/iframe-has-title", eslint::Severity::Error),
    ("jsx-a11y/img-redundant-alt", eslint::Severity::Error),
    (
        "jsx-a11y/interactive-supports-focus",
        eslint::Severity::Error,
    ),
    ("jsx-a11y/label-has-for", eslint::Severity::Off),
    (
        "jsx-a11y/label-has-associated-control",
        eslint::Severity::Error,
    ),
    ("jsx-a11y/media-has-caption", eslint::Severity::Error),
    (
        "jsx-a11y/mouse-events-have-key-events",
        eslint::Severity::Error,
    ),
    ("jsx-a11y/no-access-key", eslint::Severity::Error),
    ("jsx-a11y/no-autofocus", eslint::Severity::Error),
    ("jsx-a11y/no-distracting-elements", eslint::Severity::Error),
    (
        "jsx-a11y/no-interactive-element-to-noninteractive-role",
        eslint::Severity::Error,
    ),
    (
        "jsx-a11y/no-noninteractive-element-interactions",
        eslint::Severity::Error,
    ),
    (
        "jsx-a11y/no-noninteractive-element-to-interactive-role",
        eslint::Severity::Error,
    ),
    (
        "jsx-a11y/no-noninteractive-tabindex",
        eslint::Severity::Error,
    ),
    ("jsx-a11y/no-redundant-roles", eslint::Severity::Error),
    (
        "jsx-a11y/no-static-element-interactions",
        eslint::Severity::Error,
    ),
    (
        "jsx-a11y/role-has-required-aria-props",
        eslint::Severity::Error,
    ),
    ("jsx-a11y/role-supports-aria-props", eslint::Severity::Error),
    ("jsx-a11y/scope", eslint::Severity::Error),
    ("jsx-a11y/tabindex-no-positive", eslint::Severity::Error),
];
