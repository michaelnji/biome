/// Configuration related to [TypeScript Eslint](https://typescript-eslint.io/).
///
/// ALso, the module includes implementation to convert rule options to Biome's rule options.
use biome_deserialize_macros::Deserializable;
use biome_js_analyze::lint::style::{use_consistent_array_type, use_naming_convention};

use super::eslint;

#[derive(Debug, Default, Deserializable)]
pub(crate) struct ArrayTypeOptions {
    default: ArrayType,
    readonly: Option<ArrayType>,
}
impl From<ArrayTypeOptions> for use_consistent_array_type::ConsistentArrayTypeOptions {
    fn from(val: ArrayTypeOptions) -> Self {
        use_consistent_array_type::ConsistentArrayTypeOptions {
            syntax: val.default.into(),
        }
    }
}
#[derive(Debug, Default, Deserializable)]
pub(crate) enum ArrayType {
    #[default]
    Array,
    #[deserializable(rename = "array-simple")]
    ArraySimple,
    Generic,
}
impl From<ArrayType> for use_consistent_array_type::ConsistentArrayType {
    fn from(val: ArrayType) -> Self {
        match val {
            ArrayType::Array | ArrayType::ArraySimple => {
                use_consistent_array_type::ConsistentArrayType::Shorthand
            }
            ArrayType::Generic => use_consistent_array_type::ConsistentArrayType::Generic,
        }
    }
}

