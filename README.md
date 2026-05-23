<img width="1024" height="559" alt="image" src="https://github.com/user-attachments/assets/f28d3175-17c6-43c2-b4ba-51495f4246ea" />

# ditto

A CLI tool that finds duplicate files by content, not by name.

`ditto` scans a directory recursively, computes a SHA-256 hash of every file, and groups files that are byte-for-byte identical — regardless of what they are called or where they are located.

```
Hash: 740433b503f92795ac84a4f719428f20fc7fa0db5f2003647655c8ecebc5948b
  [1] /Downloads/Aaron Doc New (1).pdf
  [2] /Downloads/Aaron Doc New.pdf

  Delete which files? (enter numbers e.g. 1,2 or 's' to skip):
```

---

## Installation

Make sure you have [Rust installed](https://rustup.rs), then:

```bash
git clone https://github.com/yourusername/ditto
cd ditto
cargo install --path .
```

After that you can run `ditto` from anywhere on your system.

---

## Usage

### Scan a directory

```bash
ditto ~/Downloads
```

Recursively scans the folder, shows duplicate clusters, and prompts you to delete files interactively.

### Preview without deleting

```bash
ditto ~/Downloads --dry-run
```

Shows all duplicate clusters but does not prompt for deletion. Nothing is touched on disk. Always a good idea to run this first.

---

## How It Works

1. Walks the given directory recursively using `walkdir`
2. Reads every file and computes its SHA-256 hash using `sha2`
3. Groups files that share the same hash into duplicate clusters
4. Displays each cluster and lets you choose which files to delete

Two files are considered duplicates only if they are **byte-for-byte identical**. File names, extensions, and locations are irrelevant — only the content matters.

---

## Interactive Deletion

For each duplicate cluster you get three options:

- Enter one or more numbers separated by commas (e.g. `1` or `1,2`) to delete those files
- Enter `s` to skip the cluster and leave all files untouched
- Invalid input also skips safely

At least one file is always left in place — `ditto` will never delete an entire cluster.

---

## Crates Used

| Crate | Purpose |
|-------|---------|
| [`walkdir`](https://docs.rs/walkdir) | Recursive directory traversal |
| [`sha2`](https://docs.rs/sha2) | SHA-256 file hashing |
| [`hex`](https://docs.rs/hex) | Encoding hash bytes as hex strings |
| [`clap`](https://docs.rs/clap) | CLI argument parsing |

---

## Limitations

- `ditto` reads entire files into memory before hashing. Very large files (multi-GB) may be slow or memory-intensive.
- No `--exclude` flag yet — running on directories with `.app` bundles or installer packages will surface noise from their internal files.
- Deletion is permanent. There is no undo. Always use `--dry-run` first.

---
