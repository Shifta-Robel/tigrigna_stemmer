use core::panic;
use std::collections::HashMap;

mod stemmer;

fn main() {
    use stemmer::{stem_word, tokenize};

    let prefix_suffix_pair_list: Vec<(String, String)> = load_pfx_sfx_list("./static/prefix_suffix_pair.json");
    let suffix_list: Vec<String> = read_csv("./static/suffix.csv");
    let prefix_list: Vec<String> = read_csv("./static/prefix.csv");
    let stopwords: Vec<String> = read_csv("./static/stopwords.csv");
    let homophones_map: HashMap<char, char> = load_map("./static/normalize.json");
    let sadis_map: HashMap<char, char> = load_map("./static/sadis.json");

    let text = "ፀሎት  323 ስልጣን ኣብነት5ትምርቲ መንግስቲaዲሞክራሲ ትግርኛ ሰራዊት ሰዓት ኢየሱስ ምስጢር ሰሜን ዘለኣለም ሰሎሞን እስራኤል ክርስቶስ ሰላም ፅዮን ቋንቋ ";

    let stemmed: String = tokenize(text, &stopwords, &homophones_map)
        .into_iter()
        .map(|word| stem_word(word, prefix_suffix_pair_list.clone(), prefix_list.clone(), suffix_list.clone(), &sadis_map))
        .collect::<Vec<String>>()
        .join(" ");
    println!("{text}\n{stemmed}");
}

fn load_pfx_sfx_list(path: &str) -> Vec<(String, String)> {
    load_json_data(path)
}

fn load_map(path: &str) -> HashMap<char, char> {
    load_json_data(path)
}

fn read_csv(path: &str) -> Vec<String> {
    let data = std::fs::read_to_string(path);
    if let Err(e) = data { panic!("{:?}",e) };
    data.unwrap().split(',').filter(|p| !p.is_empty()).map(str::trim).map(String::from).collect()
}

fn load_json_data<T: serde::de::DeserializeOwned>(path: &str) -> T {
    let json_data = std::fs::read_to_string(path);
    if let Err(e) = json_data { panic!("{:?}",e) };
    let data = serde_json::from_str(&json_data.unwrap());
    if let Err(e) = data { panic!("{:?}",e) };
    data.unwrap()
}
