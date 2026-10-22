// Copyright 2026 Ivan Khanas
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_codex-multi"))
        .args(args)
        .output()
        .expect("failed to launch codex-multi")
}

#[test]
fn no_arguments_shows_help() {
    let output = run(&[]);
    assert!(output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stdout).contains("Usage:"));
}

#[test]
fn help_exits_successfully() {
    let output = run(&["--help"]);
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("codex-multi"));
    assert!(!stdout.contains("--version"));
}

#[test]
fn version_flag_is_unavailable() {
    let output = run(&["--version"]);
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(output.stdout.is_empty());
}

#[test]
fn unknown_arguments_are_rejected() {
    let output = run(&["--not-a-real-option"]);
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(!output.stderr.is_empty());
}
