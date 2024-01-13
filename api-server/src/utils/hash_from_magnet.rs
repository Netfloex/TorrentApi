pub fn hash_from_magnet(url: &str) -> Option<String> {
    let url = url.to_lowercase();
    let hash = url.split("btih:").nth(1)?.split('&');

    hash.clone().next().map(|h| h.to_string())
}
