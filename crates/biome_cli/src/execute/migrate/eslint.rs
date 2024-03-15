use biome_console::{markup, Console, ConsoleExt};
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::Merge;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    VisitableType,
};
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use biome_fs::{FileSystem, OpenOptions};
use biome_json_parser::JsonParserOptions;
use biome_rowan::TextRange;
use biome_service::configuration::linter::RulePlainConfiguration;
use biome_service::DynRef;
use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use std::borrow::Cow;
use std::collections::hash_set;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::process::Command;
use std::vec;
use std::{any::TypeId, marker::PhantomData, ops::Deref};

use crate::diagnostics::MigrationDiagnostic;
use crate::CliDiagnostic;

use super::{
    eslint_barrel, eslint_import, eslint_jest, eslint_jsxa11y, eslint_react, eslint_react_hooks,
    eslint_typescript, eslint_unicorn,
};

/// This modules includes implementations for deserializing an eslint configuration
/// and convert it to Biome's configuration.
///
/// The conversion relies on:
/// - the generated [super::eslint_any_rule_to_biome::migrate_eslint_any_rule]
///   module that relies on Biome's rule metadata to determine
///   the equivalent Biome's rule of an Eslint rule
/// - hand-written handling of Biome rules that have options in the current module.

const CONFIG_FILES: [&str; 5] = [
    "./.eslintrc.js",
    "./.eslintrc.cjs",
    "./.eslintrc.yaml",
    "./.eslintrc.yml",
    "./.eslintrc.json",
];

pub(crate) fn read_eslintrc(
    fs: &DynRef<'_, dyn FileSystem>,
    console: &mut dyn Console,
) -> Result<ConfigData, CliDiagnostic> {
    for config_path_str in CONFIG_FILES {
        let path = Path::new(config_path_str);
        if fs.path_exists(path) {
            return load_config(fs, path, console);
        }
    }
    Err(CliDiagnostic::MigrateError(MigrationDiagnostic { reason: "The default ESlint configuration file `.eslintrc.*` was not found in the working directory.".to_string()}))
}

pub(crate) fn load_config(
    fs: &DynRef<'_, dyn FileSystem>,
    path: &Path,
    console: &mut dyn Console,
) -> Result<ConfigData, CliDiagnostic> {
    let (deserialized, diagnostics) = match path.extension().and_then(|file_ext| file_ext.to_str()) {
        Some("json") => {
            let mut file = fs.open_with_options(path, OpenOptions::default().read(true))?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            deserialize_from_json_str::<ConfigData>(
                &content,
                JsonParserOptions::default()
                    .with_allow_trailing_commas()
                    .with_allow_comments(),
                "",
            )
        },
        Some("js" | "cjs") => {
            let path_str = path.to_string_lossy();
            let output = Command::new("node")
                .arg("--eval")
                .arg(format!("import('{path_str}').then((c) => console.log(JSON.stringify(c.default)))"))
                .output()?;
            let content = String::from_utf8_lossy(&output.stdout);
            deserialize_from_json_str::<ConfigData>(
                &content,
                JsonParserOptions::default()
                    .with_allow_trailing_commas()
                    .with_allow_comments(),
                "",
            )
        },
        Some(ext) => return Err(CliDiagnostic::MigrateError(MigrationDiagnostic{ reason: format!("ESlint configuration ending with the extension `{ext}` are not supported.") })),
        None => return Err(CliDiagnostic::MigrateError(MigrationDiagnostic{ reason: "The ESlint configuration format cannot be determined because the file has no extension.".to_string() })),
    }.consume();
    let path_str = path.to_string_lossy();
    for diagnostic in diagnostics.into_iter().filter(|diag| {
        matches!(
            diag.severity(),
            biome_diagnostics::Severity::Fatal
                | biome_diagnostics::Severity::Error
                | biome_diagnostics::Severity::Warning
        )
    }) {
        let diagnostic = diagnostic.with_file_path(path_str.to_string());
        console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}});
    }
    if let Some(result) = deserialized {
        Ok(result)
    } else {
        Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
            reason: "Could not deserialize the Eslint configuration file".to_string(),
        }))
    }
}

// The following types corresponds to Eslint's config shape.
// See https://github.com/eslint/eslint/blob/ce838adc3b673e52a151f36da0eedf5876977514/lib/shared/types.js

