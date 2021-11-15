use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser)]
pub struct EncodeArgs {
    file_path: PathBuf,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
}

#[derive(Parser)]
pub struct DecodeArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(Parser)]
pub struct RemoveArgs {
    file_path: PathBuf,
    chunk_type: String,
}

#[derive(Parser)]
pub struct PrintArgs {
    file_path: PathBuf,
}
