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
