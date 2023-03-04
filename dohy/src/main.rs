use clap::Parser;
use dothyphen::translate;

/// Simple program to translate an ASCII text to Morse code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Content to translate into Morse
    #[arg(short, long)]
    pub translate: String,
}

pub fn main() {
    let args = Args::parse();
    let translated_string = translate(&args.translate);
    println!("{}", translated_string);
}
