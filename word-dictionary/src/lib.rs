/*!
# Word Dictionary

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
*/

use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::path::PathBuf;

mod errors;

use trim_in_place::TrimInPlace;

pub use errors::*;

#[derive(Debug)]
pub struct Dictionary {
    /// The path of the dictionary file.
    path: PathBuf,
    /// Left data.
    left: Vec<String>,
    /// Right data.
    right: Vec<Vec<String>>,
}

impl Dictionary {
    /// Create a new `Dictionary` instance. But not read the file data. Use the `read_data` method to read data file the input file.
    #[inline]
    pub fn new<P: Into<PathBuf>>(path: P) -> Dictionary {
        Dictionary {
            path: path.into(),
            left: Vec::new(),
            right: Vec::new(),
        }
    }
}

impl Dictionary {
    /// Get the count of words.
    #[inline]
    pub fn count(&self) -> usize {
        debug_assert_eq!(self.left.len(), self.right.len());

        self.left.len()
    }

    /// Get the all right words.
    #[inline]
    pub fn get_all_right(&self, index: usize) -> Option<&[String]> {
        self.right.get(index).map(|v| v.as_slice())
    }

    /// Get the all right words.
    #[inline]
    pub fn get_all_right_to_string(&self, index: usize) -> Option<String> {
        self.right.get(index).map(|v| v.join(" --> "))
    }

    /// Get the last right word at a specific index.
    #[inline]
    pub fn get_right(&self, index: usize) -> Option<&str> {
        match self.right.get(index) {
            Some(v) => v.last().map(|s| s.as_str()),
            None => None,
        }
    }

    /// Get the left word at a specific index
    #[inline]
    pub fn get_left(&self, index: usize) -> Option<&str> {
        self.left.get(index).map(|s| s.as_str())
    }
}

impl Dictionary {
    /// Find a word by a keyword.
    #[inline]
    pub fn find_left_strictly<S: AsRef<str>>(&self, s: S, mut start_index: usize) -> Option<usize> {
        let size = self.count();

        if size == 0 {
            return None;
        }

        start_index %= size;

        let s = s.as_ref();

        for _ in 0..size {
            let tmp = &self.left[start_index];

            if tmp.eq_ignore_ascii_case(s) {
                return Some(start_index);
            }

            start_index += 1;

            if start_index == size {
                start_index = 0;
            }
        }

        None
    }

    /// Find a word by a keyword.
    #[inline]
    pub fn find_left<S: AsRef<str>>(&self, s: S, mut start_index: usize) -> Option<usize> {
        let size = self.count();

        if size == 0 {
            return None;
        }

        start_index %= size;

        let s = s.as_ref();

        let s_upper_case = s.to_uppercase();
        let s_lower_case = s.to_lowercase();

        for _ in 0..size {
            let tmp = &self.left[start_index];

            let tmp_upper_case = tmp.to_uppercase();

            if tmp_upper_case.contains(&s_upper_case) {
                return Some(start_index);
            }

            let tmp_lower_case = tmp.to_lowercase();

            if tmp_lower_case.contains(&s_lower_case) {
                return Some(start_index);
            }

            start_index += 1;

            if start_index == size {
                start_index = 0;
            }
        }

        None
    }

    /// Find a word by a keyword.
    #[inline]
    pub fn find_right_strictly<S: AsRef<str>>(
        &self,
        s: S,
        mut start_index: usize,
    ) -> Option<usize> {
        let size = self.count();

        if size == 0 {
            return None;
        }

        start_index %= size;

        let s = s.as_ref();

        for _ in 0..size {
            for tmp in self.right[start_index].iter().rev() {
                if tmp.eq_ignore_ascii_case(s) {
                    return Some(start_index);
                }
            }

            start_index += 1;

            if start_index == size {
                start_index = 0;
            }
        }

        None
    }

    /// Find a word by a keyword.
    #[inline]
    pub fn find_right<S: AsRef<str>>(&self, s: S, mut start_index: usize) -> Option<usize> {
        let size = self.count();

        if size == 0 {
            return None;
        }

        start_index %= size;

        let s = s.as_ref();

        let s_upper_case = s.to_uppercase();
        let s_lower_case = s.to_lowercase();

        for _ in 0..size {
            for tmp in self.right[start_index].iter().rev() {
                let tmp_upper_case = tmp.to_uppercase();

                if tmp_upper_case.contains(&s_upper_case) {
                    return Some(start_index);
                }

                let tmp_lower_case = tmp.to_lowercase();

                if tmp_lower_case.contains(&s_lower_case) {
                    return Some(start_index);
                }
            }

            start_index += 1;

            if start_index == size {
                start_index = 0;
            }
        }

        None
    }
}

