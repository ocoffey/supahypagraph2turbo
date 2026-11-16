use trie_rs::{Trie, inc_search::{Answer, IncSearch}};
use louds_rs::LoudsNodeNum;
use termion::input::TermRead;
mod hyperedges;
use crate::hyperedges::ADJACENTLETTERS;

// load dictionary into trie
// run `query_until()` for the first letter input, _and_ for each adjacent letter
// upon new letter entry, `clone()` each existing letter's incremental search,
// then run the new letter, and each adjacent to _that_ letter with further `query_until()`

// This should work by using `query_until()` in a stepwise manner
// `query_until()`, then compare index to next depth index, scrap if it doesn't match
// Ok(Answer::PrefixAndMatch)
// insert 'HELLO'
// query_until for EACH of 'H':'Y','U','J','N','B','G'
// a singular step
fn main() {
    // something that loads the dictionary into the tree
    let trie: Trie<u8> = ["our", "dictionary", "entries", "here"].into_iter().collect();

    // something that grabs user input from typing letters
    // we can grab it from distinct user keystrokes
    let mut astdin = termion::async_stdin().keys();

    // continuous loop stdin
    while let Some(Ok(termion::event::Key::Char(mut k))) = astdin.next() {
        // match letter
        match k {
            'A'..'Z' => {
                k.make_ascii_lowercase();

            },
            'a'..'z' => {

            },
            '\\' => (), // go back one space
            _ => continue // skip??
        };
        // upon each new letter, clone our IncSearch for that letter
        // and each adjacent letter

    }
    let mut inc_search = trie.inc_search();

    assert_eq!(inc_search.query_until("he"), Ok(Answer::Prefix));
    let position = LoudsNodeNum::from(inc_search);

    // inc_search is dropped.
    let mut inc_search2 = IncSearch::resume(&trie.0, position);
    assert_eq!(inc_search2.query_until("llo"), Ok(Answer::Match));
}
