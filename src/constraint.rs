use std::collections::HashMap;

use crate::pattern::Hint::{Black, Green, Yellow};
use crate::pattern::Pattern;
use crate::word::Word;

/// A set of constraints for which words may be matched against.
pub struct Constraints {
    constraints: Vec<Constraint>,
}

impl Constraints {
    /// Constructs constraints from a pattern.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wools::{Constraints, Pattern, Word};
    /// let pattern = Pattern::from_solution_and_guess(&Word::new("apple"), &Word::new("prime"));
    /// let constraints = Constraints::from_pattern(&pattern);
    ///
    /// assert!(constraints.matches(&Word::new("spade")));
    /// assert!(!constraints.matches(&Word::new("forgo")));
    /// ```
    pub fn from_pattern(pattern: &Pattern) -> Self {
        let Pattern { guess, hints } = pattern;
        let mut constraints = Vec::new();
        let mut hints_by_char = HashMap::with_capacity(Word::SIZE);

        for (i, (c, hint)) in guess.chars().zip(hints).enumerate() {
            hints_by_char
                .entry(c)
                .or_insert_with(|| Vec::with_capacity(Word::SIZE))
                .push((i, hint));
        }

        for (&char, hints) in &hints_by_char {
            for (i, hint) in hints {
                constraints.push(match hint {
                    Green => Constraint::lock(*i, char),
                    Yellow | Black => Constraint::forbid(*i, char),
                });
            }

            let yellow_count = hints
                .iter()
                .filter(|(_, hint)| matches!(hint, Yellow))
                .count();
            let black_count = hints
                .iter()
                .filter(|(_, hint)| matches!(hint, Black))
                .count();

            if yellow_count > 0 || black_count > 0 {
                let green_positions = hints
                    .iter()
                    .filter(|(_, hint)| matches!(hint, Green))
                    .map(|(i, _)| *i)
                    .collect::<Vec<usize>>();

                if yellow_count > 0 {
                    let at_least = Constraint::at_least(
                        yellow_count,
                        Constraint::not_at(&green_positions),
                        char,
                    );
                    constraints.push(at_least);
                }

                if black_count > 0 {
                    let at_most = Constraint::at_most(
                        yellow_count,
                        Constraint::not_at(&green_positions),
                        char,
                    );
                    constraints.push(at_most);
                }
            }
        }

        Constraints { constraints }
    }

    /// Matches a word against the constraints, returning whether the constraints allow the word.
    pub fn matches(&self, word: &Word) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.matches(word))
    }
}

enum Constraint {
    AtLeast {
        positions: Vec<usize>,
        count: usize,
        char: char,
    },
    AtMost {
        positions: Vec<usize>,
        count: usize,
        char: char,
    },
}

impl Constraint {
    fn lock(position: usize, char: char) -> Self {
        Constraint::AtLeast {
            positions: vec![position],
            count: 1,
            char,
        }
    }

    fn forbid(position: usize, char: char) -> Self {
        Constraint::AtMost {
            positions: vec![position],
            count: 0,
            char,
        }
    }

    fn at_least(count: usize, positions: Vec<usize>, char: char) -> Self {
        Constraint::AtLeast {
            positions,
            count,
            char,
        }
    }

    fn at_most(count: usize, positions: Vec<usize>, char: char) -> Self {
        Constraint::AtMost {
            positions,
            count,
            char,
        }
    }

    fn not_at(positions: &[usize]) -> Vec<usize> {
        (0..Word::SIZE).filter(|i| !positions.contains(i)).collect()
    }

    fn positions(&self) -> &[usize] {
        match self {
            Constraint::AtLeast { positions, .. } => positions,
            Constraint::AtMost { positions, .. } => positions,
        }
    }

    fn char(&self) -> &char {
        match self {
            Constraint::AtLeast { char, .. } => char,
            Constraint::AtMost { char, .. } => char,
        }
    }

