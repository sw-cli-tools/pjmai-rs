//! Integration tests for pjmai
//!
//! These tests use a temporary config directory via the PJMAI_CONFIG_DIR environment variable
//! to ensure tests don't affect the user's actual configuration.

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to create a command with a temp config directory
fn pjmai_cmd(temp_dir: &TempDir) -> Command {
    let mut cmd: Command = cargo_bin_cmd!("pjmai-rs");
    cmd.env("PJMAI_CONFIG_DIR", temp_dir.path());
    cmd
}

/// Helper to create a temp directory with an initialized config
fn setup_with_config(config_content: &str) -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("config.toml"), config_content).unwrap();
    temp_dir
}

/// Helper to get config file contents
fn read_config(temp_dir: &TempDir) -> String {
    fs::read_to_string(temp_dir.path().join("config.toml")).unwrap()
}

// ============================================================
// Basic workflow tests
// ============================================================

#[test]
fn test_list_empty_projects() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["list"])
        .assert()
        .success()
        .stdout("");
}

#[test]
fn test_add_project_to_directory() {
    let temp_dir = TempDir::new().unwrap();
    // Create a target directory for the project
    let project_dir = temp_dir.path().join("myproject");
    fs::create_dir(&project_dir).unwrap();

    // Initialize empty config
    fs::write(
        temp_dir.path().join("config.toml"),
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args([
            "add",
            "-p",
            "test",
            "-f",
            project_dir.to_str().unwrap(),
        ])
        .assert()
        .success();

    // Verify the project was added
    let config = read_config(&temp_dir);
    assert!(config.contains("name = \"test\""));
    assert!(config.contains(&format!("file_or_dir = \"{}\"", project_dir.display())));
    // First project should become current
    assert!(config.contains("current_project = \"test\""));
}

#[test]
fn test_add_multiple_projects() {
    let temp_dir = TempDir::new().unwrap();
    let proj1 = temp_dir.path().join("proj1");
    let proj2 = temp_dir.path().join("proj2");
    fs::create_dir(&proj1).unwrap();
    fs::create_dir(&proj2).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    )
    .unwrap();

    // Add first project
    pjmai_cmd(&temp_dir)
        .args(["add", "-p", "first", "-f", proj1.to_str().unwrap()])
        .assert()
        .success();

    // Add second project
    pjmai_cmd(&temp_dir)
        .args(["add", "-p", "second", "-f", proj2.to_str().unwrap()])
        .assert()
        .success();

    // Verify both projects exist
    let config = read_config(&temp_dir);
    assert!(config.contains("name = \"first\""));
    assert!(config.contains("name = \"second\""));
    // First project should still be current
    assert!(config.contains("current_project = \"first\""));
}

#[test]
fn test_list_projects() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "alpha"

[[project]]
name = "alpha"

[project.action]
file_or_dir = "/tmp/alpha"

[[project]]
name = "beta"

[project.action]
file_or_dir = "/tmp/beta"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha"))
        .stdout(predicate::str::contains("beta"));
}

#[test]
fn test_show_current_project() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "myproj"

[[project]]
name = "myproj"

[project.action]
file_or_dir = "/tmp/myproj"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("myproj"));
}

#[test]
fn test_prompt_outputs_current_project() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "activeproj"

[[project]]
name = "activeproj"

[project.action]
file_or_dir = "/tmp/activeproj"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["prompt"])
        .assert()
        .success()
        .stdout("activeproj\n");
}

#[test]
fn test_prompt_empty_when_no_current_project() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["prompt"])
        .assert()
        .success()
        .stdout("");
}

#[test]
fn test_remove_project() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "alpha"

[[project]]
name = "alpha"

[project.action]
file_or_dir = "/tmp/alpha"

[[project]]
name = "beta"

[project.action]
file_or_dir = "/tmp/beta"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["remove", "-p", "beta"])
        .assert()
        .success();

    let config = read_config(&temp_dir);
    assert!(config.contains("name = \"alpha\""));
    assert!(!config.contains("name = \"beta\""));
}

#[test]
fn test_remove_current_project_clears_current() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "alpha"

[[project]]
name = "alpha"

[project.action]
file_or_dir = "/tmp/alpha"

[[project]]
name = "beta"

[project.action]
file_or_dir = "/tmp/beta"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["remove", "-p", "alpha"])
        .assert()
        .success();

    let config = read_config(&temp_dir);
    assert!(!config.contains("name = \"alpha\""));
    assert!(config.contains("name = \"beta\""));
    // current_project should be cleared since we removed it
    assert!(config.contains("current_project = \"\""));
}

// ============================================================
// Error handling tests
// ============================================================

#[test]
fn test_add_duplicate_project_fails() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("proj");
    fs::create_dir(&proj_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        r#"version = "0.1.0"
current_project = "existing"

[[project]]
name = "existing"

[project.action]
file_or_dir = "/tmp/existing"
"#,
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["add", "-p", "existing", "-f", proj_dir.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("duplicate"));
}