#[derive(Debug)]
pub(crate) struct NamingConventionOptions(Vec<NamingConventionSelection>);
impl NamingConventionOptions {
    pub(crate) fn override_default(
        overrides: impl IntoIterator<Item = NamingConventionSelection>,
    ) -> Self {
        let mut result = Self::default();
        result.0.extend(overrides);
        result
    }
}
impl Default for NamingConventionOptions {
    fn default() -> Self {
        Self(vec![
            NamingConventionSelection {
                selector: Selector::Default.into(),
                format: Some(vec![NamingConventionCase::Camel]),
                leading_underscore: Some(Underscore::Allow),
                trailing_underscore: Some(Underscore::Allow),
                ..Default::default()
            },
            NamingConventionSelection {
                selector: Selector::Import.into(),
                format: Some(vec![
                    NamingConventionCase::Camel,
                    NamingConventionCase::Pascal,
                ]),
                ..Default::default()
            },
            NamingConventionSelection {
                selector: Selector::Variable.into(),
                format: Some(vec![
                    NamingConventionCase::Camel,
                    NamingConventionCase::Upper,
                ]),
                leading_underscore: Some(Underscore::Allow),
                trailing_underscore: Some(Underscore::Allow),
                ..Default::default()
            },
            NamingConventionSelection {
                selector: Selector::TypeLike.into(),
                format: Some(vec![NamingConventionCase::Pascal]),
                leading_underscore: Some(Underscore::Allow),
                trailing_underscore: Some(Underscore::Allow),
                ..Default::default()
            },
        ])
    }
}
impl From<NamingConventionOptions> for use_naming_convention::NamingConventionOptions {
    fn from(val: NamingConventionOptions) -> Self {
        let mut enum_member_format = None;
        for selection in val.0 {
            if selection.selector.contains(&Selector::EnumMember) {
                // We only extract the first format because Biome doesn't allow for now multiple cases.
                enum_member_format = selection
                    .format
                    .and_then(|format| format.into_iter().next());
            }
        }
        use_naming_convention::NamingConventionOptions {
            strict_case: matches!(
                enum_member_format,
                Some(NamingConventionCase::StrictCamel | NamingConventionCase::StrictPascal)
            ),
            require_ascii: false,
            enum_member_case: enum_member_format
                .and_then(|format| {
                    match format {
                        NamingConventionCase::Camel | NamingConventionCase::StrictCamel => {
                            Some(use_naming_convention::EnumMemberCase::Camel)
                        }
                        NamingConventionCase::Pascal | NamingConventionCase::StrictPascal => {
                            Some(use_naming_convention::EnumMemberCase::Pascal)
                        }
                        NamingConventionCase::Upper => {
                            Some(use_naming_convention::EnumMemberCase::Constant)
                        }
                        // Biome doesn't support `snake_case` for enum member
                        NamingConventionCase::Snake => None,
                    }
                })
                .unwrap_or_default(),
        }
    }
}
#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct NamingConventionSelection {
    pub(crate) selector: eslint::Shorthand<Selector>,
    pub(crate) modifiers: Option<Vec<Modifier>>,
    pub(crate) types: Option<Vec<Type>>,
    //pub(crate) custom: Option<Custom>,
    pub(crate) format: Option<Vec<NamingConventionCase>>,
    pub(crate) leading_underscore: Option<Underscore>,
    pub(crate) trailing_underscore: Option<Underscore>,
    //pub(crate) prefix: Option<Vec<String>>,
    //pub(crate) suffix: Option<Vec<String>>,
    //pub(crate) filter: Option<Filter>,
}
//#[derive(Debug, Default, Deserializable)]
//pub(crate) struct Custom {
//    regex: String,
//    #[deserializable(rename = "match")]
//    matches: bool,
//}
//#[derive(Debug, Clone)]
//pub(crate) enum Filter {
//    Regex(String),
//    Custom(Custom),
//}
//impl Deserializable for Filter {
//    fn deserialize(
//        value: &impl biome_deserialize::DeserializableValue,
//        name: &str,
//        diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
//    ) -> Option<Self> {
//        if value.is_type(VisitableType::STR) {
//            Deserializable::deserialize(value, name, diagnostics).map(Filter::Regex)
//        } else {
//            Deserializable::deserialize(value, name, diagnostics).map(Filter::Custom)
//        }
//    }
//}
#[derive(Debug, Deserializable)]
pub(crate) enum NamingConventionCase {
    #[deserializable(rename = "camelCase")]
    Camel,
    #[deserializable(rename = "strictCamelCase")]
    StrictCamel,
    #[deserializable(rename = "PascalCase")]
    Pascal,
    #[deserializable(rename = "StrictPascalCase")]
    StrictPascal,
    #[deserializable(rename = "snake_case")]
    Snake,
    #[deserializable(rename = "UPPER_CASE")]
    Upper,
}
#[derive(Debug, Default, Eq, PartialEq, Deserializable)]
pub(crate) enum Selector {
    // Individual selectors
    ClassicAccessor,
    AutoAccessor,
    Class,
    ClassMethod,
    ClassProperty,
    Enum,
    EnumMember,
    Function,
    Import,
    Interface,
    ObjectLiteralMethod,
    ObjectLiteralProperty,
    Parameter,
    ParameterProperty,
    TypeAlias,
    TypeMethod,
    TypeParameter,
    TypeProperty,
    Variable,
    // group selector
    #[default]
    Default,
    Accessor,
    MemberLike,
    Method,
    Property,
    TypeLike,
    VariableLike,
}
#[derive(Debug, Deserializable)]
pub(crate) enum Modifier {
    Abstract,
    Async,
    Const,
    Destructured,
    Exported,
    Global,
    Override,
    Private,
    Protected,
    Public,
    Readonly,
    RequiresQuotes,
    #[deserializable(rename = "#private")]
    SharpPrivate,
    Static,
    Unused,
}
#[derive(Debug, Deserializable)]
pub(crate) enum Type {
    Array,
    Boolean,
    Function,
    Number,
    String,
}
#[derive(Debug, Deserializable)]
pub(crate) enum Underscore {
    Forbid,
    Require,
    RequireDouble,
    Allow,
    AllowDouble,
    AllowSingleOrDouble,
}

