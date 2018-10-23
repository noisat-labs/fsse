use std::hash::Hash;
use std::ops::BitOr;
use siphasher::sip128::{ Hasher128, SipHasher };
use itertools::Itertools;


pub fn simhash_iter<'w, W, T>(key: u128, words: W) -> u128
where
    W: Iterator<Item = T>,
    T: Hash
{
    let mut state = [0i32; 128];
    let hasher = SipHasher::new_with_keys((key >> 64) as u64, key as u64);

    for word in words {
        let mut hasher = hasher.clone();
        word.hash(&mut hasher);
        let hash = hasher.finish128();
        let hash = u128::from_le_bytes(hash.as_bytes());

        for i in 0..128 {
            let bit = (hash >> i) & 1;
            state[i] =
                if bit == 1 {
                    state[i].saturating_add(1)
                } else {
                    state[i].saturating_sub(1)
                };
        }
    }

    state.iter()
        .enumerate()
        .filter(|(_, &v)| v > 0)
        .map(|(i, _)| 1 << i)
        .fold(0, BitOr::bitor)
}

#[derive(Hash)]
enum Word {
    Start(char),
    Middle(char, char),
    End(char)
}

pub fn simhash(key: u128, word: &str) -> u128 {
    let start = word.chars()
        .next()
        .map(Word::Start);
    let end = word.chars()
        .last()
        .map(Word::End);
    let iter = word.chars()
        .tuple_windows::<(char, char)>()
        .map(|(x, y)| Word::Middle(x, y));
    simhash_iter(key, iter.chain(start).chain(end))
}

pub fn hamming_distance(x: u128, y: u128) -> u32 {
    (x ^ y).count_ones()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simhash_english() {
        let key = 0x42;

        let hash = simhash(key, "hello");
        let hash2 = simhash(key, "hallo");
        eprintln!("{:?}", hamming_distance(hash, hash2));
        assert!(hamming_distance(hash, hash2) <= 32);

        let hash2 = simhash(key, "simhash");
        assert!(hamming_distance(hash, hash2) > 32);
    }
}
