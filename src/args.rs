use clap::{Parser, ArgAction, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    version = "0.5", 
    about = "Archivator program based on LZ77/78 compressing algorithm"
)]

pub struct LZRSArgs
{
    #[arg(long, short, value_enum, default_value_t = CompressingMode::LZ78, help = "Select compression algorithm. LZ77 or LZ78")]
    pub method: CompressingMode,

    #[arg(long, short, action = ArgAction::SetTrue, help = "Compress the given files into .lzrs archive")]
    pub compress: bool,

    #[arg(long, short, action = ArgAction::SetTrue, help = "Decompress files from given .lzrs archive")]
    pub decompress: bool,

    #[arg(long, short, help = "Specify output name of archive")]
    pub output: Option<String>,

    #[arg(required = true, help = "Specify sequence of files to archive")]
    pub files: Vec<String>
}

#[derive(ValueEnum, Clone, Debug)]
pub enum CompressingMode
{
    LZ77,
    LZ78
}