use std::ops::Deref;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};

use crate::proxy::default::{SurrealDeserializer, SurrealSerializer};

pub trait SurrealId {
    fn id(&self) -> Thing;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Link<T>
where
    T: SurrealId,
{
    Id(Thing),
    Record(T),
}

impl<T> PartialEq for Link<T>
where
    T: SurrealId,
{
    fn eq(&self, other: &Self) -> bool {
        other.id() == self.id()
    }
}

impl<T> Link<T>
where
    T: SurrealId,
{
    pub fn id(&self) -> Thing {
        match self {
            Self::Id(id) => id.clone(),
            Self::Record(r) => r.id(),
        }
    }

    pub fn record(self) -> T {
        match self {
            Self::Id(_) => {
                panic!("Expected a record got an id")
            }
            Self::Record(r) => r,
        }
    }
}

impl<T> Into<Thing> for Link<T>
where
    T: SurrealId,
{
    fn into(self) -> Thing {
        self.id()
    }
}

impl<T> Into<Thing> for &Link<T>
where
    T: SurrealId,
{
    fn into(self) -> Thing {
        self.id().clone()
    }
}

impl<T> Deref for Link<T>
where
    T: SurrealId,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Link::Id(_) => {
                panic!("The link can not be deref, it must be Link::Record(T) to be deref")
            }
            Link::Record(r) => &r,
        }
    }
}

impl<T> SurrealSerializer for Link<T>
where
    T: SurrealId + SurrealSerializer,
{
    fn serialize(self) -> Value {
        Value::from(self.id())
    }
}

impl<T> SurrealDeserializer for Link<T>
where
    T: SurrealId + SurrealDeserializer,
{
    fn deserialize(value: &Value) -> Link<T> {
        if let Value::Thing(thing) = value {
            Link::Id(thing.clone())
        } else {
            Link::Record(T::deserialize(value))
        }
    }
}
