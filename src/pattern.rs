use std::collections::HashMap;

use crate::word::Word;
use crate::Pattern::FromGuess;

/// A pattern formed by the characters in a word, encoded as an ordered sequence of [`Hint`]s.
#[derive(Debug)]
pub enum Pattern {
    /// A pattern and its guess word from which the pattern is created.
    FromGuess {
        guess: Word,
        hints: [Hint; Word::SIZE],
    },
}

impl Pattern {
    /// Creates a pattern from a guess knowing what the solution is.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wools::{Hint, Pattern};
    /// # use wools::Word;
    /// let pattern = Pattern::from_solution_and_guess(&Word::new("stunt"), &Word::new("attic"));
    /// let mut iter = pattern.hints();
    ///
    /// assert_eq!(Some(&Hint::Black), iter.next());
    /// assert_eq!(Some(&Hint::Green), iter.next());
    /// assert_eq!(Some(&Hint::Yellow), iter.next());
    /// assert_eq!(Some(&Hint::Black), iter.next());
    /// assert_eq!(Some(&Hint::Black), iter.next());
    /// assert_eq!(None, iter.next());
    /// ```
    pub fn from_solution_and_guess(solution: &Word, guess: &Word) -> Self {
        let mut hints = [Hint::Black; Word::SIZE];
        let mut solution_chars = Pattern::count_chars(solution);

        for (i, (guess_char, solution_char)) in guess.chars().zip(solution.chars()).enumerate() {
            let hint = match solution_chars.get_mut(&guess_char) {
                Some(0) | None => Hint::Black,
                Some(count) => {
                    *count -= 1;

                    if guess_char == solution_char {
                        Hint::Green
                    } else {
                        Hint::Yellow
                    }
                }
            };

            hints[i] = hint;
        }

        FromGuess {
            guess: guess.clone(),
            hints,
        }
    }

    /// Returns an iterator over the [`Hint`]s of the pattern.
    pub fn hints(&self) -> impl Iterator<Item = &Hint> {
        match self {
            FromGuess { hints, .. } => hints.iter(),
        }
    }

    fn count_chars(word: &Word) -> HashMap<char, usize> {
        let mut chars = HashMap::with_capacity(Word::SIZE);

        for char in word.chars() {
            let count = chars.entry(char).or_insert_with(|| 0_usize);
            *count += 1;
        }

        chars
    }
}

/// A hint used to constrain the set of characters that may appear in the solution.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hint {
    /// A green hint means that the same character is at the same position in the solution.
    Green,
    /// A yellow hint means that the same character appears at least once more at a different
    /// position in the solution;
    Yellow,
    /// A black hint means that the same character does not appear anymore in the solution.
    Black,
}

#[cfg(test)]
mod tests {
    use crate::pattern::Hint;
    use crate::{Pattern, Word};

    #[test]
    fn given_no_guess_char_matches_when_from_solution_and_guess_then_every_hint_is_black() {
        let pattern = Pattern::from_solution_and_guess(&Word::new("watch"), &Word::new("prime"));
        let mut iter = pattern.hints();

        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn given_some_guess_chars_match_when_from_solution_and_guess_then_matched_chars_are_green() {
        let pattern = Pattern::from_solution_and_guess(&Word::new("story"), &Word::new("stare"));
        let mut iter = pattern.hints();

        assert_eq!(Some(&Hint::Green), iter.next());
        assert_eq!(Some(&Hint::Green), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Green), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn given_a_char_match_and_is_guessed_an_extra_time_when_from_solution_and_guess_then_extra_char_is_black(
    ) {
        let pattern = Pattern::from_solution_and_guess(&Word::new("store"), &Word::new("salsa"));
        let mut iter = pattern.hints();

        assert_eq!(Some(&Hint::Green), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn given_some_guess_chars_are_misplaced_when_from_solution_and_guess_then_misplaced_chars_are_yellow(
    ) {
        let pattern = Pattern::from_solution_and_guess(&Word::new("prime"), &Word::new("sharp"));
        let mut iter = pattern.hints();

        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Yellow), iter.next());
        assert_eq!(Some(&Hint::Yellow), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn given_a_char_is_misplaced_twice_and_appears_in_solution_once_when_from_solution_and_guess_then_extra_char_is_black(
    ) {
        let pattern = Pattern::from_solution_and_guess(&Word::new("prism"), &Word::new("apple"));
        let mut iter = pattern.hints();

        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Yellow), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn given_a_char_is_placed_once_correctly_and_misplaced_once_when_from_solution_and_guess_then_chars_are_green_and_yellow(
    ) {
        let pattern = Pattern::from_solution_and_guess(&Word::new("stunt"), &Word::new("attic"));
        let mut iter = pattern.hints();

        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Green), iter.next());
        assert_eq!(Some(&Hint::Yellow), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn given_a_char_is_placed_once_correctly_and_misplaced_once_and_appears_an_extra_time_when_from_solution_and_guess_then_chars_are_green_and_yellow_and_black(
    ) {
        let pattern = Pattern::from_solution_and_guess(&Word::new("leech"), &Word::new("tepee"));
        let mut iter = pattern.hints();

        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Green), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(Some(&Hint::Yellow), iter.next());
        assert_eq!(Some(&Hint::Black), iter.next());
        assert_eq!(None, iter.next());
    }
}