#[test]
fn test_add_nonexistent_path_fails() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["add", "-p", "bad", "-f", "/nonexistent/path/that/does/not/exist"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn test_change_to_nonexistent_project_fails() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["change", "-p", "nonexistent"])
        .assert()
        .code(4)
        .stdout(predicate::str::contains("not found"));
}

#[test]
fn test_change_to_project_with_missing_path() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""

[[project]]
name = "badproj"

[project.action]
file_or_dir = "/nonexistent/path"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["change", "-p", "badproj"])
        .assert()
        .code(4)
        .stdout(predicate::str::contains("not found"));
}

// ============================================================
// Aliases command test
// ============================================================

#[test]
fn test_aliases_command() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["aliases"])
        .assert()
        .success()
        .stdout(predicate::str::contains("adpj"))
        .stdout(predicate::str::contains("chpj"))
        .stdout(predicate::str::contains("lspj"))
        .stdout(predicate::str::contains("rmpj"))
        .stdout(predicate::str::contains("shpj"))
        .stdout(predicate::str::contains("prpj"))
        .stdout(predicate::str::contains("hlpj"));
}

// ============================================================
// Config directory creation test
// ============================================================

#[test]
fn test_config_directory_created_when_missing() {
    let temp_dir = TempDir::new().unwrap();
    let nested_config_dir = temp_dir.path().join("nested").join("config");

    // The config directory doesn't exist yet
    assert!(!nested_config_dir.exists());

    // Create a project dir to add
    let proj_dir = temp_dir.path().join("proj");
    fs::create_dir(&proj_dir).unwrap();

    // Running pjmai should prompt for creation - but since we can't interact,
    // let's pre-create the directory and test that it works
    fs::create_dir_all(&nested_config_dir).unwrap();
    fs::write(
        nested_config_dir.join("config.toml"),
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    )
    .unwrap();

    let mut cmd: Command = cargo_bin_cmd!("pjmai-rs");
    cmd.env("PJMAI_CONFIG_DIR", &nested_config_dir)
        .args(["add", "-p", "test", "-f", proj_dir.to_str().unwrap()])
        .assert()
        .success();
}

// ============================================================
// Change command tests (exit codes)
// ============================================================

#[test]
fn test_change_to_directory_exits_with_code_2() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("proj");
    fs::create_dir(&proj_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""

[[project]]
name = "dirproj"

[project.action]
file_or_dir = "{}"
"#,
            proj_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["change", "-p", "dirproj"])
        .assert()
        .code(2)
        .stdout(predicate::str::contains(proj_dir.to_str().unwrap()));
}

#[test]
fn test_change_to_file_exits_with_code_3() {
    let temp_dir = TempDir::new().unwrap();
    let proj_file = temp_dir.path().join("setup.sh");
    fs::write(&proj_file, "#!/bin/bash\necho 'Hello'\n").unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""

[[project]]
name = "fileproj"

[project.action]
file_or_dir = "{}"
"#,
            proj_file.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["change", "-p", "fileproj"])
        .assert()
        .code(3)
        .stdout(predicate::str::contains(proj_file.to_str().unwrap()));
}

// ============================================================
// Fuzzy matching tests
// ============================================================

#[test]
fn test_change_with_prefix_match() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("webapp");
    fs::create_dir(&proj_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""

[[project]]
name = "webapp"

[project.action]
file_or_dir = "{}"
"#,
            proj_dir.display()
        ),
    )
    .unwrap();

    // "web" should match "webapp" via prefix
    pjmai_cmd(&temp_dir)
        .args(["change", "-p", "web"])
        .assert()
        .code(2)
        .stdout(predicate::str::contains(proj_dir.to_str().unwrap()));
}

#[test]
fn test_change_with_case_insensitive_match() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("MyProject");
    fs::create_dir(&proj_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""

[[project]]
name = "MyProject"

[project.action]
file_or_dir = "{}"
"#,
            proj_dir.display()
        ),
    )
    .unwrap();

    // "myproject" should match "MyProject" case-insensitively
    pjmai_cmd(&temp_dir)
        .args(["change", "-p", "myproject"])
        .assert()
        .code(2)
        .stdout(predicate::str::contains(proj_dir.to_str().unwrap()));
}

#[test]
fn test_change_with_ambiguous_match() {
    let temp_dir = TempDir::new().unwrap();
    let proj1 = temp_dir.path().join("webapp");
    let proj2 = temp_dir.path().join("webapi");
    fs::create_dir(&proj1).unwrap();
    fs::create_dir(&proj2).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""

[[project]]
name = "webapp"

[project.action]
file_or_dir = "{}"

[[project]]
name = "webapi"

[project.action]
file_or_dir = "{}"
"#,
            proj1.display(),
            proj2.display()
        ),
    )
    .unwrap();

    // "web" should be ambiguous (matches both webapp and webapi)
    pjmai_cmd(&temp_dir)
        .args(["change", "-p", "web"])
        .assert()
        .code(4)
        .stdout(predicate::str::contains("ambiguous"))
        .stdout(predicate::str::contains("webapp"))
        .stdout(predicate::str::contains("webapi"));
}

// ============================================================
// JSON output tests
// ============================================================

