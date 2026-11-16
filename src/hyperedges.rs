use std::sync::LazyLock;
use rustc_hash::{FxHashMap, FxHashSet};

pub static ADJACENTLETTERS: LazyLock<FxHashMap<char, FxHashSet<char>>> = LazyLock::new(||
    {
        let mut temp: FxHashMap<char, FxHashSet<char>> = FxHashMap::default();
        temp.insert('A', FxHashSet::from_iter(['Q', 'W', 'S', 'Z']));
        temp.insert('B', FxHashSet::from_iter(['V','G','H','N']));
        temp.insert('C', FxHashSet::from_iter(['X','D','F','V']));
        temp.insert('D', FxHashSet::from_iter(['E','R','F','C','X','S']));
        temp.insert('E', FxHashSet::from_iter(['R','D','S','W']));
        temp.insert('F', FxHashSet::from_iter(['R','T','G','V','C','D']));
        temp.insert('G', FxHashSet::from_iter(['T','Y','H','B','V','F']));
        temp.insert('H', FxHashSet::from_iter(['Y','U','J','N','B','G']));
        temp.insert('I', FxHashSet::from_iter(['O','K','J','U']));
        temp.insert('J', FxHashSet::from_iter(['U','I','K','M','N','H']));
        temp.insert('K', FxHashSet::from_iter(['I','O','L','M','J']));
        temp.insert('L', FxHashSet::from_iter(['K','O','P']));
        temp.insert('M', FxHashSet::from_iter(['N','J','K']));
        temp.insert('N', FxHashSet::from_iter(['B','H','J','M']));
        temp.insert('O', FxHashSet::from_iter(['P','L','K','I']));
        temp.insert('P', FxHashSet::from_iter(['L','O']));
        temp.insert('Q', FxHashSet::from_iter(['W','A']));
        temp.insert('R', FxHashSet::from_iter(['T','F','D','E']));
        temp.insert('S', FxHashSet::from_iter(['W','E','D','X','Z','A']));
        temp.insert('T', FxHashSet::from_iter(['Y','G','F','R']));
        temp.insert('U', FxHashSet::from_iter(['I','J','H','Y']));
        temp.insert('V', FxHashSet::from_iter(['C','F','G','B']));
        temp.insert('W', FxHashSet::from_iter(['E','S','A','Q']));
        temp.insert('X', FxHashSet::from_iter(['Z','S','D','C']));
        temp.insert('Y', FxHashSet::from_iter(['U','H','G','T']));
        temp.insert('Z', FxHashSet::from_iter(['A','S','X']));
        temp
    }
);
