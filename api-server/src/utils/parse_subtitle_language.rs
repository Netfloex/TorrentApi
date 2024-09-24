use std::collections::HashMap;

use regex::Regex;

// Input:
// name: MovieFile.2020.en.srt
// target_base: MovieFile.2020
// Output MovieFile.2020.en.srt

// Input:
// Name: English.srt
// Target_base: MovieFile.2020
// Output: MovieFile.2020.en.srt

// Input:
// Name: English.eng.srt
// Target_base: MovieFile.2020
// Output: MovieFile.2020.en.srt

// Input:
// Name: dut.srt
// Target_base: MovieFile.2020
// Output: MovieFile.2020.nl.srt

type LanguageAndFlags = String;
type SubtitleNameRegex = Regex;

pub fn parse_subtitle_language(
    subtitle_name: &str,
    subtitle_ext: &str,
    target_base: &str,
    subtitle_language_map: &HashMap<LanguageAndFlags, SubtitleNameRegex>,
) -> Option<String> {
    let mut output = String::from(target_base);
    let mut lang_code = "";

    for (lang, regex) in subtitle_language_map.iter() {
        if regex.is_match(subtitle_name) {
            lang_code = lang;
            break;
        }
    }

    if lang_code.is_empty() {
        // Allow if has the same base name
        // e.g. subtitle_name: MovieFile.2020.srt
        // target_base: MovieFile.2020

        if subtitle_name.starts_with(target_base) {
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
