use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use clap::Parser;

/// A Rust version of dd that allows threaded copy
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short)]
    input: String,

    /// Output file
    #[arg(short)]
    output: String,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let input_file = File::open(args.input).expect("Failed to open input file");
    let mut buf_reader = BufReader::new(input_file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut output_file = File::create(args.output).expect("Failed to open output file");
    output_file.write_all(contents.as_bytes())?;
    output_file.sync_all()?;
    Ok(())
}
