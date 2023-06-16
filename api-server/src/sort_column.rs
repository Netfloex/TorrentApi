use rocket::form::{self, FromFormField, ValueField};

#[derive(Debug)]
pub enum SortColumn {
    Added(),
    Size(),
    Leechers(),
    Seeders(),
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for SortColumn {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let sort_column = match field.value.to_ascii_lowercase().as_str() {
            "time" => SortColumn::Added(),
            "added" => SortColumn::Added(),

            "size" => SortColumn::Size(),

            "leechers" => SortColumn::Leechers(),
            "seeders" => SortColumn::Seeders(),

            _ => Err(form::Error::validation("Incorrect sort_column"))?,
        };

        Ok(sort_column)
    }
}
