use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::{self, Png};
use crate::{Error, Result};

pub fn encode(args: EncodeArgs) -> Result<()> {
    let bytes = fs::read(args.file_path.clone())?;
    let mut png = Png::try_from(bytes.as_slice())?;

    let chunk = Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.as_bytes().to_vec(),
    )?;

    png.append_chunk(chunk);

    let output_path = match args.output_file {
        Some(path) => path,
        None => args.file_path,
    };

    fs::write(output_path, png.as_bytes())?;

    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    let bytes = fs::read(args.file_path)?;
    let png = Png::try_from(bytes.as_slice())?;
    let chunk = match png.chunk_by_type(&args.chunk_type) {
        Some(chunk) => chunk,
        None => {
            return Err("No chunk with the specified type was found in the file".into());
        }
    };

    let message = String::from_utf8(chunk.data().to_vec())?;
    println!("{}", message);

    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    let bytes = fs::read(args.file_path.clone())?;
    let mut png = Png::try_from(bytes.as_slice())?;

    png.remove_chunk(&args.chunk_type)?;
    fs::write(args.file_path, png.as_bytes())?;

    println!(
        "Successfully removed secret message from {} chunk",
        args.chunk_type
    );

    Ok(())
}

pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let bytes = fs::read(args.file_path.clone())?;
    let png = Png::try_from(bytes.as_slice())?;

    println!("{:?}", png.chunks());

    Ok(())
}
