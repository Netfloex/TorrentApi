use std::fmt::Debug;

use juniper::GraphQLObject;
use serde_variant::to_variant_name;
use strum::IntoEnumIterator;

#[derive(GraphQLObject, Debug)]
pub struct FilterItem {
    display: String,
    name: String,
}
#[derive(GraphQLObject, Debug)]

pub struct Filter {
    display: String,
    name: String,
    values: Vec<FilterItem>,
}

impl Filter {
    pub fn new<T: Iterator + Debug>(filter: T, display: String, name: String) -> Self
    where
        <T as std::iter::Iterator>::Item: std::fmt::Debug,
        <T as std::iter::Iterator>::Item: serde::Serialize,
    {
        Filter {
            display,
            name,
            values: filter
                .map(|x| FilterItem {
                    display: to_variant_name(&x).unwrap().to_owned(),
                    name: format!("{:?}", x),
                })
                .collect(),
        }
    }
}
