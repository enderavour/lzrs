use std::fs::File;
use crate::lz77::{self, Lz77Token};
use std::error::Error;
use std::io::{self, Write};
use std::process::exit;
use crate::archive::LZRSEntry;
use crate::lz78::{self, Lz78Token};

static mut COMPRESSION_METHOD: u32 = 0;
static mut RANDOM_SALT: u8 = 0;

pub struct DecompressedFileEntry
{
    name: String,
    contents: Vec<u8>
}

pub fn compose_entries(archive_source: &mut [u8]) -> Vec<LZRSEntry>
{
    let mut token_array = Vec::new();

    let signature = &archive_source[..4];
    if !(signature[0] == 0x4C && signature[1] == 0x5A && signature[2] == 0x52 && signature[3] == 0x53)
    {
        eprintln!("lzrs: Provided file signature is not correct");
        exit(-1);
    }

    let random_salt = *(&archive_source[4..5][0]);
    unsafe { RANDOM_SALT = random_salt; }

    let mut compression_method_buffer = [0u8; 4];
    compression_method_buffer.copy_from_slice(&archive_source[5..9]);
    compression_method_buffer.iter_mut().for_each(|b| *b ^= random_salt);
    let compression_method = u32::from_le_bytes(compression_method_buffer);

    unsafe { COMPRESSION_METHOD = compression_method };

    let mut buffer = [0u8; 8];
    buffer.copy_from_slice(&archive_source[9..17]);

    let mut index = 17;
    let entries_count = u64::from_le_bytes(buffer);

    for _ in 0..entries_count
    {
        let compressed_size_buf = &archive_source[index..index + 8];
        buffer.copy_from_slice(compressed_size_buf);
        buffer.iter_mut().for_each(|b| *b ^= random_salt);
        let compressed_size = u64::from_le_bytes(buffer);
        index += 8;
        let original_size_buf = &archive_source[index..index + 8];
        buffer.copy_from_slice(original_size_buf);
        buffer.iter_mut().for_each(|b| *b ^= random_salt);
        let original_size = u64::from_le_bytes(buffer);
        index += 8;
        let data_offset_buf = &archive_source[index..index + 8];
        buffer.copy_from_slice(data_offset_buf);
        buffer.iter_mut().for_each(|b| *b ^= random_salt);
        let data_offset = u64::from_le_bytes(buffer);
        index += 8;
        let name_len_buf = &archive_source[index..index + 8];
        buffer.copy_from_slice(name_len_buf);
        buffer.iter_mut().for_each(|b| *b ^= random_salt);
        let name_len = u64::from_le_bytes(buffer);
        index += 8;

        let filename_buf = &mut archive_source[index..index + name_len as usize];
        filename_buf.iter_mut().for_each(|b| *b ^= random_salt);
        let filename = str::from_utf8(filename_buf).unwrap();
        index += name_len as usize;
        token_array.push(LZRSEntry {
            compressed_size: compressed_size,
            original_size: original_size,
            data_offset: data_offset,
            file_name: filename.to_owned()
        });
    }
    token_array
}

pub fn decompress_file_payloads(archive_contents: &[u8], entries: Vec<LZRSEntry>) -> Vec<DecompressedFileEntry>
{
    let mut decompressed = Vec::new();
    for entry in entries
    {
        let start_index = entry.data_offset as usize;
        let end_index = start_index + entry.compressed_size as usize;
        let data = &archive_contents[start_index..end_index];
        
        if unsafe { COMPRESSION_METHOD } == 77
        {
            let mut tokens = Vec::new();
            let mut processed_len = 0;
            let mut buf = [0u8; 5];
            while processed_len + 5 <= data.len()
            {
                buf.copy_from_slice(&data[processed_len..processed_len + 5]);
                buf.iter_mut().for_each(|b| *b ^= unsafe { RANDOM_SALT });
                let token = Lz77Token::from_bytes(buf);
                tokens.push(token);
                processed_len += 5;
            }

            let mut decompressed_file_blob = lz77::decompress(&tokens);
            decompressed_file_blob.truncate(entry.original_size as usize);

            decompressed.push(DecompressedFileEntry { 
                name: entry.file_name.clone(),
                contents: decompressed_file_blob
            });
        }
        else if unsafe { COMPRESSION_METHOD } == 78
        {
            let mut tokens = Vec::new();
            let mut processed_len = 0;
            let mut buf = [0u8; 9];

            while processed_len + 9 <= data.len()
            {
                buf.copy_from_slice(&data[processed_len..processed_len + 9]);
                let token = Lz78Token::from_bytes(buf);
                tokens.push(token);
                processed_len += 9;
            }

            let mut decompressed_file_blob = lz78::decompress(&tokens);
            decompressed_file_blob.truncate(entry.original_size as usize);

            decompressed.push(DecompressedFileEntry { 
                name: entry.file_name.clone(),
                contents: decompressed_file_blob
            });
        }
    }
    decompressed
}

pub fn extract_archive(archive_payload: &mut [u8]) -> io::Result<()>
{
    let entries = compose_entries(archive_payload);

    let payloads = decompress_file_payloads(archive_payload, entries);

    extract_files(payloads)?;
    Ok(())
}

pub fn extract_files(entries: Vec<DecompressedFileEntry>) -> io::Result<()>
{
    for entry in entries
    {
        let mut fh = File::create(entry.name)?;
        fh.write_all(&entry.contents)?;
        fh.flush()?;
    }
    Ok(())
}

pub trait _SerializeToFile
{
    fn serizalize_to_file(&self, fh: &File) -> Result<(), Box<dyn Error>>;
}

pub trait IntoBytes
{
    fn to_bytes(&self) -> Vec<u8>;
}

impl _SerializeToFile for Vec<Lz77Token>
{
    fn serizalize_to_file(&self, mut fh: &File) -> Result<(), Box<dyn Error>>
    {
        for token in self
        {
            fh.write_all(&token.to_bytes())?;
        }
        Ok(())
    }
}

impl IntoBytes for Vec<Lz77Token>
{
    fn to_bytes(&self) -> Vec<u8> 
    {
        let mut bytes = Vec::new();
        for i in self
        {
            bytes.extend_from_slice(&i.to_bytes())
        }
        bytes
    }
}

impl _SerializeToFile for Vec<Lz78Token>
{
    fn serizalize_to_file(&self, mut fh: &File) -> Result<(), Box<dyn Error>> 
    {
        for token in self
        {
            fh.write_all(&token.to_bytes())?;
        }
        Ok(())
    }
}

impl IntoBytes for Vec<Lz78Token>
{
    fn to_bytes(&self) -> Vec<u8> 
    {
        let mut bytes = Vec::new();
        for i in self 
        {
            bytes.extend_from_slice(&i.to_bytes());
        }
        bytes
    }
}