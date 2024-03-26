use std::fmt::Debug;

use async_graphql::SimpleObject;
use serde_variant::to_variant_name;

#[derive(Debug, SimpleObject)]
pub struct FilterItem {
    display: String,
    name: String,
}
#[derive(Debug, SimpleObject)]

pub struct Filter {
    display: String,
    name: String,
    type_name: String,
    values: Vec<FilterItem>,
}

impl Filter {
    pub fn new<T: Iterator + Debug>(
        filter: T,
        display: String,
        name: String,
        type_name: String,
    ) -> Self
    where
        <T as std::iter::Iterator>::Item: std::fmt::Debug,
        <T as std::iter::Iterator>::Item: serde::Serialize,
    {
        Filter {
            display,
            name,
            type_name,
            values: filter
                .map(|x| FilterItem {
                    display: to_variant_name(&x).unwrap().to_owned(),
                    name: format!("{:?}", x),
                })
                .collect(),
        }
    }
}
