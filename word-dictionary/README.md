Word Dictionary
====================

[![CI](https://github.com/magiclen/words-transformer-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/words-transformer-rs/actions/workflows/ci.yml)

This crate provides a data structure for word mapping. It can be used for language translation.

## Examples

```rust
use word_dictionary::Dictionary;

let mut dictionary = Dictionary::new("tests/data/dictionary.txt"); // input a dictionary file

// dictionary.read_data().unwrap(); // if the dictionary file already exists

dictionary.add_edit("Althasol", "阿爾瑟索").unwrap();
dictionary.add_edit("Aldun", "奧爾敦").unwrap();
dictionary.add_edit("Alduin", "阿爾杜因").unwrap();
dictionary.add_edit("Alduin", "奥杜因").unwrap();

assert_eq!("阿爾瑟索", dictionary.get_right(dictionary.find_left_strictly("Althasol", 0).unwrap()).unwrap());
assert_eq!("奧爾敦", dictionary.get_right(dictionary.find_left("dun", 0).unwrap()).unwrap());
assert_eq!("奥杜因", dictionary.get_right(dictionary.find_left("Alduin", 0).unwrap()).unwrap());
assert_eq!("阿爾杜因 --> 奥杜因", dictionary.get_all_right_to_string(dictionary.find_left("Alduin", 0).unwrap()).unwrap());

// The dictionary file now would be
/*
Alduin = 阿爾杜因 --> 奥杜因
Aldun = 奧爾敦
Althasol = 阿爾瑟索
*/
```

## Crates.io

https://crates.io/crates/word-dictionary

## Documentation

https://docs.rs/word-dictionary

## License

[MIT](LICENSE)