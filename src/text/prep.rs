use std::error::Error;

use crate::utils::strings::{str_first_char, str_get_len, str_last_char, utf8_slice_mine};

/// Cleans punctuation from the beginning and end of a word str.
/// It is able to handle characters of non-conforming byte sizes.
/// It doesn't panic.
pub fn clean_word<'a>(word: &'a str) -> Result<&'a str, Box<dyn Error>> {
    let mut word_clean: &str = word;
    while !word_clean.is_empty() && str_first_char(&word_clean).is_alphanumeric() == false {
        let word_len = str_get_len(word_clean);
        // word_clean = utf8_slice(word_clean, 1, word_clean.len() - 1).unwrap_or("");
        word_clean = utf8_slice_mine(word_clean, 1, word_len - 1);
    }
    while !word_clean.is_empty() && str_last_char(word_clean).is_alphanumeric() == false {
        // let last_byte_size = str_last_char_size(word_clean);
        let word_len = str_get_len(word_clean);
        // println!(
        //     "word length {:1} and {:2} and {:3}",
        //     word_clean, word_len, last_byte_size
        // );
        // word_clean = utf8_slice(word_clean, 0, word_len - 1).unwrap_or("");
        word_clean = utf8_slice_mine(word_clean, 0, word_len - 1);
    }
    println!("{:?}", word_clean);
    Ok(word_clean)
}
