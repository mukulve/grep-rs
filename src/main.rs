use clap::Parser;
use rayon::prelude::*;
use regex::Regex;
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

    /// Optional Flag To Indicate If You Entered A Regex String
    #[arg(short, long)]
    regex: bool,
}

fn main() {
    let start = Instant::now();
    let args = Args::parse();
    let directory = &args.directory;
    let target = args.search;
    let regex_used = args.regex;
    let compiled_regex: Option<Regex>;

    if regex_used {
        compiled_regex = Some(Regex::new(&target).expect("Invalid Regex String Entered!"));
    } else {
        compiled_regex = None;
    }

    let file_paths: Vec<_> = WalkDir::new(directory)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path().to_owned())
        .collect();

    if let Some(regex) = compiled_regex {
        file_paths.par_iter().for_each(|path| {
            find_matches_regex(path, &regex);
        });
    } else {
        file_paths.par_iter().for_each(|path| {
            find_matches_no_regex(path, target.clone());
        });
    }

    let duration = start.elapsed();
    println!("Took {:?} To Search {:?}", duration, args.directory);
}


fn find_matches_regex(path: &Path, target: &Regex) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line_result in reader
        .lines()
        .filter_map(Result::ok)
        .filter(|entry| target.is_match(&entry))
    {
        println!("{} : {:?}", path.display(), line_result.trim());
    }
}


fn find_matches_no_regex(path: &Path, target: String) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line_result in reader
        .lines()
        .filter_map(Result::ok)
        .filter(|entry| entry.contains(&target))
    {
        println!("{} : {:?}", path.display(), line_result);
    }
}
