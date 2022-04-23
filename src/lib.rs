//! Tools for the Wordle game.

pub use crate::constraint::Constraints;
pub use crate::pattern::{Hint, Pattern};
pub use crate::word::Word;

mod constraint;
mod pattern;
mod word;

/// Filters out the words using the solution and the guesses, so that only the possible solutions
/// remains.
///
/// # Examples
///
/// ```
/// # use std::str::FromStr;
/// # use wools::Word;
/// # use wools::filter;
/// let words = [Word::new("apple"), Word::new("prime")];
/// let solutions = filter(&words, &Word::new("apple"), &[Word::new("prime")]);
///
/// assert_eq!(vec!(&Word::new("apple")), solutions);
/// ```
pub fn filter<'a>(words: &'a [Word], solution: &Word, guesses: &[Word]) -> Vec<&'a Word> {
    let patterns = guesses
        .iter()
        .map(|guess| Constraints::from_pattern(&Pattern::from_solution_and_guess(solution, guess)))
        .collect::<Vec<Constraints>>();

    words
        .iter()
        .filter(|word| patterns.iter().all(|pattern| pattern.matches(word)))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{filter, Word};

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
}
