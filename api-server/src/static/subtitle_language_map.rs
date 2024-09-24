use std::collections::HashMap;

use regex::Regex;

type SubtitleLanguageMap = HashMap<String, Regex>;

pub fn create_subtitle_language_map() -> SubtitleLanguageMap {
    [
        (
            "en".to_string(),
            Regex::new(r"^(english|eng|.*\.(en|eng))$").unwrap(),
        ),
        (
            "nl".to_string(),
            Regex::new(r"^(dutch|dut|nl|.*\.(nl|dut))$").unwrap(),
        ),
    ]
    .into_iter()
    .collect()
}
