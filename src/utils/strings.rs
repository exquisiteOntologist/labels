pub fn str_first_char(word: &str) -> char {
    word.char_indices().nth(0).unwrap().1
}

pub fn str_last_char(word: &str) -> char {
    word.char_indices().nth_back(0).unwrap().1
}

/// Get the real length of a str.
/// It handles where letter is comprised of multiple characters,
/// or has greater complexity.
pub fn str_get_len<'a>(s: &'a str) -> usize {
    s.char_indices().count()
}

pub fn utf8_slice_old<'a>(string: &'a str, start: usize, end: usize) -> &'a str {
    // println!("{:?}", string.chars());
    // println!("s {:1} e {:2}", start, end);
    let start_pos = string.char_indices().nth(start);
    let end_pos = string.char_indices().nth(end);

    if start_pos.is_none() || end_pos.is_none() || end < start {
        eprintln!(
            "Could not get char indice for \"{:1}\" {:2} {:3}",
            string, start, end
        );
        return string;
    }

    // println!("sp {:1} ep {:2}", start_pos, end_pos);
    &string[start_pos.unwrap().0..end_pos.unwrap().0].trim()
}

/// Slice a string slice from start to end;
/// Even when the string has characters that are more than a single byte in size.
/// https://stackoverflow.com/questions/51982999/slice-a-string-containing-unicode-chars
pub fn utf8_slice_stack_overflow<'a>(string: &'a str, start: usize, end: usize) -> Option<&'a str> {
    // assert!(end >= start);
    // string.char_indices().nth(start).and_then(|(start_pos, _)| {
    //     string[str_size_utf8(string, 0, start_pos)..]
    //         .char_indices()
    //         .nth(end - start - 1)
    //         .map(|(end_pos, _)| {
    //             &string[str_size_utf8(string, 0, start_pos)..str_size_utf8(string, 0, end_pos)]
    //         })
    // })
    assert!(end >= start);
    string.char_indices().nth(start).and_then(|(start_pos, _)| {
        string[start_pos..]
            .char_indices()
            .nth(end - start - 1)
            .map(|(end_pos, _)| &string[start_pos..end_pos])
    })
}

/// Get the UTF-8 byte length of string.
/// Characters can be more than a single byte in size.
pub fn str_size_utf8(s: &str, start: usize, end: usize) -> usize {
    let mut size: usize = 0;
    let mut index: usize = 0;
    for (_, char) in s.char_indices() {
        if index < start || index > end {
            continue;
        }
        size += char.len_utf8();
        index += 1;
    }
    size
}

pub fn str_last_char_size(s: &str) -> usize {
    s.char_indices().nth_back(1).unwrap().1.len_utf8()
}
