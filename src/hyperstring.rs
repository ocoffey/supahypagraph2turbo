use crate::structure::Joint;
use std::collections::{HashSet};
use rustc_hash::FxHashMap;
use std::sync::Arc;

pub struct HyperString {
    str: String,
    joints: HashSet<Arc<Joint>>,
    letters_to_indices: FxHashMap<Arc<char>, Vec<u8>>,

}
