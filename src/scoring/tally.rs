use crate::utils::vecs::vec_eq;

pub struct PhraseTally<'a> {
    /// A phrase, consisting of references to word strings in the source text
    pub phrase: Vec<&'a str>,
    /// The number of occurrences of the phrase
    pub total: i32,
}

impl<'a> Clone for PhraseTally<'a> {
    fn clone(&self) -> PhraseTally<'a> {
        PhraseTally {
            phrase: self.phrase.to_vec(),
            total: self.total,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

pub fn tally_phrases_and_intersections<'a>(phrases: &Vec<Vec<&'a str>>) -> Vec<PhraseTally<'a>> {
    let word_tallies: Vec<PhraseTally<'a>> = tally_phrases(&phrases);
    let tallies_inc_intersections: Vec<PhraseTally<'a>> = tally_intersecting_phrases(word_tallies);

    tallies_inc_intersections
}

/// Count instances of a phrase. Requires the words to vec passed in to be sorted alphabetical.
/// Starts at 1 for each phrase since each is an instance.
pub fn tally_phrases<'a>(words: &Vec<Vec<&'a str>>) -> Vec<PhraseTally<'a>> {
    // pub fn tally_phrases<'a>(words: &Vec<Vec<&'a str>>) -> Vec<(Vec<&'a str>, i32)> {
    // let mut word_tallies: Vec<(Vec<&str>, i32)> = vec![];
    let mut word_tallies: Vec<PhraseTally> = Vec::new();
    let mut prior_word: Vec<&str> = Vec::new();
    let mut tally: i32 = 1;

    if words.is_empty() {
        return word_tallies;
    }

    for word in words {
        // Doesn't handle last word in text
        if vec_eq(&word, &prior_word) {
            tally += 1;
        } else {
            word_tallies.push(PhraseTally {
                phrase: prior_word,
                total: tally,
            });
            tally = 1;
        }
        prior_word = word.to_vec();
    }
    word_tallies.push(PhraseTally {
        phrase: prior_word,
        total: 1,
    });
    word_tallies
}

/// Takes an existing phrase tally and then adds to it:
/// where each occurence of that phrase exists in other tallied phrases
/// It is assumed that each input tallied phrase is distinct.
/// Current it only compares the neighbouring phrases in our list.
/// & it starts comparing from the start, so it misses 1st names.
/// TODO: If more than 2 neighbouring words overlap, the current system
/// does not include the tally of the 2nd word in the subsequent words;
/// because it's still iterating over the original vector.
/// TODO: Handle plurals scoring or pre-cleaning.
pub fn tally_intersecting_phrases<'a>(
    phrase_tallies: Vec<PhraseTally<'a>>,
) -> Vec<PhraseTally<'a>> {
    if phrase_tallies.len() < 2 {
        return phrase_tallies.clone();
    }

    let mut new_phrase_tallies: Vec<PhraseTally<'a>> = Vec::new();

    // each tally represents a single term
    for i in 1..phrase_tallies.len() {
        let pt_prior = if i == 1 {
            &phrase_tallies[i - 1]
        } else {
            &new_phrase_tallies[i - 1]
        };
        let pt = &phrase_tallies[i];
        let mut tally = pt.total;

        // because of our alphabetical sorting, the prior is always shorter
        let shorter_len: usize = pt_prior.phrase.len();

        // here we are iterating through the words of the
        // current and prior phrases,
        // checking to see if the shorter phrase is 100% within
        // the longer phrase.
        let mut shorter_fully_in_longer: bool = true;
        for i in 0..shorter_len {
            let pp_word = pt_prior.phrase[i];
            let word = pt.phrase[i];

            if pp_word != word {
                shorter_fully_in_longer = false;
                break;
            }
        }

        if pt.phrase.len() == 1 {
            let mut is_acronym: bool = true;
            'p_chars: for c in pt.phrase[0].chars() {
                if c.is_uppercase() == false {
                    is_acronym = false;
                    break 'p_chars;
                }
            }
            if is_acronym {
                tally *= 2;
            }
        } else {
            tally *= 2;
        }

        // perhaps the shorter AND the longer word
        // should both gain relevance,
        // not just the longer?
        if shorter_fully_in_longer {
            // if prior_longer {
            //     // points to prior
            //     // this only works for the first iteration
            //     // & it's redundant anyway (see note about ABC sorting)
            //     prior_tally += tally;
            // }
            // } else {
            //     // else current longer, points to longer
            //     tally += prior_tally;
            // }
            // assuming prior phrase is shorter
            tally += pt_prior.total;
        }

        // since we start at one we also have to add the prior to the list (pos 0)
        if i == 1 {
            new_phrase_tallies.push(pt_prior.clone());
        }
        new_phrase_tallies.push(PhraseTally {
            phrase: pt.phrase.to_vec(),
            total: tally,
        });
    }
    new_phrase_tallies
}