/// `plugin:@typescript-eslint/recommended` preset.
/// See https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/typescript-eslint/src/configs/recommended.ts
pub(crate) const RECOMMENDED: [(&str, eslint::Severity); 20] = [
    ("@typescript-eslint/ban-ts-comment", eslint::Severity::Error),
    ("@typescript-eslint/ban-types", eslint::Severity::Error),
    ("no-array-pub(crate) constructor", eslint::Severity::Off),
    (
        "@typescript-eslint/no-array-pub(crate) constructor",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-duplicate-enum-values",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-explicit-any",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-extra-non-null-assertion",
        eslint::Severity::Error,
    ),
    ("no-loss-of-precision", eslint::Severity::Off),
    (
        "@typescript-eslint/no-loss-of-precision",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/no-misused-new", eslint::Severity::Error),
    ("@typescript-eslint/no-namespace", eslint::Severity::Error),
    (
        "@typescript-eslint/no-non-null-asserted-optional-chain",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/no-this-alias", eslint::Severity::Error),
    (
        "@typescript-eslint/no-unnecessary-type-pub(crate) constraint",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-declaration-merging",
        eslint::Severity::Error,
    ),
    ("no-unused-vars", eslint::Severity::Off),
    ("@typescript-eslint/no-unused-vars", eslint::Severity::Error),
    (
        "@typescript-eslint/no-var-requires",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-as-pub(crate) const",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/triple-slash-reference",
        eslint::Severity::Error,
    ),
];

/// `plugin:@typescript-eslint/recommended-type-checked-only` preset.
/// See https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/typescript-eslint/src/configs/recommended-type-checked-only.ts
pub(crate) const RECOMMENDED_TYPE_CHECKED_ONLY: [(&str, eslint::Severity); 21] = [
    ("@typescript-eslint/await-thenable", eslint::Severity::Error),
    (
        "@typescript-eslint/no-base-to-string",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-duplicate-type-pub(crate) constituents",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-floating-promises",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-for-in-array",
        eslint::Severity::Error,
    ),
    ("no-implied-eval", eslint::Severity::Off),
    (
        "@typescript-eslint/no-implied-eval",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-misused-promises",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-redundant-type-pub(crate) constituents",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unnecessary-type-assertion",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-argument",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-assignment",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/no-unsafe-call", eslint::Severity::Error),
    (
        "@typescript-eslint/no-unsafe-enum-comparison",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-member-access",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-return",
        eslint::Severity::Error,
    ),
    ("require-await", eslint::Severity::Off),
    ("@typescript-eslint/require-await", eslint::Severity::Error),
    (
        "@typescript-eslint/restrict-plus-operands",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/restrict-template-expressions",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/unbound-method", eslint::Severity::Error),
];

/// `plugin:@typescript-eslint/strict` preset.
/// See https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/typescript-eslint/src/configs/strict.ts
pub(crate) const STRICT: [(&str, eslint::Severity); 30] = [
    ("@typescript-eslint/ban-ts-comment", eslint::Severity::Error),
    ("@typescript-eslint/ban-types", eslint::Severity::Error),
    ("no-array-pub(crate) constructor", eslint::Severity::Off),
    (
        "@typescript-eslint/no-array-pub(crate) constructor",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-duplicate-enum-values",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-dynamic-delete",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-explicit-any",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-extra-non-null-assertion",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-extraneous-class",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-invalid-void-type",
        eslint::Severity::Error,
    ),
    ("no-loss-of-precision", eslint::Severity::Off),
    (
        "@typescript-eslint/no-loss-of-precision",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/no-misused-new", eslint::Severity::Error),
    ("@typescript-eslint/no-namespace", eslint::Severity::Error),
    (
        "@typescript-eslint/no-non-null-asserted-nullish-coalescing",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-non-null-asserted-optional-chain",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-non-null-assertion",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/no-this-alias", eslint::Severity::Error),
    (
        "@typescript-eslint/no-unnecessary-type-pub(crate) constraint",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-declaration-merging",
        eslint::Severity::Error,
    ),
    ("no-unused-vars", eslint::Severity::Off),
    ("@typescript-eslint/no-unused-vars", eslint::Severity::Error),
    ("no-useless-pub(crate) constructor", eslint::Severity::Off),
    (
        "@typescript-eslint/no-useless-pub(crate) constructor",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-var-requires",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-as-pub(crate) const",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-literal-enum-member",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-ts-expect-error",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/triple-slash-reference",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/unified-signatures",
        eslint::Severity::Error,
    ),
];

/// `plugin:@typescript-eslint/strict-type-checked-only` preset.
/// See https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/typescript-eslint/src/configs/strict-type-checked-only.ts
pub(crate) const STRICT_YYPE_CHECKED_ONLY: [(&str, eslint::Severity); 37] = [
    ("@typescript-eslint/await-thenable", eslint::Severity::Error),
    (
        "@typescript-eslint/no-array-delete",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-base-to-string",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-confusing-void-expression",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-duplicate-type-pub(crate) constituents",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-floating-promises",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-for-in-array",
        eslint::Severity::Error,
    ),
    ("no-implied-eval", eslint::Severity::Off),
    (
        "@typescript-eslint/no-implied-eval",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-meaningless-void-operator",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-misused-promises",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/no-mixed-enums", eslint::Severity::Error),
    (
        "@typescript-eslint/no-redundant-type-pub(crate) constituents",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unnecessary-boolean-literal-compare",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unnecessary-condition",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unnecessary-type-arguments",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unnecessary-type-assertion",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-argument",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-assignment",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/no-unsafe-call", eslint::Severity::Error),
    (
        "@typescript-eslint/no-unsafe-enum-comparison",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-member-access",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-return",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-useless-template-literals",
        eslint::Severity::Error,
    ),
    ("no-throw-literal", eslint::Severity::Off),
    (
        "@typescript-eslint/only-throw-error",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-includes",
        eslint::Severity::Error,
    ),
    ("prefer-promise-reject-errors", eslint::Severity::Off),
    (
        "@typescript-eslint/prefer-promise-reject-errors",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-reduce-type-parameter",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-return-this-type",
        eslint::Severity::Error,
    ),
    ("require-await", eslint::Severity::Off),
    ("@typescript-eslint/require-await", eslint::Severity::Error),
    (
        "@typescript-eslint/restrict-plus-operands",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/restrict-template-expressions",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/unbound-method", eslint::Severity::Error),
    (
        "@typescript-eslint/use-unknown-in-catch-callback-variable",
        eslint::Severity::Error,
    ),
];

/// `plugin:@typescript-eslint/stylistic` preset.
/// https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/typescript-eslint/src/configs/stylistic.ts
pub(crate) const STYLISTIC: [(&str, eslint::Severity); 16] = [
    (
        "@typescript-eslint/adjacent-overload-signatures",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/array-type", eslint::Severity::Error),
    (
        "@typescript-eslint/ban-tslint-comment",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/class-literal-property-style",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/consistent-generic-pub(crate) constructors",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/consistent-indexed-object-style",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/consistent-type-assertions",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/consistent-type-definitions",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-confusing-non-null-assertion",
        eslint::Severity::Error,
    ),
    ("no-empty-function", eslint::Severity::Off),
    (
        "@typescript-eslint/no-empty-function",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-empty-interface",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-inferrable-types",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/prefer-for-of", eslint::Severity::Error),
    (
        "@typescript-eslint/prefer-function-type",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-namespace-keyword",
        eslint::Severity::Error,
    ),
];

/// `plugin:@typescript-eslint/stylistic-type-checked-only` preset.
/// See https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/typescript-eslint/src/configs/stylistic-type-checked-only.ts
pub(crate) const STYLISTIC_TYPE_CHECKED_ONLY: [(&str, eslint::Severity); 6] = [
    ("dot-notation", eslint::Severity::Off),
    ("@typescript-eslint/dot-notation", eslint::Severity::Error),
    (
        "@typescript-eslint/non-nullable-type-assertion-style",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-nullish-coalescing",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-optional-chain",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-string-starts-ends-with",
        eslint::Severity::Error,
    ),
];

/// `plugin:@typescript-eslint/disable-type-checked` preset.
/// See https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/typescript-eslint/src/configs/disable-type-checked.ts
pub(crate) const DISABLE_TYPE_CHECKED: [(&str, eslint::Severity); 54] = [
    ("@typescript-eslint/await-thenable", eslint::Severity::Off),
    (
        "@typescript-eslint/consistent-return",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/consistent-type-exports",
        eslint::Severity::Off,
    ),
    ("@typescript-eslint/dot-notation", eslint::Severity::Off),
    (
        "@typescript-eslint/naming_convention",
        eslint::Severity::Off,
    ),
    ("@typescript-eslint/no-array-delete", eslint::Severity::Off),
    (
        "@typescript-eslint/no-base-to-string",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/no-confusing-void-expression",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/no-duplicate-type-pub(crate) constituents",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/no-floating-promises",
        eslint::Severity::Off,
    ),
    ("@typescript-eslint/no-for-in-array", eslint::Severity::Off),
    ("@typescript-eslint/no-implied-eval", eslint::Severity::Off),
    (
        "@typescript-eslint/no-meaningless-void-operator",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/no-misused-promises",
        eslint::Severity::Off,
    ),
    ("@typescript-eslint/no-mixed-enums", eslint::Severity::Off),
    (
        "@typescript-eslint/no-redundant-type-pub(crate) constituents",
        eslint::Severity::Off,
    ),
    ("@typescript-eslint/no-throw-literal", eslint::Severity::Off),
    (
        "@typescript-eslint/no-unnecessary-boolean-literal-compare",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/no-unnecessary-condition",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/no-unnecessary-qualifier",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/no-unnecessary-type-arguments",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/no-unnecessary-type-assertion",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/no-unsafe-argument",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/no-unsafe-assignment",
        eslint::Severity::Off,
    ),
    ("@typescript-eslint/no-unsafe-call", eslint::Severity::Off),
    (
        "@typescript-eslint/no-unsafe-enum-comparison",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/no-unsafe-member-access",
        eslint::Severity::Off,
    ),
    ("@typescript-eslint/no-unsafe-return", eslint::Severity::Off),
    (
        "@typescript-eslint/no-unsafe-unary-minus",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/no-useless-template-literals",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/non-nullable-type-assertion-style",
        eslint::Severity::Off,
    ),
    ("@typescript-eslint/only-throw-error", eslint::Severity::Off),
    (
        "@typescript-eslint/prefer-destructuring",
        eslint::Severity::Off,
    ),
    ("@typescript-eslint/prefer-find", eslint::Severity::Off),
    ("@typescript-eslint/prefer-includes", eslint::Severity::Off),
    (
        "@typescript-eslint/prefer-nullish-coalescing",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/prefer-optional-chain",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/prefer-promise-reject-errors",
        eslint::Severity::Off,
    ),
    ("@typescript-eslint/prefer-readonly", eslint::Severity::Off),
    (
        "@typescript-eslint/prefer-readonly-parameter-types",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/prefer-reduce-type-parameter",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/prefer-regexp-exec",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/prefer-return-this-type",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/prefer-string-starts-ends-with",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/promise-function-async",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/require-array-sort-compare",
        eslint::Severity::Off,
    ),
    ("@typescript-eslint/require-await", eslint::Severity::Off),
    (
        "@typescript-eslint/restrict-plus-operands",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/restrict-template-expressions",
        eslint::Severity::Off,
    ),
    ("@typescript-eslint/return-await", eslint::Severity::Off),
    (
        "@typescript-eslint/strict-boolean-expressions",
        eslint::Severity::Off,
    ),
    (
        "@typescript-eslint/switch-exhaustiveness-check",
        eslint::Severity::Off,
    ),
    ("@typescript-eslint/unbound-method", eslint::Severity::Off),
    (
        "@typescript-eslint/use-unknown-in-catch-callback-variable",
        eslint::Severity::Off,
    ),
];

/// `plugin:@typescript-eslint/all` preset.
/// See https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/typescript-eslint/src/configs/all.ts
pub(crate) const ALL: [(&str, eslint::Severity); 146] = [
    (
        "@typescript-eslint/adjacent-overload-signatures",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/array-type", eslint::Severity::Error),
    ("@typescript-eslint/await-thenable", eslint::Severity::Error),
    ("@typescript-eslint/ban-ts-comment", eslint::Severity::Error),
    (
        "@typescript-eslint/ban-tslint-comment",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/ban-types", eslint::Severity::Error),
    (
        "@typescript-eslint/class-literal-property-style",
        eslint::Severity::Error,
    ),
    ("class-methods-use-this", eslint::Severity::Off),
    (
        "@typescript-eslint/class-methods-use-this",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/consistent-generic-pub(crate) constructors",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/consistent-indexed-object-style",
        eslint::Severity::Error,
    ),
    ("consistent-return", eslint::Severity::Off),
    (
        "@typescript-eslint/consistent-return",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/consistent-type-assertions",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/consistent-type-definitions",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/consistent-type-exports",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/consistent-type-imports",
        eslint::Severity::Error,
    ),
    ("default-param-last", eslint::Severity::Off),
    (
        "@typescript-eslint/default-param-last",
        eslint::Severity::Error,
    ),
    ("dot-notation", eslint::Severity::Off),
    ("@typescript-eslint/dot-notation", eslint::Severity::Error),
    (
        "@typescript-eslint/explicit-function-return-type",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/explicit-member-accessibility",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/explicit-module-boundary-types",
        eslint::Severity::Error,
    ),
    ("init-declarations", eslint::Severity::Off),
    (
        "@typescript-eslint/init-declarations",
        eslint::Severity::Error,
    ),
    ("max-params", eslint::Severity::Off),
    ("@typescript-eslint/max-params", eslint::Severity::Error),
    (
        "@typescript-eslint/member-ordering",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/method-signature-style",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/naming_convention",
        eslint::Severity::Error,
    ),
    ("no-array-pub(crate) constructor", eslint::Severity::Off),
    (
        "@typescript-eslint/no-array-pub(crate) constructor",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-array-delete",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-base-to-string",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-confusing-non-null-assertion",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-confusing-void-expression",
        eslint::Severity::Error,
    ),
    ("no-dupe-class-members", eslint::Severity::Off),
    (
        "@typescript-eslint/no-dupe-class-members",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-duplicate-enum-values",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-duplicate-type-pub(crate) constituents",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-dynamic-delete",
        eslint::Severity::Error,
    ),
    ("no-empty-function", eslint::Severity::Off),
    (
        "@typescript-eslint/no-empty-function",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-empty-interface",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-explicit-any",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-extra-non-null-assertion",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-extraneous-class",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-floating-promises",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-for-in-array",
        eslint::Severity::Error,
    ),
    ("no-implied-eval", eslint::Severity::Off),
    (
        "@typescript-eslint/no-implied-eval",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-import-type-side-effects",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-inferrable-types",
        eslint::Severity::Error,
    ),
    ("no-invalid-this", eslint::Severity::Off),
    (
        "@typescript-eslint/no-invalid-this",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-invalid-void-type",
        eslint::Severity::Error,
    ),
    ("no-loop-func", eslint::Severity::Off),
    ("@typescript-eslint/no-loop-func", eslint::Severity::Error),
    ("no-loss-of-precision", eslint::Severity::Off),
    (
        "@typescript-eslint/no-loss-of-precision",
        eslint::Severity::Error,
    ),
    ("no-magic-numbers", eslint::Severity::Off),
    (
        "@typescript-eslint/no-magic-numbers",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-meaningless-void-operator",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/no-misused-new", eslint::Severity::Error),
    (
        "@typescript-eslint/no-misused-promises",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/no-mixed-enums", eslint::Severity::Error),
    ("@typescript-eslint/no-namespace", eslint::Severity::Error),
    (
        "@typescript-eslint/no-non-null-asserted-nullish-coalescing",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-non-null-asserted-optional-chain",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-non-null-assertion",
        eslint::Severity::Error,
    ),
    ("no-redeclare", eslint::Severity::Off),
    ("@typescript-eslint/no-redeclare", eslint::Severity::Error),
    (
        "@typescript-eslint/no-redundant-type-pub(crate) constituents",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-require-imports",
        eslint::Severity::Error,
    ),
    ("no-restricted-imports", eslint::Severity::Off),
    (
        "@typescript-eslint/no-restricted-imports",
        eslint::Severity::Error,
    ),
    ("no-shadow", eslint::Severity::Off),
    ("@typescript-eslint/no-shadow", eslint::Severity::Error),
    ("@typescript-eslint/no-this-alias", eslint::Severity::Error),
    (
        "@typescript-eslint/no-unnecessary-boolean-literal-compare",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unnecessary-condition",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unnecessary-qualifier",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unnecessary-type-arguments",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unnecessary-type-assertion",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unnecessary-type-pub(crate) constraint",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-argument",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-assignment",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/no-unsafe-call", eslint::Severity::Error),
    (
        "@typescript-eslint/no-unsafe-declaration-merging",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-enum-comparison",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-member-access",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-return",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-unsafe-unary-minus",
        eslint::Severity::Error,
    ),
    ("no-unused-expressions", eslint::Severity::Off),
    (
        "@typescript-eslint/no-unused-expressions",
        eslint::Severity::Error,
    ),
    ("no-unused-vars", eslint::Severity::Off),
    ("@typescript-eslint/no-unused-vars", eslint::Severity::Error),
    ("no-use-before-define", eslint::Severity::Off),
    (
        "@typescript-eslint/no-use-before-define",
        eslint::Severity::Error,
    ),
    ("no-useless-pub(crate) constructor", eslint::Severity::Off),
    (
        "@typescript-eslint/no-useless-pub(crate) constructor",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-useless-empty-export",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-useless-template-literals",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/no-var-requires",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/non-nullable-type-assertion-style",
        eslint::Severity::Error,
    ),
    ("no-throw-literal", eslint::Severity::Off),
    (
        "@typescript-eslint/only-throw-error",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/parameter-properties",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-as-pub(crate) const",
        eslint::Severity::Error,
    ),
    ("prefer-destructuring", eslint::Severity::Off),
    (
        "@typescript-eslint/prefer-destructuring",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-enum-initializers",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/prefer-find", eslint::Severity::Error),
    ("@typescript-eslint/prefer-for-of", eslint::Severity::Error),
    (
        "@typescript-eslint/prefer-function-type",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-includes",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-literal-enum-member",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-namespace-keyword",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-nullish-coalescing",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-optional-chain",
        eslint::Severity::Error,
    ),
    ("prefer-promise-reject-errors", eslint::Severity::Off),
    (
        "@typescript-eslint/prefer-promise-reject-errors",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-readonly",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-readonly-parameter-types",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-reduce-type-parameter",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-regexp-exec",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-return-this-type",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-string-starts-ends-with",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/prefer-ts-expect-error",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/promise-function-async",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/require-array-sort-compare",
        eslint::Severity::Error,
    ),
    ("require-await", eslint::Severity::Off),
    ("@typescript-eslint/require-await", eslint::Severity::Error),
    (
        "@typescript-eslint/restrict-plus-operands",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/restrict-template-expressions",
        eslint::Severity::Error,
    ),
    ("no-return-await", eslint::Severity::Off),
    ("@typescript-eslint/return-await", eslint::Severity::Error),
    (
        "@typescript-eslint/sort-type-pub(crate) constituents",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/strict-boolean-expressions",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/switch-exhaustiveness-check",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/triple-slash-reference",
        eslint::Severity::Error,
    ),
    ("@typescript-eslint/typedef", eslint::Severity::Error),
    ("@typescript-eslint/unbound-method", eslint::Severity::Error),
    (
        "@typescript-eslint/unified-signatures",
        eslint::Severity::Error,
    ),
    (
        "@typescript-eslint/use-unknown-in-catch-callback-variable",
        eslint::Severity::Error,
    ),
];
