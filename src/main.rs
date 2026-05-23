use clap::Parser;
use hex::encode;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
struct Cli {
    path: String,
    #[arg(long)]
    dry_run: bool,
}

fn main() {
    let mut file_collection: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let cli = Cli::parse();

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
            println!("Hash: {}", hash);
            for file in files {
                println!("  {}", file.display());
            }
            println!();
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
