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
    let mut cmd: Command = cargo_bin_cmd!("pjmai");
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

    let mut cmd: Command = cargo_bin_cmd!("pjmai");
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
