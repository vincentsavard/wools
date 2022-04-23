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
        let mut hints = HashMap::with_capacity(Word::SIZE);
        let mut solution_by_position: HashMap<usize, char> = solution.chars().enumerate().collect();
        let mut guess_by_position: HashMap<usize, char> = guess.chars().enumerate().collect();

        // Greens
        for i in Word::POSITIONS {
            if let Some(guess_char) = guess_by_position.get(&i) {
                let solution_char = solution_by_position.get(&i).unwrap();
                if guess_char == solution_char {
                    hints.insert(i, Hint::Green);
                    guess_by_position.remove(&i).unwrap();
                    solution_by_position.remove(&i).unwrap();
                }
            }
        }

        // Yellows
        for i in Word::POSITIONS {
            if let Some(guess_char) = guess_by_position.get(&i) {
                for j in Word::POSITIONS {
                    if let Some(solution_char) = solution_by_position.get(&j) {
                        if guess_char == solution_char {
                            hints.insert(i, Hint::Yellow);
                            guess_by_position.remove(&i).unwrap();
                            solution_by_position.remove(&j).unwrap();
                            break;
                        }
                    }
                }
            }
        }

        // Blacks
        for i in Word::POSITIONS {
            if guess_by_position.contains_key(&i) {
                hints.insert(i, Hint::Black);
                guess_by_position.remove(&i).unwrap();
            }
        }

        let hints: [Hint; Word::SIZE] = Word::POSITIONS
            .map(|i| *hints.get(&i).unwrap())
            .collect::<Vec<Hint>>()
            .try_into()
            .unwrap();

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
