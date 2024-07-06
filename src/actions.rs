use crate::{
    scoring::tally::{tally_phrases, tally_phrases_and_intersections, PhraseTally},
    text::extraction::phrase_extraction,
};

pub fn collect_word_tallies(article: &str) -> Vec<PhraseTally> {
    let words_wanted: Vec<Vec<&str>> = phrase_extraction(article);
    let word_tallies: Vec<PhraseTally> = tally_phrases(&words_wanted);

    word_tallies
}

/// Add to word tallies a magic score based on their intersections
pub fn collect_word_tallies_with_intersections<'a>(article: &'a str) -> Vec<PhraseTally<'a>> {
    let words_wanted: Vec<Vec<&'a str>> = phrase_extraction(article);
    let tallies_inc_intersections: Vec<PhraseTally<'a>> =
        tally_phrases_and_intersections(&words_wanted);

    tallies_inc_intersections
}

pub fn collect_word_tallies_with_intersections_sorted_score(article: &str) -> Vec<PhraseTally> {
    let mut tallies_inc_intersections = collect_word_tallies_with_intersections(article);

    tallies_inc_intersections.sort_by(|a, b| b.total.cmp(&a.total));

    tallies_inc_intersections
}
