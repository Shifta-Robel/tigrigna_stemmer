use axum::{extract::State, routing::post, Router};
use std::sync::Arc;

mod stemmer;
mod utils;

use stemmer::types::{StemmerConfig, StemmerError};

async fn stem(
    State(state): State<Arc<AppState>>,
    axum::extract::Json(text): axum::extract::Json<String>,
) -> Result<String, ()> {
    let stemmed = stemmer::tokenize(&text, &state.config.stopwords, &state.config.homophones_map)
        .into_iter()
        .map(|word| {
            stemmer::stem_word(
                &word,
                state.config.prefix_suffix_pair_list.clone(),
                state.config.prefix_list.clone(),
                state.config.suffix_list.clone(),
                &state.config.sadis_map,
            )
        })
        .collect::<Vec<String>>()
        .join(" ");
    Ok(stemmed)
}

#[tokio::main]
async fn main() -> Result<(), StemmerError> {
    let config = utils::load_config(
        "./static/prefix_suffix_pair.json",
        "./static/suffix.csv",
        "./static/prefix.csv",
        "./static/stopwords.csv",
        "./static/normalize.json",
        "./static/sadis.json",
    )?;

    let state = AppState { config };

    let app = Router::new()
        .route("/", post(stem))
        .with_state(state.into());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

struct AppState {
    config: StemmerConfig,
}
