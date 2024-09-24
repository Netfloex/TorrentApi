use log::debug;
use regex::Regex;
use std::collections::HashMap;

type LanguageAndFlags = String;
type SubtitleNameRegex = Regex;

pub fn parse_subtitle_language(
    subtitle_name: &str,
    subtitle_ext: &str,
    target_base: &str,
    subtitle_language_map: &HashMap<LanguageAndFlags, SubtitleNameRegex>,
) -> Option<String> {
    let subtitle_name = subtitle_name.to_lowercase();
    let mut output = String::from(target_base);
    let mut lang_code = "";

    for (lang, regex) in subtitle_language_map.iter() {
        if regex.is_match(&subtitle_name) {
            lang_code = lang;
            break;
        }
    }

    if lang_code.is_empty() {
        // Allow if has the same base name
        // e.g. subtitle_name: MovieFile.2020.srt
        // target_base: MovieFile.2020

        if subtitle_name.starts_with(&target_base.to_lowercase()) {
            debug!(
                "Subtitle {} has the same base name as target file {}",
                subtitle_name, target_base
            );
            output.push('.');
            output.push_str(subtitle_ext);
            return Some(output);
        }

        return None;
    }

    output.push('.');
    output.push_str(lang_code);
    output.push('.');
    output.push_str(subtitle_ext);

    Some(output)
}

#[cfg(test)]
mod tests {
    use crate::r#static::subtitle_language_map::create_subtitle_language_map;

    use super::*;

    #[tokio::test]
    async fn test_parse_subtitle_language() {
        let subtitle_language_map: HashMap<LanguageAndFlags, SubtitleNameRegex> =
            create_subtitle_language_map();

        assert_eq!(
            parse_subtitle_language(
                "MovieFile.2020",
                "srt",
                "MovieFile.2020",
                &subtitle_language_map
            ),
            Some("MovieFile.2020.srt".to_owned())
        );

        assert_eq!(
            parse_subtitle_language(
                "MovieFile.2020.en",
                "srt",
                "MovieFile.2020",
                &subtitle_language_map
            ),
            Some("MovieFile.2020.en.srt".to_owned())
        );

        assert_eq!(
            parse_subtitle_language("English", "srt", "MovieFile.2020", &subtitle_language_map),
            Some("MovieFile.2020.en.srt".to_owned())
        );

        assert_eq!(
            parse_subtitle_language("Dutch.eng", "srt", "MovieFile.2020", &subtitle_language_map),
            Some("MovieFile.2020.en.srt".to_owned())
        );

        assert_eq!(
            parse_subtitle_language("Dutch", "srt", "MovieFile.2020", &subtitle_language_map),
            Some("MovieFile.2020.nl.srt".to_owned())
        );

        assert_eq!(
            parse_subtitle_language(
                "Something.dut",
                "srt",
                "MovieFile.2020",
                &subtitle_language_map
            ),
            Some("MovieFile.2020.nl.srt".to_owned())
        );
    }
}
