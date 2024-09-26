use std::ops::Deref;

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SerdeRegex(#[serde(with = "serde_regex")] Regex);

impl Deref for SerdeRegex {
    type Target = Regex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Regex> for SerdeRegex {
    fn from(val: Regex) -> Self {
        SerdeRegex(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serde_regex() {
        let json = r#""[a-z]+""#;
        let regex: SerdeRegex = serde_json::from_str(json).unwrap();
        assert_eq!(regex.as_str(), "[a-z]+");
    }
}
