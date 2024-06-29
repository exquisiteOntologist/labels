use std::error::Error;
use std::io;

use labels::actions::collect_word_tallies_with_intersections;
use labels::samples::articles::{
    SAMPLE_ARTICLE_GENETICS, SAMPLE_ARTICLE_PANTHERS, SAMPLE_ARTICLE_WIKIPEDIA,
};

fn main() -> io::Result<()> {
    _ = my_basic_experiment(SAMPLE_ARTICLE_GENETICS);
    _ = my_basic_experiment(SAMPLE_ARTICLE_PANTHERS);
    _ = my_basic_experiment(SAMPLE_ARTICLE_WIKIPEDIA);

    Ok(())
}

#[cfg(test)]
mod tests {
    use labels::samples::articles::SAMPLE_ARTICLE_WIKIPEDIA;

    use super::*;

    #[test]
    fn test_tally() {
        _ = my_basic_experiment(SAMPLE_ARTICLE_WIKIPEDIA);
    }
}

fn my_basic_experiment(article: &str) -> Result<(), Box<dyn Error>> {
    let mut tallies_inc_intersections = collect_word_tallies_with_intersections(article);
    // after this we just print all the tallies and then sort by score and print top MAX_LABELS

    for (word, tally) in &tallies_inc_intersections {
        println!(
            "tally and intersections: {:1} {:2}",
            word.join(" ").to_string(),
            tally.to_string()
        );
    }

    tallies_inc_intersections.sort_by(|(_, a), (_, b)| b.cmp(a));

    const MAX_LABELS: usize = 30;
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
