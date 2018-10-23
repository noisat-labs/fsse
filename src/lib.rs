#![feature(int_to_from_bytes)]


pub mod simhash;

use std::collections::{ BTreeMap, HashSet };
use crate::simhash::{ simhash, hamming_distance };


pub fn build_index(hk: u128, messages: &str) -> BTreeMap<u128, HashSet<usize>> {
    messages.lines()
        .enumerate()
        .filter(|(_, msg)| !msg.trim().is_empty())
        .flat_map(|(i, msg)| msg.split_whitespace().map(move |w| (i, w)))
        .map(|(i, msg)| (simhash(hk, msg.trim()), i))
        .fold(BTreeMap::new(), |mut sum, (hash, i)| {
            sum.entry(hash)
                .or_insert(HashSet::new())
                .insert(i);
            sum
        })
}

pub fn trapdoor(hk: u128, w: &str) -> u128 {
    simhash(hk, w)
}

pub fn search(map: &BTreeMap<u128, HashSet<usize>>, t: u128) -> HashSet<usize> {
    map.iter()
        .filter(|(&k, _)| hamming_distance(k, t) <= 32)
        .flat_map(|(_, v)| v.iter().cloned())
        .collect()
}


#[test]
fn it_work() {
    let text = r#"
        A runtime for writing reliable, asynchronous, and slim applications with the Rust programming language. It is:
        Fast: Tokio's zero-cost abstractions give you bare-metal performance.
        Reliable: Tokio leverages Rust's ownership, type system, and concurrency model to reduce bugs and ensure thread safety.
        Scalable: Tokio has a minimal footprint, and handles backpressure and cancellation naturally.
    "#;

    let db = text.trim().lines().map(ToOwned::to_owned).collect::<Vec<_>>();

    let key = rand::random();
    let map = build_index(key, text.trim());

    let t = trapdoor(key, "Tokio");
    let set = search(&map, t);
    let flag = set.iter()
        .any(|&i| db[i].find("Tokio").is_some());
    assert!(!set.is_empty());
    assert!(flag);

    let t = trapdoor(key, "tokio");
    let set = search(&map, t);
    let flag = set.iter()
        .any(|&i| db[i].find("Tokio").is_some());
    assert!(!set.is_empty());
    assert!(flag);

    let t = trapdoor(key, "asyncio");
    let set = search(&map, t);
    assert!(set.is_empty());
}
