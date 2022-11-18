use anyhow::Error;
use assert_cmd::prelude::*;
use std::io::Write;
// Add methods on commands
use std::fs;
use std::process::Command; // Run programs

#[test]
fn copy_txt_to_txt() -> Result<(), Error> {
    let in_filename = "in_ascii_text.txt";
    let out_filename = "out_ascii_text.txt";

    let mut input_file = fs::File::create(in_filename).expect("Failed to create file");
    input_file.write_all(b"0123456789abcdefghijklm").unwrap();

    let mut cmd = Command::cargo_bin("paradd")?;
    cmd.args(["-i", in_filename]).args(["-o", out_filename]);
    let output = cmd.output().expect("Failed to execute the command");

    println!("{:?}", output);

    let contents_origin = fs::read_to_string(in_filename).expect("Input file is missing.");
    let contents_copied = fs::read_to_string(out_filename).expect("Output file was not created.");

    assert_eq!(contents_origin, contents_copied);

    fs::remove_file(in_filename)?;
    fs::remove_file(out_filename)?;
    Ok(())
}