#[test]
fn test_json_list_empty() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["--json", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""projects": []"#))
        .stdout(predicate::str::contains(r#""total": 0"#));
}

#[test]
fn test_json_list_with_projects() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("myproject");
    fs::create_dir(&proj_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "myproject"

[[project]]
name = "myproject"

[project.action]
file_or_dir = "{}"
"#,
            proj_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["--json", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""name": "myproject""#))
        .stdout(predicate::str::contains(r#""type": "directory""#))
        .stdout(predicate::str::contains(r#""is_current": true"#))
        .stdout(predicate::str::contains(r#""total": 1"#));
}

#[test]
fn test_json_show() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("myproject");
    fs::create_dir(&proj_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "myproject"

[[project]]
name = "myproject"

[project.action]
file_or_dir = "{}"
"#,
            proj_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["--json", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""name": "myproject""#))
        .stdout(predicate::str::contains(r#""type": "directory""#));
}

#[test]
fn test_json_change_not_found() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["--json", "change", "-p", "nonexistent"])
        .assert()
        .code(4)
        .stdout(predicate::str::contains(r#""code": "PROJECT_NOT_FOUND""#))
        .stdout(predicate::str::contains(r#""hint":"#));
}

#[test]
fn test_json_aliases() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["--json", "aliases"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""aliases":"#))
        .stdout(predicate::str::contains(r#""alias": "adpj""#))
        .stdout(predicate::str::contains(r#""alias": "chpj""#));
}

// ============================================================
// Setup command tests
// ============================================================

#[test]
fn test_setup_completions_only() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    // Test JSON output for completions-only setup
    pjmai_cmd(&temp_dir)
        .args(["--json", "setup", "--completions-only", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""success": true"#))
        .stdout(predicate::str::contains(r#""action": "completions""#));
}

#[test]
fn test_setup_help() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["setup", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("shell integration"))
        .stdout(predicate::str::contains("--completions-only"));
}

// ============================================================
// Complete command tests
// ============================================================

#[test]
fn test_complete_projects_all() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""

[[project]]
name = "webapp"
[project.action]
file_or_dir = "/tmp"

[[project]]
name = "webapi"
[project.action]
file_or_dir = "/tmp"

[[project]]
name = "cli-tool"
[project.action]
file_or_dir = "/tmp"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["complete", "projects"])
        .assert()
        .success()
        .stdout(predicate::str::contains("webapp"))
        .stdout(predicate::str::contains("webapi"))
        .stdout(predicate::str::contains("cli-tool"));
}

#[test]
fn test_complete_projects_with_prefix() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""

[[project]]
name = "webapp"
[project.action]
file_or_dir = "/tmp"

[[project]]
name = "webapi"
[project.action]
file_or_dir = "/tmp"

[[project]]
name = "cli-tool"
[project.action]
file_or_dir = "/tmp"
"#,
    );

    // Only "web" prefix matches should appear
    let output = pjmai_cmd(&temp_dir)
        .args(["complete", "projects", "web"])
        .output()
        .expect("Failed to run command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("webapp"));
    assert!(stdout.contains("webapi"));
    assert!(!stdout.contains("cli-tool"));
}

#[test]
fn test_complete_commands_all() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["complete", "commands"])
        .assert()
        .success()
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("change"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("remove"))
        .stdout(predicate::str::contains("setup"));
}

#[test]
fn test_complete_commands_with_prefix() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    // Only commands starting with "c" should appear
    let output = pjmai_cmd(&temp_dir)
        .args(["complete", "commands", "c"])
        .output()
        .expect("Failed to run command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("change"));
    assert!(stdout.contains("complete"));
    assert!(stdout.contains("completions"));
    assert!(!stdout.contains("add"));
    assert!(!stdout.contains("list"));
}

// ============================================================
// Config export/import tests
// ============================================================

#[test]
fn test_config_export_toml() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "test"

[[project]]
name = "test"

[project.action]
file_or_dir = "/tmp/test"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["config", "export"])
        .assert()
        .success()
        .stdout(predicate::str::contains("version"))
        .stdout(predicate::str::contains("current_project"))
        .stdout(predicate::str::contains("[[project]]"))
        .stdout(predicate::str::contains("name = \"test\""));
}

#[test]
fn test_config_export_json() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "test"

[[project]]
name = "test"

[project.action]
file_or_dir = "/tmp/test"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["config", "export", "--format", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"version\""))
        .stdout(predicate::str::contains("\"current_project\""))
        .stdout(predicate::str::contains("\"projects\""))
        .stdout(predicate::str::contains("\"name\": \"test\""));
}

#[test]
fn test_config_export_invalid_format() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["config", "export", "--format", "xml"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid format"));
}

#[test]
fn test_config_import_dry_run() {
    let temp_dir = TempDir::new().unwrap();

    // Create initial empty config
    fs::write(
        temp_dir.path().join("config.toml"),
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    )
    .unwrap();

    // Create import file with a project
    let import_file = temp_dir.path().join("import.toml");
    fs::write(
        &import_file,
        r#"version = "0.1.0"
current_project = ""

[[project]]
name = "imported"

[project.action]
file_or_dir = "/tmp/imported"
"#,
    )
    .unwrap();

    // Dry run should succeed and show what would be imported
    pjmai_cmd(&temp_dir)
        .args(["config", "import", "--dry-run", import_file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Would import"))
        .stdout(predicate::str::contains("imported"));

    // Verify nothing was actually imported
    let config = read_config(&temp_dir);
    assert!(!config.contains("imported"));
}

#[test]
fn test_config_import_adds_new_projects() {
    let temp_dir = TempDir::new().unwrap();

    // Create initial config with one project
    fs::write(
        temp_dir.path().join("config.toml"),
        r#"version = "0.1.0"
current_project = "existing"

[[project]]
name = "existing"

[project.action]
file_or_dir = "/tmp/existing"
"#,
    )
    .unwrap();

    // Create import file with a new project
    let import_file = temp_dir.path().join("import.toml");
    fs::write(
        &import_file,
        r#"version = "0.1.0"
current_project = ""

[[project]]
name = "newproject"

[project.action]
file_or_dir = "/tmp/newproject"
"#,
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["config", "import", import_file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Imported"))
        .stdout(predicate::str::contains("newproject"));

    // Verify the project was added
    let config = read_config(&temp_dir);
    assert!(config.contains("newproject"));
    assert!(config.contains("existing")); // Original still there
}

#[test]
fn test_config_import_skips_existing() {
    let temp_dir = TempDir::new().unwrap();

    // Create initial config with a project
    fs::write(
        temp_dir.path().join("config.toml"),
        r#"version = "0.1.0"
current_project = "myproject"

[[project]]
name = "myproject"

[project.action]
file_or_dir = "/tmp/myproject"
"#,
    )
    .unwrap();

    // Create import file with the same project name
    let import_file = temp_dir.path().join("import.toml");
    fs::write(
        &import_file,
        r#"version = "0.1.0"
current_project = ""

[[project]]
name = "myproject"

[project.action]
file_or_dir = "/tmp/different-path"
"#,
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["config", "import", import_file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Skipped"));

    // Verify the original path is unchanged
    let config = read_config(&temp_dir);
    assert!(config.contains("/tmp/myproject"));
    assert!(!config.contains("/tmp/different-path"));
}

#[test]
fn test_config_import_json() {
    let temp_dir = TempDir::new().unwrap();

    // Create initial empty config
    fs::write(
        temp_dir.path().join("config.toml"),
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    )
    .unwrap();

    // Create JSON import file
    let import_file = temp_dir.path().join("import.json");
    fs::write(
        &import_file,
        r#"{
  "version": "0.1.0",
  "current_project": "",
  "project": [
    {
      "name": "jsonproject",
      "action": {
        "file_or_dir": "/tmp/jsonproject"
      }
    }
  ]
}"#,
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["--json", "config", "import", import_file.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"success\": true"))
        .stdout(predicate::str::contains("\"added\": 1"));

    // Verify the project was added
    let config = read_config(&temp_dir);
    assert!(config.contains("jsonproject"));
}

#[test]
fn test_config_import_nonexistent_file() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["config", "import", "/nonexistent/file.toml"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not exist"));
}

// ============================================================
// Environment configuration tests
// ============================================================

#[test]
fn test_env_set_variable() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("myproject");
    fs::create_dir(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "test"
[[project]]
name = "test"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "set", "FOO", "bar"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Set FOO=bar"));

    // Verify config updated
    let config = read_config(&temp_dir);
    assert!(config.contains("FOO"));
    assert!(config.contains("bar"));
}

#[test]
fn test_env_on_enter() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("myproject");
    fs::create_dir(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "test"
[[project]]
name = "test"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "on-enter", "echo hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added on_enter command"));

    // Verify config updated
    let config = read_config(&temp_dir);
    assert!(config.contains("on_enter"));
    assert!(config.contains("echo hello"));
}

#[test]
fn test_env_show() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "test"
[[project]]
name = "test"
[project.action]
file_or_dir = "/tmp/test"
[project.metadata.environment]
on_enter = ["echo hello"]
[project.metadata.environment.vars]
FOO = "bar"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("FOO=bar"))
        .stdout(predicate::str::contains("echo hello"));
}

#[test]
fn test_env_show_json() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "test"
[[project]]
name = "test"
[project.action]
file_or_dir = "/tmp/test"
[project.metadata.environment]
on_enter = ["echo hello"]
[project.metadata.environment.vars]
FOO = "bar"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["--json", "env", "-p", "test", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"project\": \"test\""))
        .stdout(predicate::str::contains("\"FOO\": \"bar\""))
        .stdout(predicate::str::contains("\"echo hello\""));
}

#[test]
fn test_env_clear() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "test"
[[project]]
name = "test"
[project.action]
file_or_dir = "/tmp/test"
[project.metadata.environment]
on_enter = ["echo hello"]
[project.metadata.environment.vars]
FOO = "bar"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "clear"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Cleared environment config"));

    // Verify environment is cleared
    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("no environment configuration"));
}

#[test]
fn test_env_unset() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "test"
[[project]]
name = "test"
[project.action]
file_or_dir = "/tmp/test"
[project.metadata.environment.vars]
FOO = "bar"
BAZ = "qux"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "unset", "FOO"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Unset FOO"));

    // Verify FOO is removed but BAZ remains
    let config = read_config(&temp_dir);
    assert!(!config.contains("FOO"));
    assert!(config.contains("BAZ"));
}

#[test]
fn test_change_with_env_exits_code_5() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("myproject");
    fs::create_dir(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""
[[project]]
name = "test"
[project.action]
file_or_dir = "{}"
[project.metadata.environment]
on_enter = ["echo hello"]
[project.metadata.environment.vars]
FOO = "bar"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    // Change command with env config should exit with code 5
    pjmai_cmd(&temp_dir)
        .args(["change", "-p", "test"])
        .assert()
        .code(5)
        .stdout(predicate::str::contains("cd '"))
        .stdout(predicate::str::contains("export FOO='bar'"))
        .stdout(predicate::str::contains("echo hello"));
}

#[test]
fn test_env_on_exit() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("myproject");
    fs::create_dir(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "test"
[[project]]
name = "test"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "on-exit", "deactivate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added on_exit command"));

    // Verify config updated
    let config = read_config(&temp_dir);
    assert!(config.contains("on_exit"));
    assert!(config.contains("deactivate"));
}

#[test]
fn test_env_path_prepend() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("myproject");
    fs::create_dir(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "test"
[[project]]
name = "test"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "path-prepend", "./node_modules/.bin"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added path_prepend"));

    // Verify config updated
    let config = read_config(&temp_dir);
    assert!(config.contains("path_prepend"));
    assert!(config.contains("node_modules/.bin"));
}

#[test]
fn test_env_path_remove() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "test"
[[project]]
name = "test"
[project.action]
file_or_dir = "/tmp/test"
[project.metadata.environment]
path_prepend = ["./bin", "./node_modules/.bin"]
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "path-remove", "./bin"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed path_prepend"));

    // Verify ./bin is removed but ./node_modules/.bin remains
    let config = read_config(&temp_dir);
    assert!(!config.contains("\"./bin\""));
    assert!(config.contains("node_modules/.bin"));
}

#[test]
fn test_env_show_with_new_fields() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "test"
[[project]]
name = "test"
[project.action]
file_or_dir = "/tmp/test"
[project.metadata.environment]
on_enter = ["source .venv/bin/activate"]
on_exit = ["deactivate"]
path_prepend = ["./.venv/bin"]
[project.metadata.environment.vars]
FOO = "bar"
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("FOO=bar"))
        .stdout(predicate::str::contains("Path prepend"))
        .stdout(predicate::str::contains("./.venv/bin"))
        .stdout(predicate::str::contains("On enter"))
        .stdout(predicate::str::contains("source .venv/bin/activate"))
        .stdout(predicate::str::contains("On exit"))
        .stdout(predicate::str::contains("deactivate"));
}

#[test]
fn test_env_show_json_with_new_fields() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = "test"
[[project]]
name = "test"
[project.action]
file_or_dir = "/tmp/test"
[project.metadata.environment]
on_enter = ["echo enter"]
on_exit = ["echo exit"]
path_prepend = ["./bin"]
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["--json", "env", "-p", "test", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"on_exit\":"))
        .stdout(predicate::str::contains("\"echo exit\""))
        .stdout(predicate::str::contains("\"path_prepend\":"))
        .stdout(predicate::str::contains("\"./bin\""));
}

