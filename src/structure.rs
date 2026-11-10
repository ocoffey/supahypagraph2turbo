use std::sync::Arc;
use crate::letters::LETTERS;

/// # `Joint` struct
///
/// ## Fields
///
/// * `letter`: uses an Atomic Reference Counter (for future thread safety)
///   which wraps around a particular `char`acter
///   * `char` is used instead of more simple representation (u8) for future proofing
/// * `indexinword`: the index in which this particular letter occurs within its larger word
/// * `letterordinal`: the ordinality of this letter compared to like letters within the word
///   * By knowing which occurrence of a given letter we are looking at, we can enrich forms
///     of comparison between hyperedges
pub struct Joint {
    letter: Arc<char>,
    indexinword: u8,
    letterordinal: u8,
}

impl Joint {
    /// # `new()`
    ///
    /// The simplest constructor for our struct
    ///
    /// ## Arguments
    ///
    /// * `letter`: a character, which represents the current letter this `Joint` will wrap
    /// * `indexinword`: the index at which this character occurs within its word
    /// * `letterordinal`: the ordinality of this letter to identical letters within the word
    ///   * Represented by its index, or "ordinality - 1"
    ///
    /// ## Body
    ///
    /// This method performs a conversion on our passed `letter`. Since we are observing letters
    /// originating within the English alphabet, we employ `.to_digit()` with a radix of 36,
    /// which converts our character into a numeric representation of base 36 (which retains letter
    /// information, but translates it to a corresponding number). This number is 10 digits above index
    /// within the English alphabet.
    ///
    /// Suggesting `letter` is an English alphabetic letter, we convert it to its appropriate index
    /// within the English alphabet (by subtracting '10'), use this index to perform a lookup from our
    /// static table of letters, and clone the `Arc<char>` at that index.
    ///
    /// We trivially assign `indexinword` and `letterordinal`.
    ///
    /// ## Return
    ///
    /// `Self`, which in this case is the struct `Joint`
    fn new(letter: char, indexinword: u8, letterordinal: u8) -> Self {
        Joint {
            letter: match letter.to_digit(36) {
                Some(l) => LETTERS[(l - 10) as usize].clone(),
                _ => panic!()
            },
            indexinword,
            letterordinal,
        }
    }
}
