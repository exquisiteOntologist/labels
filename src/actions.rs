use crate::{
    scoring::tally::{tally_intersecting_phrases, tally_phrases},
    text::extraction::phrase_extraction,
};

pub fn collect_word_tallies(article: &str) -> Vec<(Vec<&str>, i32)> {
    let words_wanted: Vec<Vec<&str>> = phrase_extraction(article);
    let word_tallies: Vec<(Vec<&str>, i32)> = tally_phrases(&words_wanted);

    word_tallies
}

/// Add to word tallies a magic score based on their intersections
pub fn collect_word_tallies_with_intersections(article: &str) -> Vec<(Vec<&str>, i32)> {
    let words_wanted: Vec<Vec<&str>> = phrase_extraction(article);
    let word_tallies: Vec<(Vec<&str>, i32)> = tally_phrases(&words_wanted);
    let tallies_inc_intersections = tally_intersecting_phrases(&word_tallies);

    tallies_inc_intersections
}

pub fn collect_word_tallies_with_intersections_sorted_score(
    article: &str,
) -> Vec<(Vec<&str>, i32)> {
    let mut tallies_inc_intersections = collect_word_tallies_with_intersections(article);

    tallies_inc_intersections.sort_by(|(_, a), (_, b)| b.cmp(a));

    tallies_inc_intersections
}
