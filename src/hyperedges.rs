use std::sync::LazyLock;
use rustc_hash::FxHashMap;

// Each HyperEdge set is stored within a `Vec` instead of a `HashSet`
// because the elements bounded by each hyperedge will be used
// exhaustively (we don't need to perform lookups within the hyperedge
// elements; we just need them all returned)
pub static ADJACENTLETTERS: LazyLock<FxHashMap<char, Vec<char>>> = LazyLock::new(||
    {
        let mut temp: FxHashMap<char, Vec<char>> = FxHashMap::default();
        temp.insert('A', Vec::from(['Q', 'W', 'S', 'Z']));
        temp.insert('B', Vec::from(['V','G','H','N']));
        temp.insert('C', Vec::from(['X','D','F','V']));
        temp.insert('D', Vec::from(['E','R','F','C','X','S']));
        temp.insert('E', Vec::from(['R','D','S','W']));
        temp.insert('F', Vec::from(['R','T','G','V','C','D']));
        temp.insert('G', Vec::from(['T','Y','H','B','V','F']));
        temp.insert('H', Vec::from(['Y','U','J','N','B','G']));
        temp.insert('I', Vec::from(['O','K','J','U']));
        temp.insert('J', Vec::from(['U','I','K','M','N','H']));
        temp.insert('K', Vec::from(['I','O','L','M','J']));
        temp.insert('L', Vec::from(['K','O','P']));
        temp.insert('M', Vec::from(['N','J','K']));
        temp.insert('N', Vec::from(['B','H','J','M']));
        temp.insert('O', Vec::from(['P','L','K','I']));
        temp.insert('P', Vec::from(['L','O']));
        temp.insert('Q', Vec::from(['W','A']));
        temp.insert('R', Vec::from(['T','F','D','E']));
        temp.insert('S', Vec::from(['W','E','D','X','Z','A']));
        temp.insert('T', Vec::from(['Y','G','F','R']));
        temp.insert('U', Vec::from(['I','J','H','Y']));
        temp.insert('V', Vec::from(['C','F','G','B']));
        temp.insert('W', Vec::from(['E','S','A','Q']));
        temp.insert('X', Vec::from(['Z','S','D','C']));
        temp.insert('Y', Vec::from(['U','H','G','T']));
        temp.insert('Z', Vec::from(['A','S','X']));
        temp
    }
);
