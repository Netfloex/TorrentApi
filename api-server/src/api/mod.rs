use crate::context::ContextPointer;
use async_graphql::Context;
pub mod mutation;
pub mod query;

pub fn get_context<'ctx>(context: &Context<'ctx>) -> &'ctx crate::Context {
    context.data::<ContextPointer>().unwrap()
}
