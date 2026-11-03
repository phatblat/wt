//! `wt --version` tests: the flag prints the version and exits zero without
//! touching the repository.

mod common;

use common::*;
use std::path::Path;

fn stdout_of(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}

#[test]
fn version_flag_prints_the_version_and_exits_zero() {
    let out = wt_cmd(Path::new("/nonexistent"))
        .arg("--version")
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(
        stdout_of(&out.stdout),
        format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn short_version_flag_prints_the_same_output() {
    let out = wt_cmd(Path::new("/nonexistent"))
        .arg("-V")
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(
        stdout_of(&out.stdout),
        format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
    );
}
