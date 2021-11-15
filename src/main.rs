mod args;
mod chunk;
mod chunk_type;
mod png;
// mod commands;
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let opts: args::PngMeArgs = args::PngMeArgs::parse();

    println!("opts");
    Ok(())
}
