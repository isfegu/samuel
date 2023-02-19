use clap::Parser;
use samuel::morse::translate;

mod cli;

pub fn main() {
    let args = cli::Args::parse();
    let translated_string = translate(&args.translate);
    println!("{}", translated_string);
}
