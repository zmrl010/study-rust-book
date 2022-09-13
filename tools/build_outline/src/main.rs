use build_outline::md::select_headings;
use clap::Parser;
use std::{fs, io, path::PathBuf};

/// Simple program to generate book outline
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// path to root of the book repo
    #[clap(short, long, value_parser)]
    path: PathBuf,

    /// path to output
    #[clap(short, long, value_parser)]
    output: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    for entry in fs::read_dir(args.path)? {
        let entry = entry?;

        let contents = fs::read_to_string(entry.path()).expect("should read file contents");

        let headings = select_headings(contents.as_str());

        fs::write(args.output.as_path(), headings).expect("should write contents to file");
    }

    Ok(())
}
