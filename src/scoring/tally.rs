use crate::utils::vecs::vec_eq;

/// Count instances of a phrase. Requires the words to vec passed in to be sorted alphabetical.
/// Starts at 1 for each phrase since each is an instance.
pub fn tally_phrases<'a>(words: &Vec<Vec<&'a str>>) -> Vec<(Vec<&'a str>, i32)> {
    let mut word_tallies: Vec<(Vec<&str>, i32)> = vec![];
    let mut prior_word: Vec<&str> = vec![];
    let mut tally: i32 = 1;

    if words.is_empty() {
        return word_tallies;
    }

    for word in words {
        // Doesn't handle last word in text
        if vec_eq(&word, &prior_word) {
            tally += 1;
        } else {
            word_tallies.push((prior_word, tally));
            tally = 1;
        }
        prior_word = word.to_vec();
    }
    word_tallies.push((prior_word, 1));
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
    phrase_tallies: &Vec<(Vec<&'a str>, i32)>,
) -> Vec<(Vec<&'a str>, i32)> {
    if phrase_tallies.len() < 2 {
        return phrase_tallies.to_vec();
    }

    let mut new_phrase_tallies: Vec<(Vec<&'a str>, i32)> = Default::default();

    // each tally represents a single term
    for i in 1..phrase_tallies.len() {
        let (prior_phrase, prior_tally) = if i == 1 {
            &phrase_tallies[i - 1]
        } else {
            &new_phrase_tallies[i - 1]
        };
        let (phrase, mut tally) = &phrase_tallies[i];

        // because of our alphabetical sorting, the prior is always shorter
        // (this variable is redundant)
        // let prior_longer: bool = false;
        let shorter_len: usize = prior_phrase.len();
        // let prior_longer: bool = prior_phrase.len() > phrase.len();
        // let shorter_len: usize = if prior_longer {
        //     phrase.len()
        // } else {
        //     prior_phrase.len()
        // };

        // here we are iterating through the words of the
        // current and prior phrases,
        // checking to see if the shorter phrase is 100% within
        // the longer phrase.
        let mut shorter_fully_in_longer: bool = true;
        for i in 0..shorter_len {
            let pp_word = prior_phrase[i];
            let word = phrase[i];

            if pp_word != word {
                shorter_fully_in_longer = false;
                break;
            }
        }

        if phrase.len() == 1 {
            let mut is_acronym: bool = true;
            'p_chars: for c in phrase[0].chars() {
                if c.is_uppercase() == false {
                    is_acronym = false;
                    break 'p_chars;
                }
            }
            if is_acronym {
                tally *= 2;
            }
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
            tally += prior_tally;
        }

        // since we start at one we also have to add the prior to the list (pos 0)
        if i == 1 {
            new_phrase_tallies.push((prior_phrase.to_vec(), *prior_tally));
        }
        new_phrase_tallies.push((phrase.to_vec(), tally));
    }
    new_phrase_tallies
}
