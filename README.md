# ATTENTION! This is a backup development branch. The code which is pushed here is not complete and the app is not runnable. Stable version can be downloaded from master branch.
# lzrs - Implementation of LZ77-based archivator made in Rust

### Description
- Project supports archivation/dearchivation of files using LZ77 compression algorithm into .lzrs archive
- To get description of the program, run
```
cargo run -- -h
```
- To download and run the project:
```
git clone https://github.com/enderavour/lz77-rs.git
```
```
cd lz77-rs
```
To create an archive (.lzrs):
```
cargo run -- -c file1.txt file2.txt -o archive.lzrs
```
To extract files from archive:
```
cargo run -- -d archive.lzrs
```

- The source code of LZ77 compression and decompression algorithms was partially inspired by and rewritten from [wolfie-things/lz77-algorithm](https://github.com/wolfie-things/lz77-algorithm)
- Contributors are welcomed.

### TODO issues
- There still exists issue where trash bytes are added in the end of extracted files, which makes several binary formats (such as PDF) corrupted. Probably the issue is somewhere during serialization of headers or compressed blobs into archive (Fixed ✅)
- Slow archivation of multiple files. Possibly fix it by reworking some MMF stuff

### Command Line Arguments quick references
| Argument | Description                                 |
| :------- | ------------------------------------------- | 
| -c       | Compress the given files into .lzrs archive |
| -d       | Decompress files from given .lzrs archive   |
| -o       | Specify output name of archive              |
| -h       | Print help                                  | 
| -v       | Print version                               | 

 
### Used crates
- [memmap2](https://crates.io/crates/memmap2) - A Rust library for cross-platform memory mapped IO.
- [clap](https://crates.io/crates/clap) - A simple to use, efficient, and full-featured Command Line Argument Parser