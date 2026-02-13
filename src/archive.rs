use std::io::{self, Write};
use crate::{LOOKAHEAD_BUFFER_SIZE, SEARCH_BUFFER_SIZE, dearchive::IntoBytes, lz77, lz78};
use std::fs::File;
use memmap2::MmapOptions;
use rand::RngExt;
use crate::args::CompressingMode;

pub struct LZRSEntry
{
    pub compressed_size: u64,
    pub original_size: u64,
    pub data_offset: u64,
    pub file_name: String
}

pub struct LZRSArchiveBuilder
{
    entries: Vec<LZRSEntry>,
    random_salt: u8,
    compression_method: u32,
    compressed_blobs: Vec<Vec<u8>>
}

impl LZRSArchiveBuilder
{
    pub fn new() -> Self
    {
        let mut rng = rand::rng();
        LZRSArchiveBuilder { 
            entries: Vec::new(), 
            random_salt: rng.random_range(1..255),
            compression_method: 0,
            compressed_blobs: Vec::new() 
        }
    }

    pub fn set_compression_method(&mut self, compressing_mode: CompressingMode)
    {
        self.compression_method = match compressing_mode 
        {
            CompressingMode::LZ77 => 77,
            CompressingMode::LZ78 => 78
        };
    }

    fn header_size(&self) -> u64
    {
        let mut size = 0;

        size += 4; // signature
        size += 1; // random salt
        size += 4; // compression method (LZ77 or LZ78)
        size += 8; // entries count
        for entry in self.entries.iter()
        {
            size += 8 + 8 + 8 + 8 + 1 + entry.file_name.len() as u64;
        }
        size
    }

    fn finalize_offsets(&mut self)
    {
        let mut offset = self.header_size();

        for entry in self.entries.iter_mut()
        {
            entry.data_offset = offset;
            offset += entry.compressed_size;
        }
    }

    pub fn add_file(&mut self, name: String, data: &[u8], compressing_mode: CompressingMode)
    {
        let mut compressed_data: Vec<u8>; 
        
        match compressing_mode 
        {
            CompressingMode::LZ77 => 
            {
                compressed_data = lz77::compress(
             data, 
            SEARCH_BUFFER_SIZE, 
        LOOKAHEAD_BUFFER_SIZE
                ).to_bytes();
            }

            CompressingMode::LZ78 => 
            {
                compressed_data = lz78::compress(data).to_bytes();
            }
        }

        compressed_data.iter_mut().for_each(|b| *b ^= self.random_salt);

        self.entries.push(LZRSEntry { 
            compressed_size: compressed_data.len() as u64,
            original_size: data.len() as u64,
            data_offset: 0,
            file_name: name
        });

        self.compressed_blobs.push(compressed_data);
    }

    pub fn add_existing_file(&mut self, filename: String, compressing_mode: CompressingMode)
    {
        let mapped_file = unsafe { 
            MmapOptions::new().map(&File::open(filename.clone()).unwrap()).unwrap()
        };
        self.add_file(filename, mapped_file.iter().as_slice(), compressing_mode);
    }

    pub fn write<W: Write>(&mut self, mut w: W) -> io::Result<()>
    {
        self.finalize_offsets();

        // Serialization of header
        w.write_all(b"LZRS")?; // Signature

        w.write_all(&[self.random_salt])?; // Random XOR cypher value

        let compr_method = &mut self.compression_method.to_le_bytes();
        compr_method.iter_mut().for_each(|b| *b ^= self.random_salt);

        w.write_all(compr_method)?; // Compression Method

        let entries_count = &mut (self.entries.len() as u64).to_le_bytes();
        entries_count.iter_mut().for_each(|b| *b ^= self.random_salt);

        w.write_all(entries_count)?; // Entries Count

        for e in self.entries.iter_mut()
        {
            let compressed_size = &mut e.compressed_size.to_le_bytes();
            compressed_size.iter_mut().for_each(|b| *b ^= self.random_salt);
            w.write_all(compressed_size)?;
            
            let original_size = &mut e.original_size.to_le_bytes();
            original_size.iter_mut().for_each(|b| *b ^= self.random_salt);
            w.write_all(original_size)?;

            let data_offset = &mut e.data_offset.to_le_bytes();
            data_offset.iter_mut().for_each(|b| *b ^= self.random_salt);
            w.write_all(data_offset)?;

            let file_name = unsafe { e.file_name.as_bytes_mut() };
            file_name.iter_mut().for_each(|b| *b ^= self.random_salt);

            let name_len = file_name.len() as u64;

            let name_len_bytes = &mut name_len.to_le_bytes();
            name_len_bytes.iter_mut().for_each(|b| *b ^= self.random_salt);

            w.write_all(name_len_bytes)?;
            w.write_all(file_name)?;
        }

        for blob in self.compressed_blobs.iter()
        {
            w.write_all(blob)?;
        }

        Ok(())
    }

    pub fn write_to_file(&mut self, filename: String) -> io::Result<()>
    {
        let fh = File::create(filename)?;
        self.write(fh)?;
        Ok(())
    }
}