use std::collections::HashMap;

pub mod types;

const MINIMUM_STEM_LENGTH: u8 = 3;

/// Transforms input string into a vector of tokens, a token being a sequence of only geez letter
/// characters, The only characters taken as valid are those within the unicode geez range i.e.
/// 1200 - 135A, all else are parsed as whitespaces
pub fn tokenize(
    text: &str,
    stopwords: &[String],
    homophones_map: &HashMap<char, char>,
) -> Vec<String> {
    text.trim()
        .split(|c| (c as u32) < 0x1200 || (c as u32) > 0x135A)
        .filter(|s| !s.is_empty())
        .filter(|s| !stopwords.contains(&String::from(*s)))
        .map(|s| normalize(s, homophones_map))
        .collect()
}

fn normalize(text: &str, homophones_map: &HashMap<char, char>) -> String {
    text.chars()
        .map(|c| *homophones_map.get(&c).unwrap_or(&c))
        .collect()
}

pub fn stem_word(
    word: &str,
    prefix_suffix_pair_list: Vec<(String, String)>,
    prefix_list: Vec<String>,
    suffix_list: Vec<String>,
    sadis_map: &HashMap<char, char>,
) -> String {
    let double_depulicated = deduplicate_double_letter(word, sadis_map);
    let pref_suf_pair_rmvd = rm_prefix_suffix_pair(double_depulicated, prefix_suffix_pair_list);
    let pref_rmvd = rm_affix(pref_suf_pair_rmvd, &prefix_list, AffixType::Prefix);
    deduplicate_single_letter(&rm_affix(pref_rmvd, &suffix_list, AffixType::Suffix), sadis_map)
}

#[derive(Copy, Clone)]
enum AffixType {
    Prefix,
    Suffix,
}

/// Removes double letter duplication. For instance, "ገልጠምጠም" gelTemTem consists of repeated sub-
/// string "ጠም" "Tem". In removing such form, first the radical (sequence of consonants) of the
/// word was extracted and checked for repeating double sequences. In this case "ግልጥምጥም " glTmTm
/// is the radical and has repeated double sequence that is "ጥም" "Tm". Therefore, the first
/// sub-string which is Tem is removed from the string and leaving the word as "ግልጠም" gelTem.
fn deduplicate_double_letter(word: &str, sadis_map: &HashMap<char, char>) -> String {
    let radical = radical(word, sadis_map);
    let indices = find_duplicate_pairs(radical.as_str());
    remove_at_indexes(word, &indices)
}

/// Changes the each character in the string to their sadis i.e. ገልጠምጠም -> ግልጥምጥም
fn radical(word: &str, sadis_map: &HashMap<char, char>) -> String {
    // አ ዐ families seem to be their own sadis forms, TODO findout if this is right
    let mut radical = String::with_capacity(word.chars().count());
    for c in word.chars() {
        radical.push(*sadis_map.get(&c).unwrap());
    }
    radical
}

fn find_duplicate_pairs(s: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    
    if chars.len() < 4 { return indices; }
    
    let mut i = 0;
    while i <= chars.len() - 4 {
        if chars[i..i + 2] == chars[i + 2..i + 4] {
            indices.extend_from_slice(&[i, i + 1]);
            i += 2;
        } else {
            i += 1;
        }
    }
    
    indices
}

/// The second step removes prefix-suffix pair. This step takes the output of the previous step as
/// an input and checks if the word contains match with any of the prefix-suffix pair. If the word
/// contains a match and the remaining string has a length greater than three, then the prefix and
/// the suffix are removed from the word. For example, the መጀመርያ mejemerya contains
/// the prefix-suffix pair መ-ያ me-ya and the remaining string after extracting the pair is
/// ጀመር jemer, which has length of three radicals. Therefore, the prefix and the suffix are
/// removed from the word and gives ጀመር jemer as an output.
fn rm_prefix_suffix_pair(word: String, prefix_suffix_pair_list: Vec<(String, String)>) -> String {
    for (p, s) in prefix_suffix_pair_list {
        if word.starts_with(&p) && word.ends_with(&s) {
            return word.chars().skip(count_radicals(&p)).take(count_radicals(&word) - count_radicals(&s)).collect();
        }
    }
    word
}

fn rm_affix(word: String, affix_list: &Vec<String>, affix_type: AffixType) -> String {
    match affix_type {
        AffixType::Prefix => {
            for p in affix_list {
                if word.starts_with(p) {
                    let stem: String = word.chars().skip(count_radicals(p)).collect();
                    if count_radicals(&stem) >= MINIMUM_STEM_LENGTH.into() {
                        return stem;
                    }
                    return word;
                }
            }
            word
        }
        AffixType::Suffix => {
            for s in affix_list {
                if word.ends_with(s) {
                    let stem: String = word.chars().take(count_radicals(&word) - count_radicals(s)).collect();
                    if count_radicals(&stem) >= MINIMUM_STEM_LENGTH.into() {
                        return stem;
                    }
                    return word;
                }
            }
            word
        }
    }
}

fn deduplicate_single_letter(word: &str, sadis_map: &HashMap<char, char>) -> String {
    let radical = radical(word, sadis_map);
    let chars: Vec<char> = radical.chars().collect();
    let mut indexes: Vec<usize> = Vec::with_capacity(count_radicals(word));
    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            indexes.push(i);
        }
    }
    remove_at_indexes(word, &indexes)
}

fn remove_at_indexes(s: &str, indexes: &[usize]) -> String {
    s.chars()
        .enumerate()
        .filter(|(i, _)| !indexes.contains(i))
        .map(|(_, c)| c)
        .collect()
}

/// Count the number of radicals in the given string, currently is seems like just length of the
/// string for tigrigna, but update this func if any new revelations come up
fn count_radicals(word: &str) -> usize {
    word.chars().count()
}
