use rocket::form::{self, FromFormField, ValueField};

#[derive(Debug)]
pub enum Order {
    Ascending(),
    Descending(),
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Order {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let order = match field.value.to_ascii_lowercase().as_str() {
            "asc" => Order::Ascending(),
            "ascending" => Order::Ascending(),

            "desc" => Order::Descending(),
            "descending" => Order::Descending(),

            _ => Err(form::Error::validation("Incorrect order"))?,
        };

        Ok(order)
    }
}
