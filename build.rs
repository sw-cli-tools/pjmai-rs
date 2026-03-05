use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");

    // Get version from Cargo.toml
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.4.1".to_string());

    // Get git commit SHA (short)
    let commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Get build host
    let host = Command::new("hostname")
        .arg("-s")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Get ISO timestamp
    let build_time = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    let full_version = format!(
        "{version}\n\nCopyright (c) 2025 Michael A Wright\nLicense: MIT\nRepository: https://github.com/sw-cli-tools/pjmai-rs\n\nBuild Information:\n  Commit: {commit}\n  Built: {build_time}\n  Host: {host}",
        version = version,
        commit = commit,
        host = host,
        build_time = build_time,
    );

    let generated = format!(
        r#"/// Package version
pub const VERSION: &str = "{version}";

/// Git commit SHA (short)
pub const COMMIT: &str = "{commit}";

/// Build host
pub const BUILD_HOST: &str = "{host}";

/// Build timestamp (ISO 8601)
pub const BUILD_TIME: &str = "{build_time}";

/// Full version string for clap
pub fn generated_version() -> &'static str {{
    "{full_version}"
}}
"#,
        version = version,
        commit = commit,
        host = host,
        build_time = build_time,
        full_version = full_version,
    );

    fs::write(&dest_path, generated).unwrap();

    // Tell Cargo to rerun if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
}
