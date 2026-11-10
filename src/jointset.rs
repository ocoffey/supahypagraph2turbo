use std::collections::{HashMap, HashSet};
use std::sync::{RwLock, Arc};
use crate::structure::Joint;

/// Temporary struct used for generation of unique Joints
pub struct JointProgress {
    data: RwLock<HashMap<Arc<char>, HashMap<u8, HashMap<u8, Arc<Joint>>>>>,
}

impl JointProgress {
    pub fn new() -> Self {
        JointProgress {
            data: RwLock::new(HashMap::new()),
        }
    }

    fn consume(self) -> HashMap<Arc<char>, HashMap<u8, HashMap<u8, Arc<Joint>>>> {
        self.data.into_inner().unwrap()
    }
}

/// Completed set of all Joints which appear from our dictionary
pub struct JointSet {
    data: RwLock<HashSet<Arc<Joint>>>,
}

impl From<JointProgress> for JointSet {
    /// Once our `JointProgress` struct is complete,
    /// This conversion flattens the structure for lookups
    fn from(value: JointProgress) -> Self {
        let data: HashSet<Arc<Joint>> = HashSet::new();
        value
            .consume()
            .into_values()
            .flat_map(|x| x.into_values())
            .flat_map(|x| x.into_values())
            .for_each(|x| data.insert(x));
        JointSet {
            data: RwLock(data),
        }
    }
}
