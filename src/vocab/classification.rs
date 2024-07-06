use std::slice::Iter;

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

pub const CLASSIFICATION_LABELS: &'static [(CLASSIFICATION, &str)] = &[
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

pub const WORD_COMMERCE_CLASSIFICATIONS: &'static [(&str, CLASSIFICATION)] = &[
    ("bank", CLASSIFICATION::Markets),
    ("banks", CLASSIFICATION::Markets),
    ("bonds", CLASSIFICATION::Markets),
    ("bought", CLASSIFICATION::Markets),
    ("brokerage", CLASSIFICATION::Markets),
    ("business", CLASSIFICATION::Markets),
    ("cash", CLASSIFICATION::Markets),
    ("coins", CLASSIFICATION::Markets),
    ("commodities", CLASSIFICATION::Markets),
    ("commodity", CLASSIFICATION::Markets),
    ("deed", CLASSIFICATION::Markets),
    ("deeds", CLASSIFICATION::Markets),
    ("dollars", CLASSIFICATION::Markets),
    ("euro", CLASSIFICATION::Markets),
    ("euros", CLASSIFICATION::Markets),
    ("guarantees", CLASSIFICATION::Markets),
    ("market", CLASSIFICATION::Markets),
    ("markets", CLASSIFICATION::Markets),
    ("owner", CLASSIFICATION::Markets),
    ("product", CLASSIFICATION::Markets),
    ("property", CLASSIFICATION::Markets),
    ("rouble", CLASSIFICATION::Markets),
    ("roubles", CLASSIFICATION::Markets),
    ("shares", CLASSIFICATION::Markets),
    ("shipments", CLASSIFICATION::Markets),
    ("sold", CLASSIFICATION::Markets),
    ("stocks", CLASSIFICATION::Markets),
    ("valuable", CLASSIFICATION::Markets),
];

pub const WORD_CREATIVE_ARTS_CLASSIFICATIONS: &'static [(&str, CLASSIFICATION)] = &[
    ("artwork", CLASSIFICATION::Art),
    ("dance", CLASSIFICATION::Art),
    ("dancer", CLASSIFICATION::Art),
    ("design", CLASSIFICATION::Creative),
    ("designer", CLASSIFICATION::Creative),
    ("performer", CLASSIFICATION::Art),
    ("singing", CLASSIFICATION::Art),
    ("song", CLASSIFICATION::Art),
];

pub const WORD_LIFE_CLASSIFICATIONS: &'static [(&str, CLASSIFICATION)] = &[
    ("birth", CLASSIFICATION::Life),
    ("divorce", CLASSIFICATION::Life),
    ("marriage", CLASSIFICATION::Life),
    ("sex", CLASSIFICATION::Life),
];

pub const WORD_MEDIA_CLASSIFICATIONS: &'static [(&str, CLASSIFICATION)] = &[
    ("book", CLASSIFICATION::Media),
    ("movie", CLASSIFICATION::Media),
    ("music", CLASSIFICATION::Media),
    ("television", CLASSIFICATION::Media),
];

pub const WORD_MEDICINE_CLASSIFICATIONS: &'static [(&str, CLASSIFICATION)] = &[
    ("pandemic", CLASSIFICATION::Media),
    ("vaccination", CLASSIFICATION::Media),
];

pub const WORD_NEWS_POLITICS_CLASSIFICATIONS: &'static [(&str, CLASSIFICATION)] = &[
    ("genocide", CLASSIFICATION::Crime),
    ("murder", CLASSIFICATION::Crime),
    ("murderer", CLASSIFICATION::Crime),
    ("ruling", CLASSIFICATION::Politics),
    ("stealing", CLASSIFICATION::Law),
    ("stoned", CLASSIFICATION::Crime),
];

pub const WORD_SPORT_CLASSIFICATIONS: &'static [(&str, CLASSIFICATION)] = &[
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
/// Could be used directly for comparison or inserted into a DB.
pub fn all_classifications() -> Vec<&'static (&'static str, CLASSIFICATION)> {
    let groups = vec![
        WORD_COMMERCE_CLASSIFICATIONS,
        WORD_CREATIVE_ARTS_CLASSIFICATIONS,
        WORD_LIFE_CLASSIFICATIONS,
        WORD_MEDIA_CLASSIFICATIONS,
        WORD_MEDICINE_CLASSIFICATIONS,
        WORD_NEWS_POLITICS_CLASSIFICATIONS,
        WORD_SPORT_CLASSIFICATIONS,
    ];

    let mut classifications: Vec<&'static (&'static str, CLASSIFICATION)> = Vec::new();
    for group in groups {
        classifications.extend::<Iter<(&'static str, CLASSIFICATION)>>(group.iter());
    }

    classifications
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
