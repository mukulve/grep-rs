use clap::Parser;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The Search Term You Want To Search For
    #[arg(short, long)]
    search: String,

    /// The Directory You Want To Search In
    #[arg(short, long)]
    directory: String,
}

fn main() {
    let start = Instant::now();

    let args = Args::parse();
    let directory = &args.directory;
    let target = &args.search;

    let file_paths: Vec<_> = WalkDir::new(directory)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path().to_owned())
        .collect();

    file_paths.par_iter().for_each(|path| {
        find_matches(path, &target);
    });

    let duration = start.elapsed();
    println!("Took {:?} To Search {:?}", duration, args.directory);
}

fn find_matches(path: &Path, target: &str) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line_result in reader
        .lines()
        .filter_map(Result::ok)
        .filter(|entry| entry.contains(target))
    {
        println!("{} : {:?}", path.display(), line_result.trim());
    }
}
