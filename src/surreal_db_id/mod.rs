use surrealdb::opt::RecordId;
use surrealdb::sql::{Idiom, Value};
use surrealdb_id::link::Link;
use surrealdb_id::relation::Relation;
use crate::serialize::SurrealSerialize;

impl<T> SurrealSerialize for Link<T> where T: SurrealSerialize + Into<RecordId> + Clone {
    fn into_idiom_value(&self) -> Vec<(Idiom, Value)> {
        match self {
            Link::Id(i) => vec![(Idiom::from("id".to_string()), Value::from(i.clone()))],
            Link::Record(r) => r.into_idiom_value()
        }
    }
}

impl SurrealSerialize for RecordId {
    fn into_idiom_value(&self) -> Vec<(Idiom, Value)> {
        vec![(Idiom::from("id".to_string()), Value::from(self.clone()))]
    }
}

impl<I, R, O> SurrealSerialize for Relation<I, R, O> where
    I: Into<RecordId> + Clone + Sized,
    R: Into<RecordId> + Clone + Sized + SurrealSerialize,
    O: Into<RecordId> + Clone + Sized
{
    fn into_idiom_value(&self) -> Vec<(Idiom, Value)> {
        let mut result: Vec<(Idiom, Value)> = vec![];

        result.push((Idiom::from("in".to_string()), Value::from(self.r#in.clone().map(|it| Value::from(Into::<RecordId>::into(it))))));

        result.push((Idiom::from("relation".to_string()), Value::from(Into::<RecordId>::into(self.relation.clone()))));

        result.push((Idiom::from("out".to_string()), Value::from(self.out.clone().map(|it| Value::from(Into::<RecordId>::into(it))))));

        result
    }
}