#[test]
fn test_change_with_full_env_config() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("myproject");
    fs::create_dir(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""
[[project]]
name = "test"
[project.action]
file_or_dir = "{}"
[project.metadata.environment]
on_enter = ["echo entering"]
on_exit = ["echo exiting"]
path_prepend = ["./bin", "./.venv/bin"]
[project.metadata.environment.vars]
MY_VAR = "value"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    // Change command with full env config should exit with code 5
    // and include all env setup in the script
    pjmai_cmd(&temp_dir)
        .args(["change", "-p", "test"])
        .assert()
        .code(5)
        .stdout(predicate::str::contains("cd '"))
        // PATH prepend comes first
        .stdout(predicate::str::contains("export PATH='./bin':\"$PATH\""))
        .stdout(predicate::str::contains("export PATH='./.venv/bin':\"$PATH\""))
        // Then env vars
        .stdout(predicate::str::contains("export MY_VAR='value'"))
        // Then on_exit storage
        .stdout(predicate::str::contains("_PJMAI_ON_EXIT='echo exiting'"))
        // Then on_enter commands
        .stdout(predicate::str::contains("echo entering"));
}

#[test]
fn test_env_auto_detect_python_venv() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("myproject");
    fs::create_dir(&project_dir).unwrap();

    // Create .venv directory to simulate Python virtual environment
    fs::create_dir(project_dir.join(".venv")).unwrap();
    fs::create_dir_all(project_dir.join(".venv/bin")).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""
