use clap::Parser;
use regex::Regex;
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

    // Build regex pattern
    let pattern = if args.ignore_case {
        format!("(?i){}", args.pattern)
    } else {
        args.pattern.clone()
    };

    let re = match Regex::new(&pattern) {
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
        search_file(&re, path, &args, None);
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
        let path = entry.path();
        search_file(re, path, args, Some(&args.path));
    }
}

fn search_file(re: &Regex, path: &Path, args: &Args, base_path: Option<&str>) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: {}", path.display(), e);
            return;
        }
    };

    let reader = BufReader::new(file);
    let path_display = path.display().to_string();
    let relative_path = if let Some(base) = base_path {
        path.strip_prefix(base)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| path_display.clone())
    } else {
        path_display.clone()
    };

    for (line_num, line_result) in reader.lines().enumerate() {
        let line_num = line_num + 1; // 1-indexed

        let line = match line_result {
            Ok(l) => l,
            Err(_) => continue,
        };

        if re.is_match(&line) {
            if args.recursive {
                // Show filename prefix for recursive search
                if args.line_number {
                    println!("{}:{}:{}", relative_path, line_num, line);
                } else {
                    println!("{}:{}", relative_path, line);
                }
            } else {
                if args.line_number {
                    println!("{}:{}", line_num, line);
                } else {
                    println!("{}", line);
                }
            }
        }
    }
}