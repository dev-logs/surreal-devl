use crate::serialize::SurrealSerialize;
use crate::surreal_edge::Edge;
use crate::surreal_id::SurrealId;
use surrealdb::sql::Thing;

pub fn record<T>(target: &T) -> String
where
    T: SurrealSerialize + SurrealId,
{
    format!(
        "{} {}",
        target.into_id_expression(),
        target.into_set_expression()
    )
}

pub fn id<T>(target: &T) -> String
where
    T: SurrealSerialize + SurrealId,
{
    target.into_id_expression()
}

pub fn set<T>(target: &T) -> String
where
    T: SurrealSerialize,
{
    target.into_set_expression()
}

pub fn content<T>(target: &T) -> String
where
    T: SurrealSerialize,
{
    target.into_content_expression()
}

pub fn date<T>(target: &T) -> String
where
    surrealdb::sql::Datetime: From<T>,
    T: Clone,
{
    surrealdb::sql::Datetime::from(target.clone()).to_string()
}

pub fn array<T>(target: &[T]) -> String
where
    surrealdb::sql::Value: From<T>,
    T: Clone,
{
    let array_value: Vec<surrealdb::sql::Value> = target
        .iter()
        .map(|v| surrealdb::sql::Value::from(v.clone()))
        .collect();

    surrealdb::sql::Array::from(array_value).to_string()
}

pub fn val<T>(target: &T) -> String
where
    surrealdb::sql::Value: From<T>,
    T: Clone,
{
    surrealdb::sql::Value::from(target.clone()).to_string()
}

pub fn duration<T>(target: &T) -> String
where
    surrealdb::sql::Duration: From<T>,
    T: Clone,
{
    surrealdb::sql::Value::Duration(surrealdb::sql::Duration::from(target.clone())).to_string()
}

pub fn relate<I, R, O>(target: &Edge<I, R, O>) -> String
where
    R: SurrealSerialize + SurrealId,
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
        record_id.tb,
        out_id.to_string(),
        set(&target.data)
    )
}
