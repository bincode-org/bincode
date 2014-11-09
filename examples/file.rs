extern crate bincode;
extern crate serialize;

use std::collections::HashMap;
use std::io::{Truncate, ReadWrite, File, BufferedReader};

fn main() {
    let mut word_counts = HashMap::new();
    word_counts.insert("foo".to_string(), 3u);
    word_counts.insert("fizzbuzz".to_string(), 8u);

    let file = File::open_mode(&Path::new("store.bin"), Truncate, ReadWrite);
    let mut file = file.unwrap();
    bincode::encode_into(&word_counts, &mut file).unwrap();
    file.fsync().unwrap();

    let out: HashMap<String, uint> =
        bincode::decode_from(&mut BufferedReader::new(file)).unwrap();

    assert!(out == word_counts);
}
