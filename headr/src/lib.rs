use clap::Parser;
use std::io::{self, Write};
use std::{fs::File, io::Read};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(name = "headr")]
#[command(version, about, author, long_about = None)]
pub struct Args {
    #[arg(index = 1)]
    pub files: String,

    #[arg(short, long, default_value_t = 10)]
    pub lines: usize,

    #[arg(short, long)]
    pub bytes: Option<usize>,
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    // read file to end
    let mut f = File::open(args.files)?;
    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;
    io::stdout().write_all(&buffer);
    Ok(())
}