#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct ConfigData {
    pub(crate) extends: Shorthand<String>,
    pub(crate) env: FxHashMap<String, bool>,
    pub(crate) globals: FxHashMap<String, GlobalConf>,
    /// The glob patterns that ignore to lint.
    pub(crate) ignore_patterns: Shorthand<String>,
    /// The parser options.
    pub(crate) rules: Rules,
    pub(crate) overrides: Vec<OverrideConfigData>,
}
impl ConfigData {
    pub(crate) fn from_preset(name: &str) -> Option<ConfigData> {
        let rules = match name {
            "eslint:recommended" => Rules::from_iter(ESLINT_RECOMMENDED.into_iter()),
            "eslint:all" => Rules::from_iter(ESLINT_ALL.into_iter()),
            "plugin:barrel/recommended" => Rules::from_iter(eslint_barrel::RECOMMENDED.into_iter()),
            "plugin:import/errors" => Rules::from_iter(eslint_import::ERRORS.into_iter()),
            "plugin:import/recommended" => Rules::from_iter(
                eslint_import::ERRORS
                    .into_iter()
                    .chain(eslint_import::WARNINGS),
            ),
            "plugin:import/warnings" => Rules::from_iter(eslint_import::WARNINGS.into_iter()),
            "plugin:jest/recommended" => Rules::from_iter(eslint_jest::RECOMMENDED.into_iter()),
            "plugin:jest/style" => Rules::from_iter(eslint_jest::STYLE.into_iter()),
            "plugin:jest/all" => Rules::from_iter(
                eslint_jest::RECOMMENDED
                    .into_iter()
                    .chain(eslint_jest::NON_RECOMMENDED_ERROR),
            ),
            "plugin:jsx-a11y/recommended" => Rules::from_iter(
                eslint_jsxa11y::STRICT
                    .into_iter()
                    .chain(eslint_jsxa11y::DISABLED_IN_RECOMMENDED),
            ),
            "plugin:jsx-a11y/strict" => Rules::from_iter(eslint_jsxa11y::STRICT.into_iter()),
            "plugin:react/all" => Rules::from_iter(
                eslint_react::RECOMMENDED
                    .into_iter()
                    .chain(eslint_react::NON_RECOMMENDED),
            ),
            "plugin:react/jsx-runtime" => Rules::from_iter(eslint_react::JSX_RUNTIME.into_iter()),
            "plugin:react-hooks/recommended" => {
                Rules::from_iter(eslint_react_hooks::RECOMMENDED.into_iter())
            }
            "plugin:react/recommended" => Rules::from_iter(eslint_react::RECOMMENDED.into_iter()),
            "plugin:@typescript-eslint/base" => Rules::default(),
            "plugin:@typescript-eslint/recommended" => {
                Rules::from_iter(eslint_typescript::RECOMMENDED.into_iter())
            }
            "plugin:@typescript-eslint/recommended-type-checked-only" => {
                Rules::from_iter(eslint_typescript::RECOMMENDED_TYPE_CHECKED_ONLY.into_iter())
            }
            "plugin:@typescript-eslint/recommended-type-checked" => Rules::from_iter(
                eslint_typescript::RECOMMENDED
                    .into_iter()
                    .chain(eslint_typescript::RECOMMENDED_TYPE_CHECKED_ONLY),
            ),
            "plugin:@typescript-eslint/strict" => {
                Rules::from_iter(eslint_typescript::STRICT.into_iter())
            }
            "plugin:@typescript-eslint/strict-type-checked-only" => {
                Rules::from_iter(eslint_typescript::STRICT_YYPE_CHECKED_ONLY.into_iter())
            }
            "plugin:@typescript-eslint/strict-type-checked" => Rules::from_iter(
                eslint_typescript::STRICT
                    .into_iter()
                    .chain(eslint_typescript::STRICT_YYPE_CHECKED_ONLY),
            ),
            "plugin:@typescript-eslint/stylistic" => {
                Rules::from_iter(eslint_typescript::STYLISTIC.into_iter())
            }
            "plugin:@typescript-eslint/stylistic-type-checked-only" => {
                Rules::from_iter(eslint_typescript::STYLISTIC_TYPE_CHECKED_ONLY.into_iter())
            }
            "plugin:@typescript-eslint/stylistic-type-checked" => Rules::from_iter(
                eslint_typescript::STYLISTIC
                    .into_iter()
                    .chain(eslint_typescript::STYLISTIC_TYPE_CHECKED_ONLY),
            ),
            "plugin:@typescript-eslint/disable-type-checked" => {
                Rules::from_iter(eslint_typescript::DISABLE_TYPE_CHECKED.into_iter())
            }
            "plugin:@typescript-eslint/all" => Rules::from_iter(eslint_typescript::ALL.into_iter()),
            "plugin:unicorn/recommeded" => {
                Rules::from_iter(eslint_unicorn::RECOMMENDED.into_iter())
            }
            "plugin:unicorn/all" => Rules::from_iter(
                eslint_unicorn::RECOMMENDED
                    .into_iter()
                    .chain(eslint_unicorn::NON_RECOMMENDED),
            ),
            _ => {
                return None;
            }
        };
        Some(ConfigData {
            rules,
            ..Default::default()
        })
    }

