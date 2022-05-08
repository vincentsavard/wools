use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// A word for which its length is strictly defined as [`Word::SIZE`], and for which characters are
/// alphabetical and normalized.
#[derive(Clone, Debug, PartialEq)]
pub struct Word {
    word: String,
}

impl Word {
    /// The size that each word must have, in unicode scalar value count.
    pub const SIZE: usize = 5;

    /// Creates a new word from a string, or panics if it cannot.
    ///
    /// For more information, see [`Word::from_str`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use wools::Word;
    /// assert_eq!(String::from("saute"), Word::new("sauté").to_string())
    /// ```
    pub fn new(word: &str) -> Self {
        Word::from_str(word).unwrap()
    }

    /// Returns an iterator over the normalized characters of the word.
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.word.chars()
    }
}

impl FromStr for Word {
    type Err = String;

    /// Creates a new word from a string. Normalizes the word in the process, making it lowercase,
    /// and transliterating some characters.
    ///
    /// Returns an error if the provided word:
    /// * has a length which is not exactly [`Word::SIZE`];
    /// * contains non-transliterable characters such as `'`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use wools::Word;
    /// let word = Word::from_str("apple").unwrap();
    ///
    /// assert_eq!(String::from("apple"), word.to_string());
    /// ```
    ///
    /// An error is returned when words are too short or too long:
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use wools::Word;
    /// assert!(Word::from_str("cut").is_err());
    /// assert!(Word::from_str("potato").is_err());
    /// ```
    ///
    /// Transliterable and uppercase characters are converted:
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use wools::Word;
    /// let word = Word::from_str("SAUTÉ").unwrap();
    ///
    /// assert_eq!(String::from("saute"), word.to_string());
    /// ```
    fn from_str(word: &str) -> Result<Self, Self::Err> {
        if word.chars().count() != Word::SIZE {
            return Err(format!("word is not {}-character long", Word::SIZE));
        }

        let word = word
            .to_lowercase()
            .chars()
            .map(|c| match c {
                'é' | 'ê' | 'ë' => 'e',
                'ó' | 'ô' | 'ö' => 'o',
                'à' => 'a',
                'ü' => 'u',
                'ñ' => 'n',
                c => c,
            })
            .collect::<String>();

        if word.chars().all(|c| ('a'..='z').contains(&c)) {
            Ok(Word { word })
        } else {
            Err("word contains non-alphabetical characters".to_string())
        }
    }
}

impl Display for Word {
    /// Formats the [`Word`] into a `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use wools::Word;
    /// let word = Word::from_str("apple").unwrap();
    /// assert_eq!("apple", format!("{}", word))
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.word)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Word;

    #[test]
    fn given_word_is_too_short_when_from_str_then_return_error() {
        assert!(Word::from_str("cut").is_err());
    }

    #[test]
    fn given_word_is_too_long_when_from_str_then_return_error() {
        assert!(Word::from_str("potato").is_err());
    }

    #[test]
    fn given_word_contains_non_alphabetic_characters_when_from_str_then_return_error() {
        assert!(Word::from_str("bob's").is_err());
    }

    #[test]
    fn given_word_contains_uppercase_characters_when_from_str_then_lowercase_characters() {
        assert_eq!("apple", Word::from_str("APPLE").unwrap().to_string());
    }

    #[test]
    fn given_word_contains_transliterable_characters_when_from_str_then_transliterate_characters() {
        assert_eq!("eeooo", Word::from_str("éêöóô").unwrap().to_string());
        assert_eq!("oaunx", Word::from_str("öàüñx").unwrap().to_string());
    }

    #[test]
    fn when_chars_then_return_iterator_over_chars() {
        let word = Word::new("apple");
        let mut iter = word.chars();

        assert_eq!(Some('a'), iter.next());
        assert_eq!(Some('p'), iter.next());
        assert_eq!(Some('p'), iter.next());
        assert_eq!(Some('l'), iter.next());
        assert_eq!(Some('e'), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn when_format_then_return_normalized_word() {
        assert_eq!("apple", format!("{}", Word::new("apple")));
    }
}