impl Dictionary {
    /// Read the dictionary from the dictionary file.
    pub fn read_data(&mut self) -> Result<(), ReadError> {
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                // it is okay with a file not found error
                return Ok(());
            }
            Err(err) => return Err(err.into()),
        };

        let mut reader = BufReader::new(file);

        let mut buffer = String::new();

        let mut line_counter = 1;

        loop {
            buffer.clear();

            let c = reader.read_line(&mut buffer)?;

            if c == 0 {
                break;
            }

            buffer.trim_in_place();

            if buffer.is_empty() {
                continue;
            }

            let mut tokenizer = buffer.split('=');

            let left_string = tokenizer.next().unwrap();

            if left_string.contains("-->") {
                return Err(ReadError::Broken {
                    line: line_counter,
                    left_string: String::from(left_string),
                    reason: BrokenReason::BadLeftString,
                });
            }

            let left_string = left_string.trim_end();

            // the format of the left string has been checked

            if let Some(index) = self.find_left_strictly(left_string, 0) {
                return Err(ReadError::Broken {
                    line: line_counter,
                    left_string: String::from(left_string),
                    reason: BrokenReason::Duplicated {
                        another_left_string: String::from(self.left[index].as_str()),
                    },
                });
            }

            let right_string = match tokenizer.next() {
                Some(right_string) => right_string,
                None => {
                    return Err(ReadError::Broken {
                        line: line_counter,
                        left_string: String::from(left_string),
                        reason: BrokenReason::NoRightString,
                    })
                }
            };

            if tokenizer.next().is_some() {
                return Err(ReadError::Broken {
                    line: line_counter,
                    left_string: String::from(left_string),
                    reason: BrokenReason::BadRightString {
                        right_string: String::from(right_string),
                    },
                });
            }

            let mut right_strings: Vec<String> = Vec::with_capacity(1);

            for s in right_string.split("-->").map(|s| s.trim()) {
                if s.is_empty() {
                    return Err(ReadError::Broken {
                        line: line_counter,
                        left_string: String::from(left_string),
                        reason: BrokenReason::BadRightString {
                            right_string: String::from(right_string),
                        },
                    });
                }

                right_strings.push(String::from(s));
            }

            self.left.push(String::from(left_string));
            self.right.push(right_strings);

            line_counter += 1;
        }

        Ok(())
    }
}

impl Dictionary {
    /// Write this dictionary to its dictionary file.
    pub fn write_data(&mut self) -> Result<(), WriteError> {
        let mut file = File::create(&self.path)?;

        let size = self.count();

        if size > 0 {
            let size_dec = size - 1;

            // When doing exchange sort, it also writes data to file.
            for i in 0..size_dec {
                let mut left = self.left[i].to_uppercase();

                for j in (i + 1)..size {
                    let left_2 = self.left[j].to_uppercase();

                    if left > left_2 {
                        self.left.swap(i, j);

                        self.right.swap(i, j);

                        left = left_2;
                    }
                }

                writeln!(file, "{} = {}", self.left[i], self.right[i].join(" --> "))?;
            }

            write!(file, "{} = {}", self.left[size_dec], self.right[size_dec].join(" --> "))?;
        }

        Ok(())
    }

    /// Delete a word.
    #[inline]
    pub fn delete(&mut self, index: usize) -> Result<bool, WriteError> {
        if index < self.count() {
            self.left.remove(index);
            self.right.remove(index);

            self.write_data()?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Add or edit a word. If the left word exists, then update it, and return `Ok(false)`.
    pub fn add_edit<L: AsRef<str>, R: AsRef<str>>(
        &mut self,
        left: L,
        right: R,
    ) -> Result<bool, WriteError> {
        let left = left.as_ref().trim();
        let right = right.as_ref().trim();

        if left.contains("-->") || left.contains('=') {
            Err(WriteError::BadLeftString)
        } else if right.contains("-->") || right.contains('=') {
            Err(WriteError::BadRightString)
        } else if left == right {
            Err(WriteError::Same)
        } else if let Some(index) = self.find_left_strictly(left, 0) {
            if self.get_right(index).unwrap() == right {
                Err(WriteError::Duplicated)
            } else {
                self.right.get_mut(index).unwrap().push(String::from(right));

                self.write_data()?;

                Ok(false)
            }
        } else {
            self.left.push(String::from(left));
            self.right.push(vec![String::from(right)]);

            self.write_data()?;

            Ok(true)
        }
    }
}