    pub(crate) fn resolve_extends(&mut self) {
        let extensions: Vec<_> = self
            .extends
            .0
            .iter()
            .filter_map(|preset| Self::from_preset(preset))
            .collect();
        self.extends = Shorthand::default();
        for ext in extensions {
            self.merge_with(ext);
        }
    }
}
impl Merge for ConfigData {
    fn merge_with(&mut self, mut other: Self) {
        self.extends.merge_with(other.extends);
        self.env.extend(other.env);
        self.globals.extend(other.globals);
        self.ignore_patterns.merge_with(other.ignore_patterns);
        self.rules.merge_with(other.rules);
        self.overrides.append(&mut other.overrides);
    }
}

#[derive(Debug)]
pub(crate) enum GlobalConf {
    Flag(bool),
    Qualifier(GlobalConfQualifier),
}
impl GlobalConf {
    pub(crate) fn is_enabled(&self) -> bool {
        match self {
            GlobalConf::Flag(result) => *result,
            GlobalConf::Qualifier(qualifier) => !matches!(qualifier, GlobalConfQualifier::Off),
        }
    }
}
impl Deserializable for GlobalConf {
    fn deserialize(
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
    ) -> Option<Self> {
        if value.is_type(VisitableType::STR) {
            Deserializable::deserialize(value, name, diagnostics).map(Self::Qualifier)
        } else {
            Deserializable::deserialize(value, name, diagnostics).map(Self::Flag)
        }
    }
}

#[derive(Debug, Deserializable)]
pub(crate) enum GlobalConfQualifier {
    Off,
    Readable,
    Readonly,
    Writable,
    Writeable,
}

#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct OverrideConfigData {
    pub(crate) extends: Shorthand<String>,
    pub(crate) env: FxHashMap<String, bool>,
    pub(crate) globals: FxHashMap<String, GlobalConf>,
    /// The glob patterns for excluded files.
    pub(crate) excluded_files: Shorthand<String>,
    /// The glob patterns for target files.
    pub(crate) files: Shorthand<String>,
    pub(crate) rules: Rules,
}

