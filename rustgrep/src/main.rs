use clap::Parser;
use regex::{Regex, RegexBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

/// A grep-like tool written in Rust
#[derive(Parser, Debug)]
#[command(name = "rustgrep")]
#[command(about = "Search for patterns in files", long_about = None)]
struct Args {
    /// Pattern to search for (regular expression)
    #[arg(required = true)]
    pattern: String,

    /// File or directory to search
    #[arg(required = true)]
    path: String,

    /// Case insensitive search
    #[arg(short = 'i', long)]
    ignore_case: bool,

    /// Show line numbers
    #[arg(short = 'n', long)]
    line_number: bool,

    /// Recursive directory search
    #[arg(short = 'r', long)]
    recursive: bool,
}

fn main() {
    let args = Args::parse();

    let re = match RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()
    {
        Ok(re) => re,
        Err(e) => {
            eprintln!("Invalid regex pattern: {}", e);
            std::process::exit(1);
        }
    };

    let path = Path::new(&args.path);

    if path.is_dir() {
        if args.recursive {
            search_directory_recursive(&re, path, &args);
        } else {
            eprintln!("{}: Is a directory (use -r for recursive search)", args.path);
            std::process::exit(1);
        }
    } else if path.is_file() {
        search_file(&re, path, &args);
    } else {
        eprintln!("{}: No such file or directory", args.path);
        std::process::exit(1);
    }
}

fn search_directory_recursive(re: &Regex, dir: &Path, args: &Args) {
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        search_file(re, entry.path(), args);
    }
}

fn search_file(re: &Regex, path: &Path, args: &Args) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: {}", path.display(), e);
            return;
        }
    };

    let reader = BufReader::new(file);

    for (line_num, line_result) in reader.lines().enumerate() {
        let line_num = line_num + 1;
        let line = match line_result {
            Ok(l) => l,
            Err(_) => continue,
        };

        if re.is_match(&line) {
            let prefix = match (args.recursive, args.line_number) {
                (true, true) => format!("{}:{}:", path.display(), line_num),
                (true, false) => format!("{}:", path.display()),
                (false, true) => format!("{}:", line_num),
                (false, false) => String::new(),
            };
            println!("{}{}", prefix, line);
        }
    }
}