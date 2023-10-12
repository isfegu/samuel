use clap::{Parser, ValueEnum};
use dothyphen::*;

/// Simple program to translate an ASCII text to Morse code and vice versa
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Content to translate
    #[arg(short, long)]
    pub translate: String,
    /// Translate the content to Morse or ASCII
    #[arg(short, long, value_enum)]
    output: Output,
}

#[derive(Clone, Debug, ValueEnum)]
enum Output {
    /// Translate the content to Morse
    Morse,
    /// Translate the content to ASCII
    ASCII,
}

pub fn main() {
    let args = Args::parse();

    match args.output {
        Output::Morse => {
            let translated_string = translate::to_morse(&args.translate);
            println!("{}", translated_string);
        }
        Output::ASCII => {
            let translated_string = translate::to_ascii(&args.translate);
            println!("{}", translated_string);
        }
    }
}
