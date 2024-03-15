use crate::run_cli;
use crate::snap_test::{assert_cli_snapshot, SnapshotPayload};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use biome_service::DynRef;
use bpaf::Args;
use std::path::Path;

#[test]
fn eslint_migrate() {
    let mut fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let configuration = r#"{ "linter": { "enabled": true } }"#;
    let eslint = r#"{
        "ignore_patterns": ["**/*.test.js"],
        "globals": [
            "var1": "writable",
            "var2": "readonly"
        ],
        "rules": {
            "eqeqeq": "warn",
            "no-eval": 1
            "no-extra-label": ["error"]
        },
        "overrides": [{
            "files": ["bin/*.js", "lib/*.js"],
            "excludedFiles": "*.test.js",
            "rules": {
                "eqeqeq": ["off"]
            }
        }],
        "unknownField": "ignored"
    }"#;

    let configuration_path = Path::new("biome.json");
    fs.insert(configuration_path.into(), configuration.as_bytes());

    let eslint_path = Path::new("./.eslintrc.json");
    fs.insert(eslint_path.into(), eslint.as_bytes());

    let result = run_cli(
        DynRef::Borrowed(&mut fs),
        &mut console,
        Args::from([("migrate"), "eslint"].as_slice()),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "eslint_migrate",
        fs,
        console,
        result,
    ));
}
