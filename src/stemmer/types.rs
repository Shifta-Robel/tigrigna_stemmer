use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum StemmerError {
    #[error("Failed to load resource {0}")]
    ResourceError(String),
    #[error("Invalid input: {0}")]
    InputError(String),
}
impl From<std::io::Error> for StemmerError {
    fn from(err: std::io::Error) -> Self {
        StemmerError::ResourceError(err.to_string())
    }
}

impl From<serde_json::Error> for StemmerError {
    fn from(err: serde_json::Error) -> Self {
        StemmerError::ResourceError(err.to_string())
    }
}

pub struct StemmerConfig {
    pub prefix_suffix_pair_list: Vec<(String, String)>,
    pub suffix_list: Vec<String>,
    pub prefix_list: Vec<String>,
    pub stopwords: Vec<String>,
    pub homophones_map: HashMap<char, char>,
    pub sadis_map: HashMap<char, char>,
}
