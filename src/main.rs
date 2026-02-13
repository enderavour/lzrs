mod lz77;
mod lz78;
mod dearchive;
mod archive;
mod args;
use std::error::Error;
use std::fs::{File, OpenOptions};
use archive::LZRSArchiveBuilder;
use clap::Parser;
use memmap2::MmapOptions;
use args::LZRSArgs;
use std::process::exit;
use crate::args::CompressingMode;

// Constants for LZ77
const SEARCH_BUFFER_SIZE: usize = 256;
const LOOKAHEAD_BUFFER_SIZE: i32 = 64;

fn main() -> Result<(), Box<dyn Error>> 
{
    let command_line_args = LZRSArgs::parse();

    if command_line_args.compress && command_line_args.decompress
    {
        eprintln!("lzrs: Error: cannot enter compress and decompress in the same place.");
        exit(-1);
    }

    if command_line_args.compress
    {
        match command_line_args.method
        {
            CompressingMode::LZ77 => 
            {
                let mut builder = LZRSArchiveBuilder::new();
                builder.set_compression_method(CompressingMode::LZ77);
                for file_name in &command_line_args.files 
                {
                    builder.add_existing_file(
                        file_name.clone(), 
                        CompressingMode::LZ77
                    );
                }

                builder.write_to_file(command_line_args.output.unwrap_or("unnamed.lzrs".to_owned()))?;
            }
            CompressingMode::LZ78 => 
            {
                let mut builder = LZRSArchiveBuilder::new();
                builder.set_compression_method(CompressingMode::LZ78);
                
                for file_name in &command_line_args.files 
                {
                    builder.add_existing_file(
                        file_name.clone(), 
                        CompressingMode::LZ78
                    );
                }

                builder.write_to_file(command_line_args.output.unwrap_or("unnamed.lzrs".to_owned()))?;
            }
        }
    }
    
    if command_line_args.decompress
    {
        let entered_file_name = &command_line_args.files[0];
        if entered_file_name.ends_with(".lzrs") 
        {
            let archive = OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .open(entered_file_name)?;
            let mut mapped_archive = unsafe {
                MmapOptions::new().map_mut(&archive).unwrap()
            };

            dearchive::extract_archive(mapped_archive.as_mut())?;
        }
        else 
        {
            eprintln!("lzrs: Incorrect file format for decompress: {}", entered_file_name);
            return Err("File should have .lzrs extension".into());
        }
    }
    Ok(())
}
