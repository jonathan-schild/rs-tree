/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

use std::{env, process::Command};

fn main() {
    let hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap()
        .stdout;
    println!(
        "cargo:rustc-env=BUILD_HASH={}",
        String::from_utf8(hash).unwrap()
    );

    let epoch = Command::new("date")
        .args(["-u", "+%s"])
        .output()
        .unwrap()
        .stdout;
    println!(
        "cargo:rustc-env=BUILD_EPOCH={}",
        String::from_utf8(epoch).unwrap()
    );

    let date = Command::new("date")
        .args(["-u", "-Iseconds"])
        .output()
        .unwrap()
        .stdout;
    println!(
        "cargo:rustc-env=BUILD_DATE={}",
        String::from_utf8(date).unwrap()
    );

    let rustc = Command::new(env::var("RUSTC").unwrap())
        .args(["--version"])
        .output()
        .unwrap()
        .stdout;
    println!(
        "cargo:rustc-env=BUILD_RUSTC={}",
        String::from_utf8(rustc).unwrap()
    )
}
