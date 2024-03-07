/// Common classifications.
/// If multiple apply to your content, it is recommended to go by weight.
pub enum CLASSIFICATION {
    Art,
    Creative,
    Crime,
    Life,
    Markets,
    Media,
    LaunderingAndGambling,
    Law,
    Politics,
    Sport,
    War,
}

pub const CLASSIFICATION_LABELS: [(CLASSIFICATION, &str); 11] = [
    (CLASSIFICATION::Art, "Art"),
    (CLASSIFICATION::Creative, "Creative"),
    (CLASSIFICATION::Crime, "Crime"),
    (CLASSIFICATION::Life, "Life"),
    (CLASSIFICATION::Markets, "Markets"),
    (CLASSIFICATION::Media, "Media"),
    (
        CLASSIFICATION::LaunderingAndGambling,
        "Laundering & Gambling",
    ),
    (CLASSIFICATION::Law, "Law"),
    (CLASSIFICATION::Politics, "Politics"),
    (CLASSIFICATION::Sport, "Sport"),
    (CLASSIFICATION::War, "War"),
];

pub const WORD_COMMERCE_CLASSIFICATIONS: [(&str, CLASSIFICATION); 2] = [
    ("business", CLASSIFICATION::Markets),
    ("markets", CLASSIFICATION::Markets),
];

pub const WORD_CREATIVE_ARTS_CLASSIFICATIONS: [(&str, CLASSIFICATION); 8] = [
    ("artwork", CLASSIFICATION::Art),
    ("dance", CLASSIFICATION::Art),
    ("dancer", CLASSIFICATION::Art),
    ("design", CLASSIFICATION::Creative),
    ("designer", CLASSIFICATION::Creative),
    ("performer", CLASSIFICATION::Art),
    ("singing", CLASSIFICATION::Art),
    ("song", CLASSIFICATION::Art),
];

pub const WORD_LIFE_CLASSIFICATIONS: [(&str, CLASSIFICATION); 4] = [
    ("birth", CLASSIFICATION::Life),
    ("divorce", CLASSIFICATION::Life),
    ("marriage", CLASSIFICATION::Life),
    ("sex", CLASSIFICATION::Life),
];

pub const WORD_MEDIA_CLASSIFICATIONS: [(&str, CLASSIFICATION); 4] = [
    ("book", CLASSIFICATION::Media),
    ("movie", CLASSIFICATION::Media),
    ("music", CLASSIFICATION::Media),
    ("television", CLASSIFICATION::Media),
];

pub const WORD_MEDICINE_CLASSIFICATIONS: [(&str, CLASSIFICATION); 2] = [
    ("pandemic", CLASSIFICATION::Media),
    ("vaccination", CLASSIFICATION::Media),
];

pub const WORD_NEWS_POLITICS_CLASSIFICATIONS: [(&str, CLASSIFICATION); 6] = [
    ("genocide", CLASSIFICATION::Crime),
    ("murder", CLASSIFICATION::Crime),
    ("murderer", CLASSIFICATION::Crime),
    ("ruling", CLASSIFICATION::Politics),
    ("stealing", CLASSIFICATION::Law),
    ("stoned", CLASSIFICATION::Crime),
];

pub const WORD_SPORT_CLASSIFICATIONS: [(&str, CLASSIFICATION); 18] = [
    ("AFL", CLASSIFICATION::Sport),
    ("baseball", CLASSIFICATION::Sport),
    ("cricket", CLASSIFICATION::Sport),
    ("professional basketball", CLASSIFICATION::Sport),
    ("football", CLASSIFICATION::Sport),
    ("horse races", CLASSIFICATION::Sport),
    ("horse races", CLASSIFICATION::LaunderingAndGambling),
    ("horse racing", CLASSIFICATION::Sport),
    ("horse racing", CLASSIFICATION::LaunderingAndGambling),
    ("innings", CLASSIFICATION::Sport),
    ("NBA", CLASSIFICATION::Sport),
    ("NRL", CLASSIFICATION::Sport),
    ("olympic swimming", CLASSIFICATION::Sport),
    ("poker", CLASSIFICATION::Sport),
    ("poker", CLASSIFICATION::LaunderingAndGambling),
    ("soccer", CLASSIFICATION::Sport),
    ("softball", CLASSIFICATION::Sport),
    ("sports", CLASSIFICATION::Sport),
];

/// All classifications combined into a single list.
pub fn all_classifications<'a>() -> Vec<(&'a str, CLASSIFICATION)> {
    vec![
        Vec::from(WORD_COMMERCE_CLASSIFICATIONS),
        Vec::from(WORD_CREATIVE_ARTS_CLASSIFICATIONS),
        Vec::from(WORD_LIFE_CLASSIFICATIONS),
        Vec::from(WORD_MEDIA_CLASSIFICATIONS),
        Vec::from(WORD_MEDICINE_CLASSIFICATIONS),
        Vec::from(WORD_NEWS_POLITICS_CLASSIFICATIONS),
        Vec::from(WORD_SPORT_CLASSIFICATIONS),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::all_classifications;

    #[test]
    fn classifications_list_populates() {
        let list = all_classifications();
        assert!(list.is_empty() == false);
    }
}
