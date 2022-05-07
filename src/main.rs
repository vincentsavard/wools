use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::{Parser, Subcommand};

use wools::Word;

const DICTIONARY_FILE_PATH: &str = "/usr/share/dict/american-english";
const DEFAULT_WORDLE_URL: &str = "https://www.nytimes.com/games/wordle/index.html";

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
    /// Displays the list of valid, normalized words from the dictionary.
    Dict,
    /// Opens Wordle in the default browser.
    Open {
        #[clap(short, long, default_value = DEFAULT_WORDLE_URL)]
        url: OsString,
    },
}

fn main() -> Result<(), String> {
    let opt: Opt = Opt::parse();
    let words = load_words(opt.dictionary)?;

    match opt.command {
        Command::Filter { solution, guesses } => filter(words, solution, guesses),
        Command::Dict => dict(words),
        Command::Open { url } => open(url),
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

fn dict(words: Vec<Word>) -> Result<(), String> {
    for word in words {
        println!("{}", word);
    }

    Ok(())
}

fn open<S: AsRef<OsStr>>(url: S) -> Result<(), String> {
    let output = std::process::Command::new("xdg-open").arg(url).output();

    match output {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) if output.stderr.is_empty() => Err(output.status.to_string()),
        Ok(output) => Err(format!("{}", String::from_utf8_lossy(&output.stderr))),
        Err(error) => Err(error.to_string()),
    }
}