#[derive(Debug, Default)]
pub(crate) struct Shorthand<T>(Vec<T>);
impl<T> Merge for Shorthand<T> {
    fn merge_with(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }
}
impl<T> From<T> for Shorthand<T> {
    fn from(value: T) -> Self {
        Self(vec![value])
    }
}
impl<T> Deref for Shorthand<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> IntoIterator for Shorthand<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<T: Deserializable> Deserializable for Shorthand<T> {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        Some(Shorthand(if value.is_type(VisitableType::ARRAY) {
            Deserializable::deserialize(value, name, diagnostics)?
        } else {
            Vec::from_iter([Deserializable::deserialize(value, name, diagnostics)?])
        }))
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RuleConf<T = (), U = ()> {
    Severity(Severity),
    Option(Severity, T),
    Options(Severity, T, U),
    Spread(Severity, Vec<T>),
}
impl<T, U> RuleConf<T, U> {
    pub(crate) fn severity(&self) -> Severity {
        match self {
            Self::Severity(severity) => *severity,
            Self::Option(severity, _) => *severity,
            Self::Options(severity, _, _) => *severity,
            Self::Spread(severity, _) => *severity,
        }
    }
}
impl<T> RuleConf<T, ()> {
    pub(crate) fn into_vec(self) -> Vec<T> {
        match self {
            RuleConf::Severity(_) => vec![],
            RuleConf::Option(_, value) | RuleConf::Options(_, value, _) => vec![value],
            RuleConf::Spread(_, result) => result,
        }
    }
}
impl<T: Default, U: Default> RuleConf<T, U> {
    pub(crate) fn option_or_default(self) -> T {
        match self {
            RuleConf::Severity(_) | RuleConf::Options(_, _, _) | RuleConf::Spread(_, _) => {
                T::default()
            }
            RuleConf::Option(_, option) => option,
        }
    }
}
impl<T: Deserializable + 'static, U: Deserializable + 'static> Deserializable for RuleConf<T, U> {
    fn deserialize(
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor<T, U>(PhantomData<(T, U)>);
        impl<T: Deserializable + 'static, U: Deserializable + 'static> DeserializationVisitor
            for Visitor<T, U>
        {
            type Output = RuleConf<T, U>;
            const EXPECTED_TYPE: VisitableType = VisitableType::ARRAY;
            fn visit_array(
                self,
                values: impl Iterator<Item = Option<impl DeserializableValue>>,
                range: TextRange,
                _name: &str,
                diagnostics: &mut Vec<DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                let mut values = values.flatten();
                let Some(first_value) = values.next() else {
                    diagnostics.push(
                        DeserializationDiagnostic::new("A severity is expected.").with_range(range),
                    );
                    return None;
                };
                let severity = Deserializable::deserialize(&first_value, "", diagnostics)?;
                if TypeId::of::<T>() == TypeId::of::<()>() {
                    return Some(RuleConf::Severity(severity));
                }
                let Some(second_value) = values.next() else {
                    return Some(RuleConf::Severity(severity));
                };
                let Some(option) = T::deserialize(&second_value, "", diagnostics) else {
                    // Recover by ignoring the failed deserialization
                    return Some(RuleConf::Severity(severity));
                };
                let Some(third_value) = values.next() else {
                    return Some(RuleConf::Option(severity, option));
                };
                if TypeId::of::<U>() != TypeId::of::<()>() {
                    if let Some(option2) = U::deserialize(&third_value, "", diagnostics) {
                        return Some(RuleConf::Options(severity, option, option2));
                    } else {
                        // Recover by ignoring the failed deserialization
                        return Some(RuleConf::Option(severity, option));
                    }
                }
                let Some(option2) = T::deserialize(&third_value, "", diagnostics) else {
                    // Recover by ignoring the failed deserialization
                    return Some(RuleConf::Option(severity, option));
                };
                let mut spread = Vec::new();
                spread.push(option);
                spread.push(option2);
                spread.extend(values.filter_map(|val| T::deserialize(&val, "", diagnostics)));
                Some(RuleConf::Spread(severity, spread))
            }
        }
        if value.is_type(VisitableType::NUMBER) || value.is_type(VisitableType::STR) {
            Deserializable::deserialize(value, name, diagnostics).map(RuleConf::Severity)
        } else {
            value.deserialize(Visitor(PhantomData), name, diagnostics)
        }
    }
}

#[derive(Clone, Copy, Debug, Deserializable)]
#[deserializable(try_from = "NumberOrString")]
pub(crate) enum Severity {
    Off,
    Warn,
    Error,
}
impl TryFrom<NumberOrString> for Severity {
    type Error = &'static str;

    fn try_from(value: NumberOrString) -> Result<Self, &'static str> {
        match value {
            NumberOrString::Number(n) => match n {
                0 => Ok(Severity::Off),
                1 => Ok(Severity::Warn),
                2 => Ok(Severity::Error),
                _ => Err("Severity should be 0, 1 or 2."),
            },
            NumberOrString::String(s) => match s.as_ref() {
                "off" => Ok(Severity::Off),
                "warn" => Ok(Severity::Warn),
                "error" => Ok(Severity::Error),
                _ => Err("Severity should be 'off', 'warn' or 'error'."),
            },
        }
    }
}
impl From<Severity> for RulePlainConfiguration {
    fn from(value: Severity) -> RulePlainConfiguration {
        match value {
            Severity::Off => RulePlainConfiguration::Off,
            Severity::Warn => RulePlainConfiguration::Warn,
            Severity::Error => RulePlainConfiguration::Error,
        }
    }
}
#[derive(Debug, Clone)]
enum NumberOrString {
    Number(u64),
    String(String),
}
impl Deserializable for NumberOrString {
    fn deserialize(
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
    ) -> Option<Self> {
        Some(if value.is_type(VisitableType::STR) {
            Self::String(Deserializable::deserialize(value, name, diagnostics)?)
        } else {
            Self::Number(Deserializable::deserialize(value, name, diagnostics)?)
        })
    }
}

