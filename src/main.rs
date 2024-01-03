use std::error::Error;
use std::io;

use labels::samples::articles::SAMPLE_ARTICLE;
use labels::scoring::tally::{tally_intersecting_phrases, tally_phrases};
use labels::text::extraction::words_extraction;

fn main() -> io::Result<()> {
    _ = my_basic_experiment();

    Ok(())
}

fn my_basic_experiment() -> Result<(), Box<dyn Error>> {
    let mut words_wanted: Vec<Vec<&str>> = words_extraction(&SAMPLE_ARTICLE);

    words_wanted.sort();

    // for word in words_wanted.clone() {
    //     // println!("word: {:?}", word);
    // }

    // count similars
    let word_tallies: Vec<(Vec<&str>, i32)> = tally_phrases(&words_wanted);

    // change sorting from alphabetical to tally score
    // word_tallies.sort_by(|(_, a), (_, b)| a.cmp(b));

    for (word, tally) in &word_tallies {
        println!(
            "tally: {:1} {:2}",
            word.join(" ").to_string(),
            tally.to_string()
        );
    }

    let new_tallies = tally_intersecting_phrases(word_tallies);

    for (word, tally) in new_tallies {
        println!(
            "tally and intersections: {:1} {:2}",
            word.join(" ").to_string(),
            tally.to_string()
        );
    }

    // TODO: Next comes scoring for intersecting words;
    // words that contain other words' characters.

    Ok(())
}
