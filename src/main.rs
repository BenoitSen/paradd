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

    /// Number of bytes to copy
    #[arg(long, default_value_t = 512)]
    count: usize,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let input_file = File::open(args.input).expect("Failed to open input file");
    let mut buf_reader = BufReader::with_capacity(args.count, input_file);
    let mut contents = vec![0; args.count];
    let byte_read = buf_reader.read(&mut contents)?;

    let mut output_file = File::create(args.output).expect("Failed to open output file");
    output_file.write_all(&contents[..byte_read])?;
    output_file.sync_all()?;
    Ok(())
}
