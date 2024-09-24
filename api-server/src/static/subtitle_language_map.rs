use std::collections::HashMap;

use regex::Regex;

type SubtitleLanguageMap = HashMap<String, Regex>;

pub fn create_subtitle_language_map() -> SubtitleLanguageMap {
    let mut subtitle_language_map = HashMap::new();
    subtitle_language_map.insert(
        "en".to_string(),
        Regex::new(r"^(english|eng|.*\.eng|.*\.en)$").unwrap(),
    );
    subtitle_language_map.insert(
        "nl".to_string(),
        Regex::new(r"^(dutch|nederlands|nl|.*\.nl)$").unwrap(),
    );
    subtitle_language_map
}
