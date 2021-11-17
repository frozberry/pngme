mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::PngMeArgs;
use clap::Parser;
use png::Png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let opts: PngMeArgs = PngMeArgs::parse();

    match opts {
        PngMeArgs::Decode(a) => commands::decode(a),
        PngMeArgs::Encode(a) => commands::encode(a),
        PngMeArgs::Remove(a) => commands::remove(a),
        PngMeArgs::Print(a) => commands::print_chunks(a),
    }
}
