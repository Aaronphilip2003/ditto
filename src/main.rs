use std::env;
use walkdir::WalkDir;

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
            println!("{}", entry.path().display());
        }
    }
}
