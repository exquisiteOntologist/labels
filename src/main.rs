use std::error::Error;
use std::io;

use labels::samples::articles::{SAMPLE_ARTICLE_GENETICS, SAMPLE_ARTICLE_PANTHERS};
use labels::scoring::tally::{tally_intersecting_phrases, tally_phrases};
use labels::text::extraction::phrase_extraction;

fn main() -> io::Result<()> {
    _ = my_basic_experiment(SAMPLE_ARTICLE_GENETICS);
    _ = my_basic_experiment(SAMPLE_ARTICLE_PANTHERS);

    Ok(())
}

#[cfg(test)]
mod tests {
    use labels::samples::articles::SAMPLE_ARTICLE_PANTHERS;

    use super::*;

    #[test]
    fn test_tally() {
        _ = my_basic_experiment(SAMPLE_ARTICLE_PANTHERS);
    }
}

const MAX_LABELS: usize = 30;

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

    let max = if MAX_LABELS > tallies_inc_intersections.len() {
        tallies_inc_intersections.len()
    } else {
        MAX_LABELS
    };

    println!("phrases count {:?}", tallies_inc_intersections.len());

    for i in 0..max {
        let (word, tally) = &tallies_inc_intersections[i];
        if word.is_empty() {
            println!("Identified whitespace - - - !!!");
            // yep, it's not whitespace but instead a word of 0 length
        }
        println!(
            "top phrase: {:1} \"{:2}\" {:3}",
            i,
            word.join(" ").to_string(),
            tally.to_string()
        );
    }

    Ok(())
}
