// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {

    let mut map = HashMap::new();
    for word in magazine {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    for word_note in note {
        map.entry(word_note).and_modify(|v| *v -= 1).or_insert(-1);
        if (*map.get(word_note).unwrap() < 0) {
            return false;
        }
    }
    true
}
