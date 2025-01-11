use surrealdb::sql::{Data, Thing};

use crate::proxy::default::SurrealSerializer;
use crate::surreal_edge::Edge;
use crate::surreal_id::SurrealId;

pub fn record<T>(target: &T) -> String
where
    T: SurrealSerializer + SurrealId + Clone,
{
    let id = target.id();
    format!(
        "{} {}",
        id.to_string(),
        Data::ContentExpression(target.clone().serialize()).to_string()
    )
}

pub fn id<T>(target: &T) -> String
where
    T: SurrealId,
{
    target.id().to_string()
}

pub fn content<T>(target: &T) -> String
where
    T: SurrealSerializer + Clone,
{
    Data::ContentExpression(target.clone().serialize()).to_string()
}

pub fn array<T>(target: &[T]) -> String
where
    T: SurrealSerializer + Clone,
{
    let array_value: Vec<surrealdb::sql::Value> =
        target.into_iter().map(|v| v.clone().serialize()).collect();

    surrealdb::sql::Array::from(array_value).to_string()
}

pub fn val<T>(target: &T) -> String
where
    T: SurrealSerializer,
    T: Clone,
{
    target.clone().serialize().to_string()
}

pub fn relate<I, R, O>(target: &Edge<I, R, O>) -> String
where
    R: SurrealSerializer + SurrealId + Clone,
    I: SurrealId,
    O: SurrealId,
{
    let record_id: Thing = target.data.id();
    let in_id: Thing = target
        .r#in
        .as_ref()
        .expect("In direction cannot be null when serialize")
        .id();
    let out_id: Thing = target
        .out
        .as_ref()
        .expect("Out direction cannot be null when serialize")
        .id();

    format!(
        "RELATE {} -> {} -> {} {}",
        in_id.to_string(),
        record_id,
        out_id.to_string(),
        Data::ContentExpression(target.data.clone().serialize()).to_string()
    )
}
