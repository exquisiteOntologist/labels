use std::error::Error;
use std::io;

use labels::samples::articles::SAMPLE_ARTICLE;
use labels::scoring::tally::{tally_intersecting_phrases, tally_phrases};
use labels::text::extraction::phrase_extraction;

fn main() -> io::Result<()> {
    _ = my_basic_experiment(SAMPLE_ARTICLE);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tally() {
        _ = my_basic_experiment(SAMPLE_ARTICLE);
    }
}

fn my_basic_experiment(article: &str) -> Result<(), Box<dyn Error>> {
    let words_wanted: Vec<Vec<&str>> = phrase_extraction(article);

    // count similars
    let word_tallies: Vec<(Vec<&str>, i32)> = tally_phrases(&words_wanted);

    // change sorting from alphabetical to tally score
    // word_tallies.sort_by(|(_, a), (_, b)| a.cmp(b));

    let mut tallies_inc_intersections = tally_intersecting_phrases(&word_tallies);

    for (word, tally) in &tallies_inc_intersections {
        println!(
            "tally and intersections: {:1} {:2}",
            word.join(" ").to_string(),
            tally.to_string()
        );
    }

    tallies_inc_intersections.sort_by(|(_, a), (_, b)| b.cmp(a));

    for i in 0..10 {
        let (word, tally) = &tallies_inc_intersections[i];
        println!(
            "top phrase: {:1} {:2} {:3}",
            i,
            word.join(" ").to_string(),
            tally.to_string()
        );
    }

    // TODO: Next comes scoring for intersecting words;
    // words that contain other words' characters.

    Ok(())
}
