// Added by [ppamorim](https://github.com/ppamorim)
// Taken from https://github.com/datafuselabs/openraft/blob/209ae677ade5b624fea9f6630e9ff191963f5d74/openraft/src/membership/membership.rs#L21
// License: Openraft is licensed under the terms of the MIT License or the Apache License 2.0, at your choosing.

use rand::{prelude::ThreadRng, Rng};
use std::collections::BTreeSet;

type NodeId = u64;

#[derive(
    bincode_2::Encode, bincode_2::Decode, serde::Serialize, serde::Deserialize, Debug, PartialEq,
)]
#[bincode(crate = "bincode_2")]
pub struct Membership {
    /// learners set
    learners: BTreeSet<NodeId>,

    /// Multi configs.
    configs: Vec<BTreeSet<NodeId>>,

    /// Cache of all node ids.
    all_members: BTreeSet<NodeId>,
}

#[test]
pub fn test() {
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        crate::test_same(Membership {
            learners: random_btreeset(&mut rng),
            configs: vec_random_btreeset(&mut rng),
            all_members: random_btreeset(&mut rng),
        });
    }
}

fn vec_random_btreeset(rng: &mut ThreadRng) -> Vec<BTreeSet<NodeId>> {
    let mut vec = Vec::with_capacity(10);
    for _ in 0..rng.gen_range(0..10) {
        vec.push(random_btreeset(rng));
    }
    vec
}

fn random_btreeset(rng: &mut ThreadRng) -> BTreeSet<NodeId> {
    let mut set = BTreeSet::new();
    for _ in 0..rng.gen_range(0..100) {
        let v = rng.gen();
        set.insert(v);
    }
    set
}
