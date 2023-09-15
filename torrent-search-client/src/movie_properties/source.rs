use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub enum Source {
    #[default]
    Unknown,
    Cam,
    Telesync,
    Telecine,
    Dvd,
    Hdtv,
    Hdrip,
    WebRip,
    BluRay,
}

impl FromStr for Source {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let source = match s.to_ascii_lowercase().as_str() {
            s if s.contains("cam") => Source::Cam,

            s if s.contains("telesync") => Source::Telesync,
            s if s.contains("hdts") => Source::Telesync,

            s if s.contains("telecine") => Source::Telecine,
            s if s.contains("hdtc") => Source::Telecine,

            s if s.contains("dvd") => Source::Dvd,

            s if s.contains("hdtv") => Source::Hdtv,
            s if s.contains("pdtv") => Source::Hdtv,

            s if s.contains("hdrip") => Source::Hdrip,
            s if s.contains("hd-rip") => Source::Hdrip,

            s if s.contains("web") => Source::WebRip,

            s if s.contains("bluray") => Source::BluRay,
            s if s.contains("brrip") => Source::BluRay,
            s if s.contains("brip") => Source::BluRay,

            s if s.contains("tc") => Source::Telecine,
            s if s.contains("ts") => Source::Telesync,

            _ => Self::Unknown,
        };

        Ok(source)
    }
}
