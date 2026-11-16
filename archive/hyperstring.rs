use rustc_hash::{FxHashMap, FxHashSet};
use std::sync::Arc;
use crate::jointset::JointSet;
use crate::structure::Joint;
use crate::letters::LETTERS;

/// # `HyperString`
///
/// A struct containing multidimensional representations of a corresponding `String` word
///
/// ## Fields
///
/// * `joints`: A `FxHashSet` containing an `Arc` of each `Joint`
///   * Each `Joint` is wrapped within an `Arc` to ensure reuse across defining all words
///   * Each `Joint` is contained within a `HashSet` for O(1) lookups
///     * Since each `Joint` holds its full state within a given word
/// * `letters_to_joints`: A `FxHashMap` which maps `Arc<char>` -> `Vec<Arc<Joint>>`
///   * The `Vec<Arc<Joint>>` is each joint that contains the given `char`
/// * `indices_to_joints`: An `index` mapping to each `Joint`
///   * Idealogically equivalent to a `String`, but using the `Arc<Joint>` for `clone`ing
///   reuse and for maximizing information availability within queries
/// * `ordinals_to_joints`: As of yet, unneccessary, but possibly meaningful within
///   queries when more is understood about hyperedge relations
///
/// ## Further Description
///
/// ### `joints` is a hyperedge
///
/// We make use of a `FxHashSet` because each `Joint` contains the full representation of state
/// for their given character. Therefore, items do not need ordering, and we prioritize
/// O(1) lookups for `Joint` membership for hyperedge comparison.
///
/// ### `letters_to_joints`, `ordinals_to_joints`, and `indices_to_joints` each represent hypergraphs
///
/// For `letters_to_joints`, each hyperedge represents "characters present
/// within this word". The vertices bounded by these edges of characters are
/// "all Joints present within this word which contain this character".
/// A `FxHashSet` is used to hold the `Joint` values, since we don't care about their order,
/// but instead only care about membership
///
/// For `ordinals_to_joints`, each hyperedge represents "the set of ordinals
/// from first..`last` where `last` is the largest occurrence of a character
/// within this word". The vertices bounded by these edges of ordinals are
/// "all Joints whose character occurs at the given ordinal within its word".
///
/// > `ordinals_to_joints` is represented as a nested `Vec` of `FxHashSet`, since it's less
/// > costly to create and the conversion of ordinal -> index is trivial (ordinal -1 = index)
/// > A `FxHashSet` is used because we don't care about the order of letters within each
/// > ordinal level, only their membership
///
/// For `indices_to_joints`, each hyperedge represents "the index at which
/// a given `Joint` occurs within its word". Each hyperedge contains a single
/// vertex, since the mapping of indices -> characters is 1:1.
///
pub struct HyperString {
    joints: FxHashSet<Arc<Joint>>,
    letters_to_joints: FxHashMap<Arc<char>, FxHashSet<Arc<Joint>>>,
    indices_to_joints: Vec<Arc<Joint>>,
    ordinals_to_joints: Vec<FxHashSet<Arc<Joint>>>,
}

impl HyperString {
    fn new(input: String, jointset: Arc<JointSet>) -> Self {
        let convert: FxHashSet<Arc<Joint>>
        for letter in input {

        }
        HyperString {

        }
    }
}
