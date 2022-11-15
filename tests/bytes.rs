use anyhow::Error;
use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::process::Command; // Run programs

#[test]
fn copy_txt_to_txt() -> Result<(), Error> {
    let mut cmd = Command::cargo_bin("paradd")?;
    cmd.arg("-i ressources/ascii_text.txt")
        .arg("-o tmp/ascii_text.txt");
    cmd.assert().success();

    let contents_origin =
        fs::read_to_string("tmp/ascii_text.txt").expect("Output file was not created.");
    let contents_copied =
        fs::read_to_string("ressources/ascii_text.txt").expect("Input file is missing.");

    assert_eq!(contents_origin, contents_copied);

    Ok(())
}
