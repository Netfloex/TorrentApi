use derive_getters::Getters;
use juniper::GraphQLInputObject;
use rocket::form::{self, Error};

#[derive(FromForm, Debug, GraphQLInputObject, Getters)]
pub struct SearchParams {
    query: Option<String>,
    #[field(validate = or(&self.query))]
    imdb: Option<String>,
    #[field(validate = or(&self.query))]
    title: Option<String>,
    #[field(validate = or(&self.imdb))]
    category: Option<String>,
    sort: Option<String>,
    order: Option<String>,
    limit: Option<i32>,
    quality: Option<Vec<String>>,
    codec: Option<Vec<String>>,
    source: Option<Vec<String>>,
}

fn or<'v>(first: &Option<String>, second: &Option<String>) -> form::Result<'v, ()> {
    match (first, second) {
        (Some(_), Some(_)) => Err(Error::validation("Not both"))?,
        _ => Ok(()),
    }
}