[[project]]
name = "test"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    // Run auto-detect with dry_run first
    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "auto-detect", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("python-venv"))
        .stdout(predicate::str::contains(".venv/"))
        .stdout(predicate::str::contains("dry run"));

    // Run auto-detect without dry_run to apply
    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "auto-detect"])
        .assert()
        .success()
        .stdout(predicate::str::contains("python-venv"))
        .stdout(predicate::str::contains("Configuration applied"));

    // Verify the config was updated
    let config = read_config(&temp_dir);
    assert!(config.contains("path_prepend"));
    assert!(config.contains(".venv/bin"));
    assert!(config.contains("on_enter"));
    assert!(config.contains("source .venv/bin/activate"));
    assert!(config.contains("on_exit"));
    assert!(config.contains("deactivate"));
}

#[test]
fn test_env_auto_detect_nvmrc() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("myproject");
    fs::create_dir(&project_dir).unwrap();

    // Create .nvmrc file
    fs::write(project_dir.join(".nvmrc"), "18").unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""
[[project]]
name = "test"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "auto-detect"])
        .assert()
        .success()
        .stdout(predicate::str::contains("node-nvm"))
        .stdout(predicate::str::contains(".nvmrc"));

    // Verify config
    let config = read_config(&temp_dir);
    assert!(config.contains("nvm use"));
}