    fn matches(&self, word: &Word) -> bool {
        let char_count = word
            .chars()
            .enumerate()
            .filter(|(i, _)| self.positions().contains(i))
            .filter(|(_, c)| c == self.char())
            .count();

        match self {
            Constraint::AtLeast { count, .. } => char_count >= *count,
            Constraint::AtMost { count, .. } => char_count <= *count,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint::Constraints;
    use crate::{Pattern, Word};

    #[test]
    fn given_guess_is_solution_when_matches_then_pattern_matches_solution() {
        let pattern = Pattern::from_solution_and_guess(&Word::new("stare"), &Word::new("stare"));
        let constraints = Constraints::from_pattern(&pattern);

        assert!(constraints.matches(&Word::new("stare")));
    }

    #[test]
    fn given_guess_is_solution_when_matches_then_pattern_does_not_match_non_solution_words() {
        let pattern = Pattern::from_solution_and_guess(&Word::new("stare"), &Word::new("stare"));
        let constraints = Constraints::from_pattern(&pattern);

        assert!(!constraints.matches(&Word::new("start")));
        assert!(!constraints.matches(&Word::new("place")));
        assert!(!constraints.matches(&Word::new("piece")));
        assert!(!constraints.matches(&Word::new("watch")));
        assert!(!constraints.matches(&Word::new("toner")));
    }

    #[test]
    fn given_guess_contains_greens_when_matches_then_words_with_greens_match() {
        let pattern = Pattern::from_solution_and_guess(&Word::new("toner"), &Word::new("poser"));
        let constraints = Constraints::from_pattern(&pattern);

        assert!(constraints.matches(&Word::new("toner")));
        assert!(constraints.matches(&Word::new("boxer")));
        assert!(constraints.matches(&Word::new("coder")));
        assert!(constraints.matches(&Word::new("homer")));
        assert!(constraints.matches(&Word::new("joker")));
    }

    #[test]
    fn given_guess_contains_greens_and_blacks_when_matches_then_words_without_greens_do_not_match()
    {
        let pattern = Pattern::from_solution_and_guess(&Word::new("toner"), &Word::new("poser"));
        let constraints = Constraints::from_pattern(&pattern);

        assert!(!constraints.matches(&Word::new("tints")));
        assert!(!constraints.matches(&Word::new("tonal")));
        assert!(!constraints.matches(&Word::new("tanks")));
        assert!(!constraints.matches(&Word::new("tango")));
        assert!(!constraints.matches(&Word::new("tunic")));
    }

    #[test]
    fn given_guess_contains_blacks_when_matches_then_words_with_blacks_do_not_match() {
        let pattern = Pattern::from_solution_and_guess(&Word::new("toner"), &Word::new("poser"));
        let constraints = Constraints::from_pattern(&pattern);

        assert!(!constraints.matches(&Word::new("poser")));
        assert!(!constraints.matches(&Word::new("passe")));
        assert!(!constraints.matches(&Word::new("pasta")));
        assert!(!constraints.matches(&Word::new("posse")));
        assert!(!constraints.matches(&Word::new("pushy")));
    }

    #[test]
    fn given_guess_contains_yellows_when_matches_then_words_with_yellow_elsewhere_match() {
        let pattern = Pattern::from_solution_and_guess(&Word::new("larva"), &Word::new("stare"));
        let constraints = Constraints::from_pattern(&pattern);

        assert!(constraints.matches(&Word::new("larva")));
        assert!(constraints.matches(&Word::new("rayon")));
        assert!(constraints.matches(&Word::new("march")));
        assert!(constraints.matches(&Word::new("argon")));
        assert!(constraints.matches(&Word::new("radar")));
    }

    #[test]
    fn given_guess_contains_yellows_when_matches_then_words_with_yellow_at_same_position_do_not_match(
    ) {
        let pattern = Pattern::from_solution_and_guess(&Word::new("larva"), &Word::new("stare"));
        let constraints = Constraints::from_pattern(&pattern);

        assert!(!constraints.matches(&Word::new("alarm")));
        assert!(!constraints.matches(&Word::new("board")));
        assert!(!constraints.matches(&Word::new("charm")));
        assert!(!constraints.matches(&Word::new("dwarf")));
        assert!(!constraints.matches(&Word::new("ozark")));
    }

    #[test]
    fn given_guess_contains_yellows_when_matches_then_words_without_yellow_elsewhere_do_not_match()
    {
        let pattern = Pattern::from_solution_and_guess(&Word::new("larva"), &Word::new("stare"));
        let constraints = Constraints::from_pattern(&pattern);

        assert!(!constraints.matches(&Word::new("delve")));
        assert!(!constraints.matches(&Word::new("evils")));
        assert!(!constraints.matches(&Word::new("vowel")));
        assert!(!constraints.matches(&Word::new("veils")));
        assert!(!constraints.matches(&Word::new("solve")));
    }

    #[test]
    fn given_guess_contains_yellows_and_blacks_for_the_same_letter_when_matches_then_words_with_equal_occurrences_of_yellow_match(
    ) {
        let pattern = Pattern::from_solution_and_guess(&Word::new("tonal"), &Word::new("swoop"));
        let constraints = Constraints::from_pattern(&pattern);

        assert!(constraints.matches(&Word::new("tonal")));
        assert!(constraints.matches(&Word::new("ionic")));
        assert!(constraints.matches(&Word::new("toady")));
        assert!(constraints.matches(&Word::new("outer")));
        assert!(constraints.matches(&Word::new("ratio")));
    }

    #[test]
    fn given_guess_contains_yellows_and_blacks_for_the_same_letter_when_matches_then_words_with_fewer_occurrences_of_yellow_do_not_match(
    ) {
        let pattern = Pattern::from_solution_and_guess(&Word::new("tonal"), &Word::new("swoop"));
        let constraints = Constraints::from_pattern(&pattern);

        assert!(!constraints.matches(&Word::new("again")));
        assert!(!constraints.matches(&Word::new("burst")));
        assert!(!constraints.matches(&Word::new("flank")));
        assert!(!constraints.matches(&Word::new("night")));
        assert!(!constraints.matches(&Word::new("tibia")));
    }

    #[test]
    fn given_guess_contains_yellows_and_blacks_for_the_same_letter_when_matches_then_words_with_greater_occurrences_of_yellow_do_not_match(
    ) {
        let pattern = Pattern::from_solution_and_guess(&Word::new("tonal"), &Word::new("swoop"));
        let constraints = Constraints::from_pattern(&pattern);

        assert!(!constraints.matches(&Word::new("bloom")));
        assert!(!constraints.matches(&Word::new("oozed")));
        assert!(!constraints.matches(&Word::new("outdo")));
        assert!(!constraints.matches(&Word::new("rodeo")));
        assert!(!constraints.matches(&Word::new("motto")));
    }
}
