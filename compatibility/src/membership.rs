use std::collections::BTreeSet;
use rand::{prelude::ThreadRng, Rng};

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
            learners: random_btreeset(&mut rng, 1000),
            configs: vec_random_btreeset(&mut rng,),
            all_members: random_btreeset(&mut rng, 1000),
        });
    }
}

fn vec_random_btreeset(rng: &mut ThreadRng) -> Vec<BTreeSet<NodeId>> {
  let mut vec = Vec::with_capacity(5);
  for _ in 0..10 {
    vec.push(random_btreeset(rng, 100));
  }
  vec
}

fn random_btreeset(rng: &mut ThreadRng, count: u32) -> BTreeSet<NodeId> {
  let mut set = BTreeSet::new();
  for _ in 0..count {
    let v = rng.gen();
    set.insert(v);
  }
  set
}