#[derive(Debug, Default)]
pub(crate) struct Rules(pub(crate) FxHashSet<Rule>);
impl Rules {
    fn from_iter(rules: impl Iterator<Item = (&'static str, Severity)>) -> Self {
        Self(
            rules
                .map(|(name, severity)| Rule::new(name, severity))
                .collect(),
        )
    }
}
impl Merge for Rules {
    fn merge_with(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}
impl IntoIterator for Rules {
    type Item = Rule;
    type IntoIter = hash_set::IntoIter<Rule>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl Deref for Rules {
    type Target = FxHashSet<Rule>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deserializable for Rules {
    fn deserialize(
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Rules;
            const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
            fn visit_map(
                self,
                members: impl Iterator<
                    Item = Option<(
                        impl biome_deserialize::DeserializableValue,
                        impl biome_deserialize::DeserializableValue,
                    )>,
                >,
                _range: biome_rowan::TextRange,
                name: &str,
                diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                use biome_deserialize::Text;
                let mut result = FxHashSet::default();
                for (key, value) in members.flatten() {
                    let Some(rule_name) = Text::deserialize(&key, "", diagnostics) else {
                        continue;
                    };
                    match rule_name.text() {
                        // Eslint rules with options that we handle
                        "no-restricted-globals" => {
                            if let Some(conf) = RuleConf::deserialize(&value, name, diagnostics) {
                                result.insert(Rule::NoRestrictedGlobals(conf));
                            }
                        }
                        // Eslint plugin rules with options that we handle
                        "jsx-a11y/aria-role" => {
                            if let Some(conf) = RuleConf::deserialize(&value, name, diagnostics) {
                                result.insert(Rule::Jsxa11yArioaRoles(conf));
                            }
                        }
                        "@typescript-eslint/array-type" => {
                            if let Some(conf) = RuleConf::deserialize(&value, name, diagnostics) {
                                result.insert(Rule::TypeScriptArrayType(conf));
                            }
                        }
                        "@typescript-eslint/naming-convention" => {
                            if let Some(conf) = RuleConf::deserialize(&value, name, diagnostics) {
                                result.insert(Rule::TypeScriptNamingConvention(conf));
                            }
                        }
                        "unicorn/filename-case" => {
                            if let Some(conf) = RuleConf::deserialize(&value, name, diagnostics) {
                                result.insert(Rule::UnicornFilenameCase(conf));
                            }
                        }
                        // Other rules
                        rule_name => {
                            if let Some(conf) =
                                RuleConf::<()>::deserialize(&value, name, diagnostics)
                            {
                                result.insert(Rule::Any(
                                    Cow::Owned(rule_name.to_string()),
                                    conf.severity(),
                                ));
                            }
                        }
                    }
                }
                Some(Rules(result))
            }
        }
        value.deserialize(Visitor, name, diagnostics)
    }
}

#[derive(Debug)]
pub(crate) enum NoRestrictedGlobal {
    Plain(String),
    WithMessage(GlobalWithMessage),
}
impl NoRestrictedGlobal {
    pub(crate) fn into_name(self) -> String {
        match self {
            NoRestrictedGlobal::Plain(name) => name,
            NoRestrictedGlobal::WithMessage(named) => named.name,
        }
    }
}
impl Deserializable for NoRestrictedGlobal {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        if value.is_type(VisitableType::STR) {
            Deserializable::deserialize(value, name, diagnostics).map(NoRestrictedGlobal::Plain)
        } else {
            Deserializable::deserialize(value, name, diagnostics)
                .map(NoRestrictedGlobal::WithMessage)
        }
    }
}
#[derive(Debug, Default, Deserializable)]
pub(crate) struct GlobalWithMessage {
    name: String,
    message: String,
}

#[derive(Debug)]
pub(crate) enum Rule {
    /// Any rule without its options.
    Any(Cow<'static, str>, Severity),
    // Eslint rules with its options
    // We use this to configure equivalent Bione's rules.
    NoRestrictedGlobals(RuleConf<Box<NoRestrictedGlobal>>),
    // Eslint plugins
    Jsxa11yArioaRoles(RuleConf<Box<eslint_jsxa11y::AriaRoleOptions>>),
    TypeScriptArrayType(RuleConf<eslint_typescript::ArrayTypeOptions>),
    TypeScriptNamingConvention(RuleConf<Box<eslint_typescript::NamingConventionSelection>>),
    UnicornFilenameCase(RuleConf<eslint_unicorn::FilenameCaseOptions>),
}
impl Rule {
    pub(crate) fn new(name: &'static str, severity: Severity) -> Self {
        match name {
            "no-restricted-globals" => Rule::NoRestrictedGlobals(RuleConf::Severity(severity)),
            "jsx-a11y/aria-role" => Rule::Jsxa11yArioaRoles(RuleConf::Severity(severity)),
            "@typescript-eslint/array-type" => {
                Rule::TypeScriptArrayType(RuleConf::Severity(severity))
            }
            "@typescript-eslint/naming_convention" => {
                Rule::TypeScriptNamingConvention(RuleConf::Severity(severity))
            }
            "unicorn/filename-case" => Rule::UnicornFilenameCase(RuleConf::Severity(severity)),
            name => Self::Any(Cow::Borrowed(name), severity),
        }
    }

