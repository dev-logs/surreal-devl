use surrealdb::{
    opt::{IntoResource, RecordId, Resource},
    sql::{Id, Value},
};

use crate::errors::Error;

pub struct DbResource<R>(pub (String, Id), pub R);

impl<R> IntoResource<R> for DbResource<R> {
    fn into_resource(self) -> surrealdb::Result<Resource> {
        Ok(Resource::RecordId(RecordId {
            tb: self.0 .0,
            id: self.0 .1,
        }))
    }
}

pub trait IntoDbResource<R> {
    const TABLE_NAME: &'static str;
    fn into_db_resource(self) -> Result<DbResource<R>, Error>;
}

impl<R> Into<Value> for DbResource<R> {
    fn into(self) -> Value {
        Value::from(
            <(std::string::String, Id) as IntoResource<std::option::Option<R>>>::into_resource(
                self.0,
            )
            .unwrap(),
        )
    }
}
