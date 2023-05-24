use rocket::form::{self, FromFormField, ValueField};
use torrent_search_client::Category;

pub struct CategoryFormField {
    category: Category,
}

impl CategoryFormField {
    pub fn get(self) -> Category {
        self.category
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for CategoryFormField {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let category = match field.value.to_ascii_lowercase().as_str() {
            "all" => Category::All,
            "applications" => Category::Applications,
            "audio" => Category::Audio,
            "games" => Category::Games,
            "other" => Category::Other,
            "video" => Category::Video,
            _ => Err(form::Error::validation("Incorrect category"))?,
        };

        Ok(CategoryFormField { category })
    }
}
