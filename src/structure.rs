use std::sync::Arc;
use crate::letters::LETTERS;

pub struct Joint {
    index: u8,
    letter: Arc<char>,
}

impl Joint {
    fn new(index: u8, letter: char) -> Self {
        Joint {
            index: index,
            letter: match letter.to_digit(36) {
                Some(l) if (0..26).contains(&(l - 10 as u32)) => LETTERS[(l - 10) as usize].clone(),
                _ => panic!()
            },
        }
    }
}
