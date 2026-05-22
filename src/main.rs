use std::env;
use walkdir::WalkDir;
use sha2::{Digest, Sha256};
use std::path::Path;
use hex::encode;
use std::fs::File;
use std::io::{self, Read};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ditto <path>");
        return;
    }

    let path = &args[1];

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();

        if entry.file_type().is_file() {
            let file_path = entry.path();
            let hash = hash_file(file_path).unwrap();
            println!("{}: {}", file_path.display(), hash);
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