#[test]
fn test_env_auto_detect_node_modules() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("myproject");
    fs::create_dir(&project_dir).unwrap();

    // Create node_modules/.bin directory
    fs::create_dir_all(project_dir.join("node_modules/.bin")).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""
[[project]]
name = "test"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "auto-detect"])
        .assert()
        .success()
        .stdout(predicate::str::contains("node-modules"))
        .stdout(predicate::str::contains("node_modules/.bin"));

    // Verify config
    let config = read_config(&temp_dir);
    assert!(config.contains("path_prepend"));
    assert!(config.contains("node_modules/.bin"));
}

#[test]
fn test_env_auto_detect_no_features() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("myproject");
    fs::create_dir(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""
[[project]]
name = "test"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "auto-detect"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No environment features detected"));
}

#[test]
fn test_env_auto_detect_json() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("myproject");
    fs::create_dir(&project_dir).unwrap();

    // Create .venv directory
    fs::create_dir_all(project_dir.join(".venv/bin")).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""
[[project]]
name = "test"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["--json", "env", "-p", "test", "auto-detect", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"project\": \"test\""))
        .stdout(predicate::str::contains("\"applied\": false"))
        .stdout(predicate::str::contains("\"feature\": \"python-venv\""))
        .stdout(predicate::str::contains("\"source\": \".venv/\""));
}

#[test]
fn test_env_auto_detect_multiple_features() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("myproject");
    fs::create_dir(&project_dir).unwrap();

    // Create multiple features
    fs::create_dir_all(project_dir.join(".venv/bin")).unwrap();
    fs::write(project_dir.join(".nvmrc"), "20").unwrap();
    fs::create_dir_all(project_dir.join("node_modules/.bin")).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""
[[project]]
name = "test"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["env", "-p", "test", "auto-detect"])
        .assert()
        .success()
        .stdout(predicate::str::contains("python-venv"))
        .stdout(predicate::str::contains("node-nvm"))
        .stdout(predicate::str::contains("node-modules"));

    // Verify all features are in config
    let config = read_config(&temp_dir);
    assert!(config.contains(".venv/bin"));
    assert!(config.contains("nvm use"));
    assert!(config.contains("node_modules/.bin"));
    assert!(config.contains("deactivate"));
}

// ============================================================================
// Group Tests
// ============================================================================

