use std::hash::Hash;
use std::sync::{Arc, LazyLock};
use std::collections::HashMap;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::letters::LETTERS;

pub static ADJACENTLETTERS: LazyLock<FxHashMap<Arc<char>, FxHashSet<Arc<char>>> = LazyLock::new(||
    {
        let mut temp: FxHashMap<Arc<char>, FxHashSet<Arc<char>>> = FxHashMap::new();
        // 'A': 'Q', 'W', 'S', 'Z'
        temp.insert(LETTERS[0].clone(), FxHashSet::from([
            LETTERS[16].clone(), LETTERS[22].clone(), LETTERS[18].clone(), LETTERS[Z].clone()
        ]));

        //     0: 'A',
        //     1: 'B',
        //     2: 'C',
        //     3: 'D',
        //     4: 'E',
        //     5: 'F',
        //     6: 'G',
        //     7: 'H',
        //     8: 'I',
        //     9: 'J',
        //     10: 'K',
        //     11: 'L',
        //     12: 'M',
        //     13: 'N',
        //     14: 'O',
        //     15: 'P',
        //     16: 'Q',
        //     17: 'R',
        //     18: 'S',
        //     19: 'T',
        //     20: 'U',
        //     21: 'V',
        //     22: 'W',
        //     23: 'X',
        //     24: 'Y',
        //     25: 'Z'
        temp
    }
);
