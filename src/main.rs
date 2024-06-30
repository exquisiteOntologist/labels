use std::error::Error;
use std::io;

use labels::actions::collect_word_tallies_with_intersections;
use labels::samples::articles::{
    SAMPLE_ARTICLE_GENETICS, SAMPLE_ARTICLE_PANTHERS, SAMPLE_ARTICLE_WIKIPEDIA,
};
use labels::scoring::tally::PhraseTally;

fn main() -> io::Result<()> {
    _ = my_basic_experiment(SAMPLE_ARTICLE_GENETICS);
    _ = my_basic_experiment(SAMPLE_ARTICLE_PANTHERS);
    _ = my_basic_experiment(SAMPLE_ARTICLE_WIKIPEDIA);

    Ok(())
}

#[cfg(test)]
mod tests {
    use labels::samples::{articles::SAMPLE_ARTICLE_WIKIPEDIA, problems::PROBLEM_A};

    use super::*;

    #[test]
    fn test_tally() {
        _ = my_basic_experiment(SAMPLE_ARTICLE_WIKIPEDIA);
    }

    #[test]
    fn test_problem_a() {
        let results = my_basic_experiment(PROBLEM_A).unwrap();
        let has_problem = results
            .iter()
            .any(|pt| pt.phrase.get(0) == Some(&"epidemic.”Elected"));
        assert_eq!(results.get(0).unwrap().phrase.get(0).unwrap(), &"There");
        assert!(has_problem == false);
    }
}

const MAX_LABELS: usize = 30;

fn my_basic_experiment(article: &str) -> Result<Vec<PhraseTally>, Box<dyn Error>> {
    let mut tallies_inc_intersections = collect_word_tallies_with_intersections(article);
    // after this we just print all the tallies and then sort by score and print top MAX_LABELS

    for pt in &tallies_inc_intersections {
        println!(
            "tally and intersections: {:1} {:2}",
            pt.phrase.join(" "),
            pt.total
        );
    }

    tallies_inc_intersections.sort_by(|a, b| b.total.cmp(&a.total));

    let max = if MAX_LABELS > tallies_inc_intersections.len() {
        tallies_inc_intersections.len()
    } else {
        MAX_LABELS
    };

    println!("phrases count {:?}", tallies_inc_intersections.len());

    for i in 0..max {
        let pt = &tallies_inc_intersections[i];
        if pt.phrase.is_empty() {
            println!("Identified whitespace - - - !!!");
            // yep, it's not whitespace but instead a word of 0 length
        }
        println!(
            "top phrase: {:1} \"{:2}\" {:3}",
            i,
            pt.phrase.join(" "),
            pt.total
        );
    }

    Ok(tallies_inc_intersections)
}