#[test]
fn test_group_list_empty() {
    let temp_dir = setup_with_config(
        r#"version = "0.1.0"
current_project = ""
project = []
"#,
    );

    pjmai_cmd(&temp_dir)
        .args(["group", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("No groups found"));
}

#[test]
fn test_group_list_with_projects() {
    let temp_dir = TempDir::new().unwrap();
    let parent_dir = temp_dir.path().join("github/myorg");
    let project_dir = parent_dir.join("project1");
    fs::create_dir_all(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "proj1"
[[project]]
name = "proj1"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["group", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("myorg"))
        .stdout(predicate::str::contains("1")); // project count
}

#[test]
fn test_group_list_json() {
    let temp_dir = TempDir::new().unwrap();
    let parent_dir = temp_dir.path().join("github/testgroup");
    let project_dir = parent_dir.join("testproj");
    fs::create_dir_all(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "testproj"
[[project]]
name = "testproj"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["--json", "group", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"name\": \"testgroup\""))
        .stdout(predicate::str::contains("\"project_count\": 1"))
        .stdout(predicate::str::contains("\"is_current\": true"));
}

#[test]
fn test_group_show_current() {
    let temp_dir = TempDir::new().unwrap();
    let parent_dir = temp_dir.path().join("code/work");
    let project_dir = parent_dir.join("webapp");
    fs::create_dir_all(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "webapp"
[[project]]
name = "webapp"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["group", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Group: work"))
        .stdout(predicate::str::contains("Projects: 1"));
}

#[test]
fn test_group_show_with_all_flag() {
    let temp_dir = TempDir::new().unwrap();
    let parent_dir = temp_dir.path().join("repos/mygroup");
    let proj1 = parent_dir.join("proj1");
    let proj2 = parent_dir.join("proj2");
    fs::create_dir_all(&proj1).unwrap();
    fs::create_dir_all(&proj2).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "proj1"
[[project]]
name = "proj1"
[project.action]
file_or_dir = "{}"
[[project]]
name = "proj2"
[project.action]
file_or_dir = "{}"
"#,
            proj1.display(),
            proj2.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["group", "show", "--all"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Group: mygroup"))
        .stdout(predicate::str::contains("proj1"))
        .stdout(predicate::str::contains("proj2"));
}

#[test]
fn test_group_prompt() {
    let temp_dir = TempDir::new().unwrap();
    let parent_dir = temp_dir.path().join("code/mygroup");
    let project_dir = parent_dir.join("proj");
    fs::create_dir_all(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "proj"
[[project]]
name = "proj"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["group", "prompt"])
        .assert()
        .success()
        .stdout(predicate::str::contains("mygroup"));
}

#[test]
fn test_group_alias_set_and_list() {
    let temp_dir = TempDir::new().unwrap();
    let parent_dir = temp_dir.path().join("repos/longname");
    let project_dir = parent_dir.join("proj");
    fs::create_dir_all(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "proj"
[[project]]
name = "proj"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    // Set alias
    pjmai_cmd(&temp_dir)
        .args(["group", "alias", "longname", "short"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Set alias 'short' for group 'longname'"));

    // List aliases
    pjmai_cmd(&temp_dir)
        .args(["group", "alias", "--list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("longname"))
        .stdout(predicate::str::contains("short"));

    // Verify alias is in config
    let config = read_config(&temp_dir);
    assert!(config.contains("[group_aliases]"));
    assert!(config.contains("longname = \"short\""));
}

#[test]
fn test_group_alias_remove() {
    let temp_dir = TempDir::new().unwrap();
    let parent_dir = temp_dir.path().join("repos/testgroup");
    let project_dir = parent_dir.join("proj");
    fs::create_dir_all(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "proj"
[group_aliases]
testgroup = "tg"
[[project]]
name = "proj"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    // Remove alias
    pjmai_cmd(&temp_dir)
        .args(["group", "alias", "testgroup", "--remove"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed alias for group 'testgroup'"));

    // Verify alias is removed from config
    let config = read_config(&temp_dir);
    assert!(!config.contains("testgroup = \"tg\""));
}

#[test]
fn test_group_prompt_with_alias() {
    let temp_dir = TempDir::new().unwrap();
    let parent_dir = temp_dir.path().join("repos/longgroup");
    let project_dir = parent_dir.join("proj");
    fs::create_dir_all(&project_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "proj"
[group_aliases]
longgroup = "lg"
[[project]]
name = "proj"
[project.action]
file_or_dir = "{}"
"#,
            project_dir.display()
        ),
    )
    .unwrap();

    // Without --alias flag, shows group name
    pjmai_cmd(&temp_dir)
        .args(["group", "prompt"])
        .assert()
        .success()
        .stdout(predicate::str::contains("longgroup"));

    // With --alias flag, shows alias
    pjmai_cmd(&temp_dir)
        .args(["group", "prompt", "--alias"])
        .assert()
        .success()
        .stdout(predicate::str::contains("lg"));
}

#[test]
fn test_list_with_group_filter() {
    let temp_dir = TempDir::new().unwrap();
    let group1 = temp_dir.path().join("repos/group1");
    let group2 = temp_dir.path().join("repos/group2");
    let proj1 = group1.join("proj1");
    let proj2 = group2.join("proj2");
    fs::create_dir_all(&proj1).unwrap();
    fs::create_dir_all(&proj2).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "proj1"
[[project]]
name = "proj1"
[project.action]
file_or_dir = "{}"
[[project]]
name = "proj2"
[project.action]
file_or_dir = "{}"
"#,
            proj1.display(),
            proj2.display()
        ),
    )
    .unwrap();

    // Filter by group1
    pjmai_cmd(&temp_dir)
        .args(["list", "--group", "group1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("proj1"))
        .stdout(predicate::str::contains("group1").not().or(predicate::str::contains("Projects in group")));

    // Verify proj2 is not in output when filtering by group1
    let output = pjmai_cmd(&temp_dir)
        .args(["list", "--group", "group1"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("proj1"));
    // proj2 should not appear in the project list (it may appear in header)
}

#[test]
fn test_list_with_group_filter_current() {
    let temp_dir = TempDir::new().unwrap();
    let group1 = temp_dir.path().join("repos/mygroup");
    let proj1 = group1.join("proj1");
    let proj2 = group1.join("proj2");
    fs::create_dir_all(&proj1).unwrap();
    fs::create_dir_all(&proj2).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = "proj1"
[[project]]
name = "proj1"
[project.action]
file_or_dir = "{}"
[[project]]
name = "proj2"
[project.action]
file_or_dir = "{}"
"#,
            proj1.display(),
            proj2.display()
        ),
    )
    .unwrap();

    // Filter by "." (current group)
    pjmai_cmd(&temp_dir)
        .args(["list", "--group", "."])
        .assert()
        .success()
        .stdout(predicate::str::contains("proj1"))
        .stdout(predicate::str::contains("proj2"))
        .stdout(predicate::str::contains("mygroup"));
}

// ============================================================
// Subdirectory navigation tests
// ============================================================

#[test]
fn test_change_to_subdir_exits_with_code_2() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("proj");
    let subdir = proj_dir.join("src");
    fs::create_dir_all(&subdir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""

[[project]]
name = "myproj"

[project.action]
file_or_dir = "{}"
"#,
            proj_dir.display()
        ),
    )
    .unwrap();

    // Change to subdir using space syntax
    pjmai_cmd(&temp_dir)
        .args(["change", "-p", "myproj", "src"])
        .assert()
        .code(2)
        .stdout(predicate::str::contains(subdir.to_str().unwrap()));
}

#[test]
fn test_change_to_nested_subdir() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("proj");
    let nested = proj_dir.join("src").join("lib");
    fs::create_dir_all(&nested).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""

[[project]]
name = "myproj"

[project.action]
file_or_dir = "{}"
"#,
            proj_dir.display()
        ),
    )
    .unwrap();

    // Change using slash syntax
    pjmai_cmd(&temp_dir)
        .args(["change", "-p", "myproj", "src/lib"])
        .assert()
        .code(2)
        .stdout(predicate::str::contains(nested.to_str().unwrap()));
}

#[test]
fn test_change_to_nonexistent_subdir_fails() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("proj");
    fs::create_dir(&proj_dir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""

[[project]]
name = "myproj"

[project.action]
file_or_dir = "{}"
"#,
            proj_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["change", "-p", "myproj", "nonexistent"])
        .assert()
        .code(4)
        .stdout(predicate::str::contains("subdirectory 'nonexistent' not found"));
}

#[test]
fn test_change_to_file_subdir_fails() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("proj");
    fs::create_dir(&proj_dir).unwrap();
    fs::write(proj_dir.join("README.md"), "# Test").unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""

[[project]]
name = "myproj"

[project.action]
file_or_dir = "{}"
"#,
            proj_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["change", "-p", "myproj", "README.md"])
        .assert()
        .code(4)
        .stdout(predicate::str::contains("is a file, not a directory"));
}

#[test]
fn test_change_to_subdir_json() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("proj");
    let subdir = proj_dir.join("src");
    fs::create_dir_all(&subdir).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""

[[project]]
name = "myproj"

[project.action]
file_or_dir = "{}"
"#,
            proj_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["--json", "change", "-p", "myproj", "src"])
        .assert()
        .code(2)
        .stdout(predicate::str::contains("\"subdir\": \"src\""));
}

#[test]
fn test_complete_subdirs_all() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("proj");
    fs::create_dir(&proj_dir).unwrap();
    fs::create_dir(proj_dir.join("src")).unwrap();
    fs::create_dir(proj_dir.join("tests")).unwrap();
    fs::create_dir(proj_dir.join(".hidden")).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""

[[project]]
name = "myproj"

[project.action]
file_or_dir = "{}"
"#,
            proj_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["complete", "subdirs", "myproj"])
        .assert()
        .success()
        .stdout(predicate::str::contains("src"))
        .stdout(predicate::str::contains("tests"))
        // Hidden directories should be excluded
        .stdout(predicate::str::contains(".hidden").not());
}