    pub(crate) fn name(&self) -> Cow<'static, str> {
        match self {
            Rule::Any(name, _) => name.clone(),
            Rule::NoRestrictedGlobals(_) => Cow::Borrowed("no-restricted-globals"),
            Rule::Jsxa11yArioaRoles(_) => Cow::Borrowed("jsx-a11y/aria-role"),
            Rule::TypeScriptArrayType(_) => Cow::Borrowed("@typescript-eslint/array-type"),
            Rule::TypeScriptNamingConvention(_) => {
                Cow::Borrowed("@typescript-eslint/naming_convention")
            }
            Rule::UnicornFilenameCase(_) => Cow::Borrowed("unicorn/filename-case"),
        }
    }
}
impl Eq for Rule {}
impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

/// `eslint:recommended` preset.
/// See https://github.com/eslint/eslint/blob/main/packages/js/src/configs/eslint-recommended.js
const ESLINT_RECOMMENDED: [(&str, Severity); 61] = [
    ("constructor-super", Severity::Error),
    ("for-direction", Severity::Error),
    ("getter-return", Severity::Error),
    ("no-async-promise-executor", Severity::Error),
    ("no-case-declarations", Severity::Error),
    ("no-class-assign", Severity::Error),
    ("no-compare-neg-zero", Severity::Error),
    ("no-cond-assign", Severity::Error),
    ("no-const-assign", Severity::Error),
    ("no-constant-binary-expression", Severity::Error),
    ("no-constant-condition", Severity::Error),
    ("no-control-regex", Severity::Error),
    ("no-debugger", Severity::Error),
    ("no-delete-var", Severity::Error),
    ("no-dupe-args", Severity::Error),
    ("no-dupe-class-members", Severity::Error),
    ("no-dupe-else-if", Severity::Error),
    ("no-dupe-keys", Severity::Error),
    ("no-duplicate-case", Severity::Error),
    ("no-empty", Severity::Error),
    ("no-empty-character-class", Severity::Error),
    ("no-empty-pattern", Severity::Error),
    ("no-empty-static-block", Severity::Error),
    ("no-ex-assign", Severity::Error),
    ("no-extra-boolean-cast", Severity::Error),
    ("no-fallthrough", Severity::Error),
    ("no-func-assign", Severity::Error),
    ("no-global-assign", Severity::Error),
    ("no-import-assign", Severity::Error),
    ("no-invalid-regexp", Severity::Error),
    ("no-irregular-whitespace", Severity::Error),
    ("no-loss-of-precision", Severity::Error),
    ("no-misleading-character-class", Severity::Error),
    ("no-new-native-nonconstructor", Severity::Error),
    ("no-nonoctal-decimal-escape", Severity::Error),
    ("no-obj-calls", Severity::Error),
    ("no-octal", Severity::Error),
    ("no-prototype-builtins", Severity::Error),
    ("no-redeclare", Severity::Error),
    ("no-regex-spaces", Severity::Error),
    ("no-self-assign", Severity::Error),
    ("no-setter-return", Severity::Error),
    ("no-shadow-restricted-names", Severity::Error),
    ("no-sparse-arrays", Severity::Error),
    ("no-this-before-super", Severity::Error),
    ("no-undef", Severity::Error),
    ("no-unexpected-multiline", Severity::Error),
    ("no-unreachable", Severity::Error),
    ("no-unsafe-finally", Severity::Error),
    ("no-unsafe-negation", Severity::Error),
    ("no-unsafe-optional-chaining", Severity::Error),
    ("no-unused-labels", Severity::Error),
    ("no-unused-private-class-members", Severity::Error),
    ("no-unused-vars", Severity::Error),
    ("no-useless-backreference", Severity::Error),
    ("no-useless-catch", Severity::Error),
    ("no-useless-escape", Severity::Error),
    ("no-with", Severity::Error),
    ("require-yield", Severity::Error),
    ("use-isnan", Severity::Error),
    ("valid-typeof", Severity::Error),
];

/// `eslint:all` preset.
/// See https://github.com/eslint/eslint/blob/main/packages/js/src/configs/eslint-recommended.js
const ESLINT_ALL: [(&str, Severity); 199] = [
    ("accessor-pairs", Severity::Error),
    ("array-callback-return", Severity::Error),
    ("arrow-body-style", Severity::Error),
    ("block-scoped-var", Severity::Error),
    ("camelcase", Severity::Error),
    ("capitalized-comments", Severity::Error),
    ("class-methods-use-this", Severity::Error),
    ("complexity", Severity::Error),
    ("consistent-return", Severity::Error),
    ("consistent-this", Severity::Error),
    ("constructor-super", Severity::Error),
    ("curly", Severity::Error),
    ("default-case", Severity::Error),
    ("default-case-last", Severity::Error),
    ("default-param-last", Severity::Error),
    ("dot-notation", Severity::Error),
    ("eqeqeq", Severity::Error),
    ("for-direction", Severity::Error),
    ("func-name-matching", Severity::Error),
    ("func-names", Severity::Error),
    ("func-style", Severity::Error),
    ("getter-return", Severity::Error),
    ("grouped-accessor-pairs", Severity::Error),
    ("guard-for-in", Severity::Error),
    ("id-denylist", Severity::Error),
    ("id-length", Severity::Error),
    ("id-match", Severity::Error),
    ("init-declarations", Severity::Error),
    ("line-comment-position", Severity::Error),
    ("logical-assignment-operators", Severity::Error),
    ("max-classes-per-file", Severity::Error),
    ("max-depth", Severity::Error),
    ("max-lines", Severity::Error),
    ("max-lines-per-function", Severity::Error),
    ("max-nested-callbacks", Severity::Error),
    ("max-params", Severity::Error),
    ("max-statements", Severity::Error),
    ("multiline-comment-style", Severity::Error),
    ("new-cap", Severity::Error),
    ("no-alert", Severity::Error),
    ("no-array-constructor", Severity::Error),
    ("no-async-promise-executor", Severity::Error),
    ("no-await-in-loop", Severity::Error),
    ("no-bitwise", Severity::Error),
    ("no-caller", Severity::Error),
    ("no-case-declarations", Severity::Error),
    ("no-class-assign", Severity::Error),
    ("no-compare-neg-zero", Severity::Error),
    ("no-cond-assign", Severity::Error),
    ("no-console", Severity::Error),
    ("no-const-assign", Severity::Error),
    ("no-constant-binary-expression", Severity::Error),
    ("no-constant-condition", Severity::Error),
    ("no-constructor-return", Severity::Error),
    ("no-continue", Severity::Error),
    ("no-control-regex", Severity::Error),
    ("no-debugger", Severity::Error),
    ("no-delete-var", Severity::Error),
    ("no-div-regex", Severity::Error),
    ("no-dupe-args", Severity::Error),
    ("no-dupe-class-members", Severity::Error),
    ("no-dupe-else-if", Severity::Error),
    ("no-dupe-keys", Severity::Error),
    ("no-duplicate-case", Severity::Error),
    ("no-duplicate-imports", Severity::Error),
    ("no-else-return", Severity::Error),
    ("no-empty", Severity::Error),
    ("no-empty-character-class", Severity::Error),
    ("no-empty-function", Severity::Error),
    ("no-empty-pattern", Severity::Error),
    ("no-empty-static-block", Severity::Error),
    ("no-eq-null", Severity::Error),
    ("no-eval", Severity::Error),
    ("no-ex-assign", Severity::Error),
    ("no-extend-native", Severity::Error),
    ("no-extra-bind", Severity::Error),
    ("no-extra-boolean-cast", Severity::Error),
    ("no-extra-label", Severity::Error),
    ("no-fallthrough", Severity::Error),
    ("no-func-assign", Severity::Error),
    ("no-global-assign", Severity::Error),
    ("no-implicit-coercion", Severity::Error),
    ("no-implicit-globals", Severity::Error),
    ("no-implied-eval", Severity::Error),
    ("no-import-assign", Severity::Error),
    ("no-inline-comments", Severity::Error),
    ("no-inner-declarations", Severity::Error),
    ("no-invalid-regexp", Severity::Error),
    ("no-invalid-this", Severity::Error),
    ("no-irregular-whitespace", Severity::Error),
    ("no-iterator", Severity::Error),
    ("no-label-var", Severity::Error),
    ("no-labels", Severity::Error),
    ("no-lone-blocks", Severity::Error),
    ("no-lonely-if", Severity::Error),
    ("no-loop-func", Severity::Error),
    ("no-loss-of-precision", Severity::Error),
    ("no-magic-numbers", Severity::Error),
    ("no-misleading-character-class", Severity::Error),
    ("no-multi-assign", Severity::Error),
    ("no-multi-str", Severity::Error),
    ("no-negated-condition", Severity::Error),
    ("no-nested-ternary", Severity::Error),
    ("no-new", Severity::Error),
    ("no-new-func", Severity::Error),
    ("no-new-native-nonconstructor", Severity::Error),
    ("no-new-wrappers", Severity::Error),
    ("no-nonoctal-decimal-escape", Severity::Error),
    ("no-obj-calls", Severity::Error),
    ("no-object-constructor", Severity::Error),
    ("no-octal", Severity::Error),
    ("no-octal-escape", Severity::Error),
    ("no-param-reassign", Severity::Error),
    ("no-plusplus", Severity::Error),
    ("no-promise-executor-return", Severity::Error),
    ("no-proto", Severity::Error),
    ("no-prototype-builtins", Severity::Error),
    ("no-redeclare", Severity::Error),
    ("no-regex-spaces", Severity::Error),
    ("no-restricted-exports", Severity::Error),
    ("no-restricted-globals", Severity::Error),
    ("no-restricted-imports", Severity::Error),
    ("no-restricted-properties", Severity::Error),
    ("no-restricted-syntax", Severity::Error),
    ("no-return-assign", Severity::Error),
    ("no-script-url", Severity::Error),
    ("no-self-assign", Severity::Error),
    ("no-self-compare", Severity::Error),
    ("no-sequences", Severity::Error),
    ("no-setter-return", Severity::Error),
    ("no-shadow", Severity::Error),
    ("no-shadow-restricted-names", Severity::Error),
    ("no-sparse-arrays", Severity::Error),
    ("no-template-curly-in-string", Severity::Error),
    ("no-ternary", Severity::Error),
    ("no-this-before-super", Severity::Error),
    ("no-throw-literal", Severity::Error),
    ("no-undef", Severity::Error),
    ("no-undef-init", Severity::Error),
    ("no-undefined", Severity::Error),
    ("no-underscore-dangle", Severity::Error),
    ("no-unexpected-multiline", Severity::Error),
    ("no-unmodified-loop-condition", Severity::Error),
    ("no-unneeded-ternary", Severity::Error),
    ("no-unreachable", Severity::Error),
    ("no-unreachable-loop", Severity::Error),
    ("no-unsafe-finally", Severity::Error),
    ("no-unsafe-negation", Severity::Error),
    ("no-unsafe-optional-chaining", Severity::Error),
    ("no-unused-expressions", Severity::Error),
    ("no-unused-labels", Severity::Error),
    ("no-unused-private-class-members", Severity::Error),
    ("no-unused-vars", Severity::Error),
    ("no-use-before-define", Severity::Error),
    ("no-useless-assignment", Severity::Error),
    ("no-useless-backreference", Severity::Error),
    ("no-useless-call", Severity::Error),
    ("no-useless-catch", Severity::Error),
    ("no-useless-computed-key", Severity::Error),
    ("no-useless-concat", Severity::Error),
    ("no-useless-constructor", Severity::Error),
    ("no-useless-escape", Severity::Error),
    ("no-useless-rename", Severity::Error),
    ("no-useless-return", Severity::Error),
    ("no-var", Severity::Error),
    ("no-void", Severity::Error),
    ("no-warning-comments", Severity::Error),
    ("no-with", Severity::Error),
    ("object-shorthand", Severity::Error),
    ("one-var", Severity::Error),
    ("operator-assignment", Severity::Error),
    ("prefer-arrow-callback", Severity::Error),
    ("prefer-const", Severity::Error),
    ("prefer-destructuring", Severity::Error),
    ("prefer-exponentiation-operator", Severity::Error),
    ("prefer-named-capture-group", Severity::Error),
    ("prefer-numeric-literals", Severity::Error),
    ("prefer-object-has-own", Severity::Error),
    ("prefer-object-spread", Severity::Error),
    ("prefer-promise-reject-errors", Severity::Error),
    ("prefer-regex-literals", Severity::Error),
    ("prefer-rest-params", Severity::Error),
    ("prefer-spread", Severity::Error),
    ("prefer-template", Severity::Error),
    ("radix", Severity::Error),
    ("require-atomic-updates", Severity::Error),
    ("require-await", Severity::Error),
    ("require-unicode-regexp", Severity::Error),
    ("require-yield", Severity::Error),
    ("sort-imports", Severity::Error),
    ("sort-keys", Severity::Error),
    ("sort-vars", Severity::Error),
    ("strict", Severity::Error),
    ("symbol-description", Severity::Error),
    ("unicode-bom", Severity::Error),
    ("use-isnan", Severity::Error),
    ("valid-typeof", Severity::Error),
    ("vars-on-top", Severity::Error),
    ("yoda", Severity::Error),
];
