use torrent_search_client::Category;

pub fn parse_category(value: &str) -> Option<Category> {
    let category = match value.to_ascii_lowercase().as_str() {
        "all" => Some(Category::All),
        "applications" => Some(Category::Applications),
        "audio" => Some(Category::Audio),
        "games" => Some(Category::Games),
        "other" => Some(Category::Other),
        "video" => Some(Category::Video),
        _ => None,
    };

    category
}
