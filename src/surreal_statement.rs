use crate::serialize::SurrealSerialize;

pub fn record<T> (target: &T) -> String where T: SurrealSerialize {
    format!("{} {}", target.into_id_expression(), target.into_set_expression())
}

pub fn id<T> (target: &T) -> String where T: SurrealSerialize {
    target.into_id_expression()
}

pub fn set<T> (target: &T) -> String where T: SurrealSerialize {
   target.into_set_expression()
}

pub fn content<T> (target: &T) -> String where T: SurrealSerialize {
    target.into_content_expression()
}

pub fn date<T> (target: &T) -> String where surrealdb::sql::Datetime: From<T>, T: Clone {
    surrealdb::sql::Datetime::from(target.clone()).to_string()
}

pub fn array<T> (target: &Vec<T>) -> String where surrealdb::sql::Value: From<T>, T: Clone {
    let array_value: Vec<surrealdb::sql::Value> = target.iter().map(|v| {
        surrealdb::sql::Value::from(v.clone())
    })
    .collect();

    surrealdb::sql::Array::from(array_value).to_string()
}
