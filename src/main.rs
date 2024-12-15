use axum::{routing::post, Router, extract::State};
use std::{sync::Arc, collections::HashMap};

mod stemmer;

async fn stem(
    State(state): State<Arc<AppState>>,
    axum::extract::Json(text): axum::extract::Json<String>,
) -> Result<String, ()> {
    let stemmed = stemmer::tokenize(&text, &state.stopwords, &state.homophones_map)
        .into_iter()
        .map(|word| {
            stemmer::stem_word(
                word,
                state.prefix_suffix_pair_list.clone(),
                state.prefix_list.clone(),
                state.suffix_list.clone(),
                &state.sadis_map,
            )
        })
        .collect::<Vec<String>>()
        .join(" ");
    Ok(stemmed)
}
// curl -X POST http://localhost:3030/ -H 'Content-Type: application/json' -d '"ፀሎት  323 ስልጣን ኣብነት5ትምርቲ መንግስቲaዲሞክራሲ ትግርኛ ሰራዊት ሰዓት ኢየሱስ ምስጢር ሰሜን ዘለኣለም ሰሎሞን እስራኤል ክርስቶስ ሰላም ፅዮን ቋንቋ"'

#[tokio::main]
async fn main() {
    let (prefix_suffix_pair_list, suffix_list, prefix_list, stopwords, homophones_map, sadis_map) = (
        load_pfx_sfx_list("./static/prefix_suffix_pair.json"),
        read_csv("./static/suffix.csv"),
        read_csv("./static/prefix.csv"),
        read_csv("./static/stopwords.csv"),
        load_map("./static/normalize.json"),
        load_map("./static/sadis.json"),
    );

    let state = AppState {
        prefix_suffix_pair_list,
        suffix_list,
        prefix_list,
        stopwords,
        homophones_map,
        sadis_map,
    };

    let app = Router::new()
        .route("/", post(stem))
        .with_state(state.into());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

struct AppState {
    prefix_suffix_pair_list: Vec<(String, String)>,
    suffix_list: Vec<String>,
    prefix_list: Vec<String>,
    stopwords: Vec<String>,
    homophones_map: HashMap<char, char>,
    sadis_map: HashMap<char, char>,
}

fn load_pfx_sfx_list(path: &str) -> Vec<(String, String)> {
    load_json_data(path)
}

fn load_map(path: &str) -> HashMap<char, char> {
    load_json_data(path)
}

fn read_csv(path: &str) -> Vec<String> {
    let data = std::fs::read_to_string(path);
    if let Err(e) = data {
        panic!("{e:?}")
    };
    data.unwrap()
        .split(',')
        .filter(|p| !p.is_empty())
        .map(str::trim)
        .map(String::from)
        .collect()
}

fn load_json_data<T: serde::de::DeserializeOwned>(path: &str) -> T {
    let json_data = std::fs::read_to_string(path);
    if let Err(e) = json_data {
        panic!("{e:?}")
    };
    let data = serde_json::from_str(&json_data.unwrap());
    if let Err(e) = data {
        panic!("{e:?}")
    };
    data.unwrap()
}