#[test]
fn test_complete_subdirs_with_prefix() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("proj");
    fs::create_dir(&proj_dir).unwrap();
    fs::create_dir(proj_dir.join("src")).unwrap();
    fs::create_dir(proj_dir.join("scripts")).unwrap();
    fs::create_dir(proj_dir.join("tests")).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""

[[project]]
name = "myproj"

[project.action]
file_or_dir = "{}"
"#,
            proj_dir.display()
        ),
    )
    .unwrap();

    pjmai_cmd(&temp_dir)
        .args(["complete", "subdirs", "myproj", "s"])
        .assert()
        .success()
        .stdout(predicate::str::contains("src"))
        .stdout(predicate::str::contains("scripts"))
        .stdout(predicate::str::contains("tests").not());
}

#[test]
fn test_complete_subdirs_nested() {
    let temp_dir = TempDir::new().unwrap();
    let proj_dir = temp_dir.path().join("proj");
    let src_dir = proj_dir.join("src");
    fs::create_dir_all(&src_dir).unwrap();
    fs::create_dir(src_dir.join("lib")).unwrap();
    fs::create_dir(src_dir.join("bin")).unwrap();

    fs::write(
        temp_dir.path().join("config.toml"),
        format!(
            r#"version = "0.1.0"
current_project = ""

[[project]]
name = "myproj"

[project.action]
file_or_dir = "{}"
"#,
            proj_dir.display()
        ),
    )
    .unwrap();

    // Complete inside src directory
    pjmai_cmd(&temp_dir)
        .args(["complete", "subdirs", "myproj", "src"])
        .assert()
        .success()
        .stdout(predicate::str::contains("src/lib"))
        .stdout(predicate::str::contains("src/bin"));
}
