use std::sync::{Arc, LazyLock};

pub static LETTERS: LazyLock<[Arc<char>; 26]> = LazyLock::new(||
    {[
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
    ]}
);
