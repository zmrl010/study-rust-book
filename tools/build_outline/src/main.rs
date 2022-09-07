use clap::Parser;
use std::fs;

/// Simple program to generate book outline
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// path to root of the book repo
    #[clap(short, long, value_parser)]
    source: String,

    /// path to output
    #[clap(short, long, value_parser)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let _contents = fs::read_to_string(args.source);
}
