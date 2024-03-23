use getset::{Getters, Setters};
use multimap::MultiMap;
use url::Url;

#[derive(Getters, Debug, Setters)]
#[getset(get = "pub", set = "pub")]
pub struct Magnet {
    display_name: String,
    trackers: Vec<String>,
    info_hash: String,
}

impl Magnet {
    pub fn from_url(url: &str) -> Result<Self, String> {
        let url = Url::parse(url).map_err(|e| e.to_string())?;

        if url.scheme() != "magnet" {
            return Err("Not a magnet link".into());
        }
        let qp: MultiMap<_, _> = url.query_pairs().collect();

        Ok(Self {
            display_name: qp
                .get("dn")
                .map(|v| v.to_string())
                .unwrap_or_else(|| "Unknown".into()),
            trackers: qp
                .get_vec("tr")
                .map(|v| v.iter().map(|v| v.to_string()).collect())
                .unwrap_or_default(),
            info_hash: qp
                .get("xt")
                .map(|v| v.replace("urn:btih:", ""))
                .unwrap_or_else(|| "Unknown".into()),
        })
    }

    pub fn url(&self) -> String {
        format!(
            "magnet:?xt=urn:btih:{}&dn={}&tr={}",
            self.info_hash,
            self.display_name,
            self.trackers.join("&tr=")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MAGNET: &str = "magnet:?xt=urn:btih:1234567890&dn=Test&tr=udp://test.com";
    #[test]
    fn test_magnet_from_url() {
        let magnet = Magnet::from_url(TEST_MAGNET).unwrap();
        assert_eq!(magnet.display_name(), "Test");
        assert_eq!(magnet.info_hash(), "1234567890");
        assert_eq!(magnet.trackers(), &["udp://test.com"]);
    }

    #[test]
    fn test_magnet_from_url_no_trackers() {
        let magnet = Magnet::from_url("magnet:?xt=urn:btih:1234567890&dn=Test").unwrap();
        assert_eq!(magnet.display_name(), "Test");
        assert_eq!(magnet.info_hash(), "1234567890");
        assert_eq!(magnet.trackers(), &Vec::<String>::new());
    }

    #[test]
    fn test_magnet_from_url_no_display_name() {
        let magnet = Magnet::from_url("magnet:?xt=urn:btih:1234567890&tr=udp://test.com").unwrap();
        assert_eq!(magnet.display_name(), "Unknown");
        assert_eq!(magnet.info_hash(), "1234567890");
        assert_eq!(magnet.trackers(), &["udp://test.com"]);
    }

    #[test]
    fn test_magnet_from_url_no_info_hash() {
        let magnet = Magnet::from_url("magnet:?dn=Test&tr=udp://test.com").unwrap();
        assert_eq!(magnet.display_name(), "Test");
        assert_eq!(magnet.info_hash(), "Unknown");
        assert_eq!(magnet.trackers(), &["udp://test.com"]);
    }

    #[test]
    fn test_magnet_from_url_invalid() {
        assert!(Magnet::from_url("https://google.com").is_err());
    }

    #[test]
    fn test_magnet_url() {
        let magnet = Magnet::from_url(TEST_MAGNET).unwrap();
        assert_eq!(magnet.url(), TEST_MAGNET);
    }

    #[test]
    fn test_magnet_multiple_trackers() {
        let magnet = Magnet::from_url(
            "magnet:?xt=urn:btih:1234567890&dn=Test&tr=udp://test.com&tr=udp://test2.com",
        )
        .unwrap();
        assert_eq!(magnet.trackers(), &["udp://test.com", "udp://test2.com"]);
    }
}
