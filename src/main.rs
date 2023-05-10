use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::{Parser, Subcommand};

use wools::{load_default_words, Hint, Word};

const DEFAULT_WORDLE_URL: &str = "https://www.nytimes.com/games/wordle/index.html";

#[derive(Parser)]
#[clap(version, about)]
struct Opt {
    /// Sets the path to the dictionary
    #[clap(short, long, value_parser)]
    dictionary: Option<PathBuf>,

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
    /// Finds the word matching the pattern knowing the solution
    Match {
        /// Sets the five-letter word as the solution
        #[clap()]
        solution: Word,
        /// Sets the pattern to match
        #[clap(name = "PATTERN", value_parser = parse_hints)]
        hints: [Hint; Word::SIZE],
    },
    /// Finds the words that may be the solution
    Solve {
        /// Sets the guess and its hints, separated by a comma
        #[clap(name = "GUESS", value_parser = parse_guess_and_hints)]
        guesses_and_hints: Vec<(Word, [Hint; Word::SIZE])>,
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
    let words = opt
        .dictionary
        .map(load_words)
        .unwrap_or_else(|| Ok(load_default_words()))?;

    match opt.command {
        Command::Filter { solution, guesses } => filter(words, solution, guesses),
        Command::Match { solution, hints } => matches(words, solution, hints),
        Command::Solve { guesses_and_hints } => solve(words, guesses_and_hints),
        Command::Dict => dict(words),
        Command::Open { url } => open(url),
    }
}

fn parse_hints(s: &str) -> Result<[Hint; Word::SIZE], String> {
    let s = s.to_lowercase();

    if s.chars().count() != Word::SIZE {
        return Err("pattern is not five-character long".to_string());
    } else if !s.chars().all(|c| matches!(c, 'g' | 'y' | 'b')) {
        return Err("pattern contains unsupported characters".to_string());
    }

    let hints: Vec<Hint> = s
        .chars()
        .map(|c| match c {
            'g' => Hint::Green,
            'y' => Hint::Yellow,
            'b' => Hint::Black,
            _ => unreachable!(),
        })
        .collect();

    Ok(hints.try_into().unwrap())
}

fn parse_guess_and_hints(s: &str) -> Result<(Word, [Hint; Word::SIZE]), String> {
    let parts: Vec<&str> = s.split(',').collect();

    if parts.len() != 2 {
        return Err("input cannot be split in two".to_string());
    }

    // SAFETY: `parts` is guaranteed to have a length of two.
    unsafe {
        let word = Word::from_str(parts.get_unchecked(0))?;
        let hints = parse_hints(parts.get_unchecked(1))?;

        Ok((word, hints))
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

fn matches(words: Vec<Word>, solution: Word, hints: [Hint; Word::SIZE]) -> Result<(), String> {
    for word in wools::matches(&words, &solution, &hints) {
        println!("{}", word);
    }

    Ok(())
}

fn solve(
    words: Vec<Word>,
    guesses_and_hints: Vec<(Word, [Hint; Word::SIZE])>,
) -> Result<(), String> {
    for word in wools::solve(&words, &guesses_and_hints) {
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
