# lzrs - Implementation of LZ77/78-based archivator made in Rust

### Description
- Project supports archivation/dearchivation of files using LZ77 and LZ78 compression algorithms into .lzrs archive
- To get description of the program, run
```
cargo run -- -h
```
- To download and run the project:
```
git clone https://github.com/enderavour/lzrs.git
```
```
cd lz77-rs
```
To create an archive (.lzrs):
```
cargo run -- -c file1.txt file2.txt -o archive.lzrs
```
```
cargo run -- -m lz78 -c file1.txt file2.txt -o archive.lzrs
```
To extract files from archive:
```
cargo run -- -d archive.lzrs
```

- The source code of LZ77 compression and decompression algorithm was partially inspired by and rewritten from [wolfie-things/lz77-algorithm](https://github.com/wolfie-things/lz77-algorithm)
- The source code of LZ78 compresion and decompression algorithm was partially inspired by and rewritten from [Kumar-laxmi/Algorithms](https://github.com/Kumar-laxmi/Algorithms/blob/main/C++/Data%20Compression/LZ78.cpp)
- Contributors are welcomed.

### TODO issues
- Slow archivation of multiple files. Possibly fix it by reworking some MMF stuff

### Command Line Arguments quick references
| Argument | Description                                                     |
| :------- | --------------------------------------------------------------- | 
| -c       | Compress the given files into .lzrs archive                     |
| -d       | Decompress files from given .lzrs archive                       |
| -o       | Specify output name of archive                                  |
| -h       | Print help                                                      | 
| -v       | Print version                                                   | 
| -m       | Select compression algorithm. LZ77 or LZ78. Default one is LZ77 |

 
### Used crates
- [memmap2](https://crates.io/crates/memmap2) - A Rust library for cross-platform memory mapped IO.
- [clap](https://crates.io/crates/clap) - A simple to use, efficient, and full-featured Command Line Argument Parser