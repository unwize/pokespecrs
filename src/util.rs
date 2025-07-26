use std::collections::HashSet;
use rand::Rng;

pub fn sample_hash_set<T: Clone>(collection: &HashSet<T>) -> T {
    let mut rng = rand::rng();
    let index = rng.random_range(0..collection.len());
    collection.iter().nth(index).unwrap().clone()
}