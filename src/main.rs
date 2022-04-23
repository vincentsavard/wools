use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::{Parser, Subcommand};

use wools::Word;

const DICTIONARY_FILE_PATH: &str = "/usr/share/dict/american-english";

#[derive(Parser)]
#[clap(version, about)]
struct Opt {
    /// Sets the path to the dictionary
    #[clap(short, long, parse(from_os_str), default_value = DICTIONARY_FILE_PATH)]
    dictionary: PathBuf,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Filters the list of words using the guesses
    Filter {
        /// Sets the five-letter word as the solution
        #[clap()]
        solution: Word,
        /// Sets the five-letter guesses to use to filter
        #[clap()]
        guesses: Vec<Word>,
    },
}

fn main() -> Result<(), String> {
    let opt: Opt = Opt::parse();
    let words = load_words(opt.dictionary)?;

    match opt.command {
        Command::Filter { solution, guesses } => filter(words, solution, guesses),
    }
}

fn load_words<P: AsRef<Path>>(dictionary_path: P) -> Result<Vec<Word>, String> {
    let file = File::open(dictionary_path).map_err(|err| err.to_string())?;
    let mut words = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| Word::from_str(&line).ok())
        .collect::<Vec<Word>>();
    words.dedup();
    Ok(words)
}

fn filter(words: Vec<Word>, solution: Word, guesses: Vec<Word>) -> Result<(), String> {
    for word in wools::filter(&words, &solution, &guesses) {
        println!("{}", word);
    }

    Ok(())
}
