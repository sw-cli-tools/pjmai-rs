use chrono::Utc;
use std::process::Command;

fn main() {
    // Get git commit SHA (short)
    let commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Get build host
    let host = hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string());

    // Get ISO timestamp
    let build_time = Utc::now().to_rfc3339();

    // Pass to compiler as environment variables
    println!("cargo:rustc-env=BUILD_COMMIT={}", commit);
    println!("cargo:rustc-env=BUILD_HOST={}", host);
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);

    // Tell Cargo to rerun if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
}
