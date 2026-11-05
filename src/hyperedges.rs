use std::hash::Hash;
use std::sync::{Arc, LazyLock};
use std::collections::HashMap;
use rustc_hash::FxHasher;
use crate::letters::LETTERS;

pub static ADJACENTLETTERS: LazyLock<HashMap<Arc<char>, &'static [Arc<char>]>> = LazyLock::new(||
    {
        let mut temp: HashMap<Arc<char>, &[Arc<char>]> = HashMap::with_hasher(FxHasher);

            [
            LETTERS[0].clone(),
            Arc::new('A'),
            Arc::new('B'),
            Arc::new('C'),
            Arc::new('D'),
            Arc::new('E'),
            Arc::new('F'),
            Arc::new('G'),
            Arc::new('H'),
            Arc::new('I'),
            Arc::new('J'),
            Arc::new('K'),
            Arc::new('L'),
            Arc::new('M'),
            Arc::new('N'),
            Arc::new('O'),
            Arc::new('P'),
            Arc::new('Q'),
            Arc::new('R'),
            Arc::new('S'),
            Arc::new('T'),
            Arc::new('U'),
            Arc::new('V'),
            Arc::new('W'),
            Arc::new('X'),
            Arc::new('Y'),
            Arc::new('Z')
        ]);
    }
);
