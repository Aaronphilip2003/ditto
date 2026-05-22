use walkdir::WalkDir;
fn main() {
    let path = "/Users/aaron/Downloads";

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        println!("{}",entry.path().display());
    }
}
