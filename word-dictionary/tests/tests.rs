extern crate word_dictionary;

#[macro_use]
extern crate slash_formatter;

use std::fs;
use std::path::Path;

use word_dictionary::*;

static DIRECTORY_PATH: &str = concat_with_file_separator_build!("tests", "data");

#[test]
fn correct_usage() {
    let dictionary_path = Path::new(DIRECTORY_PATH).join("correct_usage.txt");

    let dictionary_data = "Abez = 阿別茲 -->阿貝茲
  Abhai = 阿拜
ability =技能
Abmin = 阿布明

Abraxas= 阿柏拉克薩斯
Absu = 阿布蘇";

    fs::write(&dictionary_path, dictionary_data).unwrap();

    let mut dictionary = Dictionary::new(&dictionary_path);

    dictionary.read_data().unwrap();

    assert_eq!(6, dictionary.count());

    assert_eq!(Some("Abez"), dictionary.get_left(0));
    assert_eq!(Some("阿貝茲"), dictionary.get_right(0));
    assert_eq!(Some(String::from("阿別茲 --> 阿貝茲")), dictionary.get_all_right_to_string(0));

    assert_eq!(Some(0), dictionary.find_left_strictly("Abez", 0));
    assert_eq!(Some(0), dictionary.find_left("A", 0));
    assert_eq!(Some(2), dictionary.find_left_strictly("aBility", 0));
    assert_eq!(None, dictionary.find_left_strictly("aBilit", 0));
    assert_eq!(Some(2), dictionary.find_left("aBilit", 0));

    assert_eq!(Some(0), dictionary.find_right_strictly("阿貝茲", 0));
    assert_eq!(Some(0), dictionary.find_right_strictly("阿別茲", 0));
    assert_eq!(None, dictionary.find_right_strictly("阿貝", 0));
    assert_eq!(Some(0), dictionary.find_right("阿貝", 0));
    assert_eq!(Some(4), dictionary.find_right_strictly("阿柏拉克薩斯", 0));
    assert_eq!(Some(4), dictionary.find_right("柏拉克", 0));

    assert_eq!(Some(1), dictionary.find_left("A", 1));
    assert_eq!(Some(1), dictionary.find_right("阿", 1));

    assert_eq!(true, dictionary.add_edit("Alric", "阿里克").unwrap());
    assert_eq!(7, dictionary.count());
    assert_eq!(Some("阿里克"), dictionary.get_right(6));

    assert_eq!(false, dictionary.add_edit("Abez", "阿別茲").unwrap());
    assert_eq!(7, dictionary.count());
    assert_eq!(Some("阿別茲"), dictionary.get_right(0));

    assert_eq!(true, dictionary.delete(4).unwrap());

    let dictionary_data = "Abez = 阿別茲 --> 阿貝茲 --> 阿別茲
Abhai = 阿拜
ability = 技能
Abmin = 阿布明
Absu = 阿布蘇
Alric = 阿里克";

    assert_eq!(dictionary_data, fs::read_to_string(&dictionary_path).unwrap());
}

#[test]
fn incorrect_usage() {
    let dictionary_path = Path::new(DIRECTORY_PATH).join("incorrect_usage.txt");

    let dictionary_data = "Abez = 阿別茲 -->";

    fs::write(&dictionary_path, dictionary_data).unwrap();

    let mut dictionary = Dictionary::new(&dictionary_path);

    assert!(dictionary.read_data().is_err());

    let dictionary_data = "Abez = 阿別=茲";

    fs::write(&dictionary_path, dictionary_data).unwrap();

    let mut dictionary = Dictionary::new(&dictionary_path);

    assert!(dictionary.read_data().is_err());

    let dictionary_data = "Abe-->z = 阿別茲";

    fs::write(&dictionary_path, dictionary_data).unwrap();

    let mut dictionary = Dictionary::new(&dictionary_path);

    assert!(dictionary.read_data().is_err());

    let dictionary_data = "Abez = 阿別茲 --> 阿貝茲
Abhai = 阿拜";

    fs::write(&dictionary_path, dictionary_data).unwrap();

    let mut dictionary = Dictionary::new(&dictionary_path);

    dictionary.read_data().unwrap();

    assert!(dictionary.add_edit("Abez", "阿貝茲").is_err());
    assert_eq!(false, dictionary.delete(2).unwrap());
}
