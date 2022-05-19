//! Tools for the Wordle game.

pub use crate::constraint::Constraints;
pub use crate::pattern::{Hint, Pattern};
pub use crate::word::Word;

mod constraint;
mod pattern;
mod word;

/// Filters out the words using the solution and the guesses, so that only the possible solutions
/// remain.
///
/// # Examples
///
/// ```
/// # use wools::{Word, filter};
/// let words = [Word::new("apple"), Word::new("prime")];
/// let solutions = filter(&words, &Word::new("apple"), &[Word::new("prime")]);
///
/// assert_eq!(vec!(&Word::new("apple")), solutions);
/// ```
pub fn filter<'a>(words: &'a [Word], solution: &Word, guesses: &[Word]) -> Vec<&'a Word> {
    let constraints = guesses
        .iter()
        .map(|guess| Constraints::from_pattern(&Pattern::from_solution_and_guess(solution, guess)))
        .collect::<Vec<Constraints>>();

    words
        .iter()
        .filter(|word| constraints.iter().all(|pattern| pattern.matches(word)))
        .collect()
}

/// Finds the words which produce the same hints given the solution.
///
/// # Examples
///
/// ```
/// # use wools::{Hint, matches, Word};
/// let words = [Word::new("cargo"), Word::new("babel"), Word::new("orbit")];
/// let hints = [Hint::Black, Hint::Green, Hint::Black, Hint::Black, Hint::Black];
/// let matches = matches(&words, &Word::new("cargo"), &hints);
///
/// assert_eq!(vec!(&Word::new("babel")), matches);
/// ```
pub fn matches<'a>(
    words: &'a [Word],
    solution: &Word,
    hints: &[Hint; Word::SIZE],
) -> Vec<&'a Word> {
    words
        .iter()
        .filter(|word| Pattern::from_solution_and_guess(solution, word).hints == *hints)
        .collect()
}

/// Filters out the words using the guesses and hints, so that only the possible solutions remain.
///
/// # Examples
///
/// ```
/// # use wools::{Hint, solve, Word};
/// let words = [Word::new("cargo"), Word::new("babel"), Word::new("orbit")];
/// let guess = Word::new("pants");
/// let hints = [Hint::Black, Hint::Green, Hint::Black, Hint::Black, Hint::Black];
/// let solutions = solve(&words, &[(guess, hints)]);
/// ```
pub fn solve<'a>(
    words: &'a [Word],
    guesses_and_hints: &[(Word, [Hint; Word::SIZE])],
) -> Vec<&'a Word> {
    let constraints = guesses_and_hints
        .iter()
        .map(|(guess, hints)| {
            Constraints::from_pattern(&Pattern::from_guess_and_hints(guess, hints))
        })
        .collect::<Vec<Constraints>>();

    words
        .iter()
        .filter(|word| constraints.iter().all(|pattern| pattern.matches(word)))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{filter, matches, solve, Hint, Word};

    #[test]
    fn given_guess_is_solution_when_filter_then_no_other_words_can_be_the_solution() {
        let words = ["apple", "prime", "plume", "torch", "watch", "soles"]
            .into_iter()
            .map(Word::new)
            .collect::<Vec<Word>>();
        let solutions = filter(&words, &Word::new("apple"), &[Word::new("apple")]);

        assert_eq!(vec![&Word::new("apple")], solutions);
    }

    #[test]
    fn when_guess_then_return_only_words_that_can_be_the_solution() {
        let words = ["apple", "prime", "plume", "torch", "watch", "soles"]
            .into_iter()
            .map(Word::new)
            .collect::<Vec<Word>>();
        let solutions = filter(&words, &Word::new("apple"), &[Word::new("coupe")]);

        assert_eq!(vec![&Word::new("apple"), &Word::new("prime")], solutions);
    }

    #[test]
    fn given_multiple_guesses_when_filter_then_return_only_words_that_can_be_the_solution() {
        let words = ["apple", "flock", "adept", "wiped", "nepal"]
            .into_iter()
            .map(Word::new)
            .collect::<Vec<Word>>();
        let guesses = ["pouch", "empty", "viper", "lapse"]
            .into_iter()
            .map(Word::new)
            .collect::<Vec<Word>>();
        let solutions = filter(&words, &Word::new("apple"), &guesses);

        assert_eq!(vec![&Word::new("apple")], solutions);
    }

    #[test]
    fn given_hints_are_all_green_when_matches_then_only_solution_matches() {
        let words = ["apple", "prime", "plume", "torch", "watch", "soles"]
            .into_iter()
            .map(Word::new)
            .collect::<Vec<Word>>();
        let hints = [
            Hint::Green,
            Hint::Green,
            Hint::Green,
            Hint::Green,
            Hint::Green,
        ];
        let matches = matches(&words, &Word::new("apple"), &hints);

        assert_eq!(vec![&Word::new("apple")], matches);
    }

    #[test]
    fn given_hints_when_matches_then_only_possible_words_match() {
        let words = ["apple", "prime", "plume", "phone", "torch", "watch"]
            .into_iter()
            .map(Word::new)
            .collect::<Vec<Word>>();
        let hints = [
            Hint::Yellow,
            Hint::Black,
            Hint::Black,
            Hint::Black,
            Hint::Green,
        ];
        let matches = matches(&words, &Word::new("apple"), &hints);

        assert_eq!(vec![&Word::new("prime"), &Word::new("phone")], matches);
    }

    #[test]
    fn given_guess_and_hints_when_solve_then_filter_out_non_possible_words() {
        let words = ["apple", "prime", "plume", "torch", "watch", "soles"]
            .into_iter()
            .map(Word::new)
            .collect::<Vec<Word>>();
        let guess = Word::new("coupe");
        let hints = [
            Hint::Black,
            Hint::Black,
            Hint::Black,
            Hint::Yellow,
            Hint::Green,
        ];
        let solutions = solve(&words, &[(guess, hints)]);

        assert_eq!(vec![&Word::new("apple"), &Word::new("prime")], solutions);
    }
}
