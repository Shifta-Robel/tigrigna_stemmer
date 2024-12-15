use std::collections::HashMap;

use serde::de::DeserializeOwned;

use crate::stemmer::types::{StemmerConfig, StemmerError};

pub fn load_config(
    pfx_sfx_path: &str,
    suffix_path: &str,
    prefix_path: &str,
    stopwords_path: &str,
    homophones_path: &str,
    sadis_path: &str,
) -> Result<StemmerConfig, StemmerError> {
    Ok(StemmerConfig {
        prefix_suffix_pair_list: load_pfx_sfx_list(pfx_sfx_path)?,
        suffix_list: read_csv(suffix_path)?,
        prefix_list: read_csv(prefix_path)?,
        stopwords: read_csv(stopwords_path)?,
        homophones_map: load_map(homophones_path)?,
        sadis_map: load_map(sadis_path)?,
    })
}

fn load_pfx_sfx_list(path: &str) -> Result<Vec<(String, String)>, StemmerError> {
    load_json_data(path)
}

fn load_map(path: &str) -> Result<HashMap<char, char>, StemmerError> {
    load_json_data(path)
}

fn read_csv(path: &str) -> Result<Vec<String>, StemmerError> {
    Ok(std::fs::read_to_string(path)
        .map_err(StemmerError::from)?
        .split(',')
        .filter(|p| !p.is_empty())
        .map(str::trim)
        .map(String::from)
        .collect())
}

fn load_json_data<T: DeserializeOwned>(path: &str) -> Result<T, StemmerError> {
    let json_data = std::fs::read_to_string(path).map_err(StemmerError::from)?;
    serde_json::from_str(&json_data).map_err(StemmerError::from)
}
