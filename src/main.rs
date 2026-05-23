use clap::Parser;
use hex::encode;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
struct Cli {
    path: String,
    #[arg(long)]
    dry_run: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut file_collection: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for entry in WalkDir::new(&cli.path) {
        let entry = entry.unwrap();

        if entry.file_type().is_file() {
            let file_path = entry.path().to_path_buf();
            let hash = hash_file(&file_path).unwrap();
            file_collection
                .entry(hash)
                .or_insert_with(Vec::new)
                .push(file_path);
        }
    }

    for (hash, files) in &file_collection {
        if files.len() > 1 {
            println!("\nHash: {}", hash);
            for (index, file) in files.iter().enumerate() {
                println!("  [{}] {}", index + 1, file.display());
            }

            if cli.dry_run {
                println!("  (dry-run: no files will be deleted)");
                continue;
            }

            print!("  Keep which file? (enter number, or 's' to skip): ");
            io::stdout().flush().unwrap();

            let stdin = io::stdin();
            let mut input = String::new();
            stdin.lock().read_line(&mut input).unwrap();
            let input = input.trim();

            if input == "s" {
                println!("  Skipped.");
                continue;
            }

            let keep_index: usize = match input.parse::<usize>() {
                Ok(n) if n >= 1 && n <= files.len() => n - 1,
                _ => {
                    println!("  Invalid input, skipping.");
                    continue;
                }
            };

            for (index, file) in files.iter().enumerate() {
                if index != keep_index {
                    match fs::remove_file(file) {
                        Ok(_) => println!("  Deleted: {}", file.display()),
                        Err(e) => println!("  Failed to delete {}: {}", file.display(), e),
                    }
                }
            }
        }
    }
}

fn hash_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let hash = Sha256::digest(&buffer);
    Ok(encode(hash))
}