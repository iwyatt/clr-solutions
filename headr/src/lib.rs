use clap::Parser;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
pub struct Config {
	files: Vec<String>,
	lines: usize,
	bytes: Option<usize>,
}

#[derive(Parser, Debug)]
#[command(name = "headr")]
#[command(version, about, author, long_about = None)]
pub struct Args {
	#[arg(short,long, default_value_t = "-")]
	files: Vec<String>,

	#[arg(short,long, default_value_t = 10)]
	lines: usize,

	#[arg(short,long)]
	bytes: Option<usize>
}

pub fn run() -> MyResult<()>{
	let args = Args::parse();
	println!("{:#?}", args);
	Ok(())
}
