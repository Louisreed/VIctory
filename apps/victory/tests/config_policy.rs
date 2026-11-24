//! `victory config` must fail when a configured path breaks the asset policy.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn victory_config(cache: &Path, data: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_victory"))
        .arg("config")
        // Run outside the repo so no ./victory.toml is picked up.
        .current_dir(std::env::temp_dir())
        .env_remove("VICTORY_CONFIG")
        .env_remove("VICTORY_GAME_DIR")
        .env_remove("VICTORY_TEST_TITLES")
        .env("VICTORY_CACHE_DIR", cache)
        .env("VICTORY_DATA_DIR", data)
        .output()
        .expect("running victory config")
}

fn outside(name: &str) -> PathBuf {
    std::env::temp_dir()
        .join(format!("victory-config-policy-{}", std::process::id()))
        .join(name)
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

#[test]
fn paths_outside_the_repo_succeed() {
    let out = victory_config(&outside("cache"), &outside("data"));
    assert!(out.status.success(), "{out:?}");
}

#[test]
fn a_cache_inside_the_repo_fails() {
    let out = victory_config(&repo_root().join("cache"), &outside("data"));
    assert!(!out.status.success(), "{out:?}");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("POLICY VIOLATION"), "{stderr}");
    assert!(stderr.contains("1 configured path(s)"), "{stderr}");
}

#[test]
fn every_violation_is_counted() {
    let root = repo_root();
    let out = victory_config(&root.join("cache"), &root.join("data"));
    assert!(!out.status.success(), "{out:?}");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(stderr.matches("POLICY VIOLATION").count(), 2, "{stderr}");
    assert!(stderr.contains("2 configured path(s)"), "{stderr}");
}
