use crate::proxy::default::{SurrealDeserializer, SurrealSerializer};
use crate::surreal_id::{Link, SurrealId};
use crate::surreal_qr::SurrealResponseError;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use surrealdb::sql::{Thing, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge<I, R, O>
where
    R: SurrealSerializer + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    pub r#in: Option<Link<I>>,
    pub out: Option<Link<O>>,
    #[serde(flatten)]
    pub data: R,
}

impl<I, R, O> PartialEq for Edge<I, R, O>
where
    R: PartialEq + SurrealSerializer + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<I, R, O> Deref for Edge<I, R, O>
where
    R: SurrealSerializer + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub trait IntoRelation<I, O>
where
    Self: SurrealSerializer + SurrealId + Sized,
    I: SurrealId,
    O: SurrealId,
{
    fn relate(self, i: I, o: O) -> Edge<I, Self, O>;
}

impl<I, R, O> IntoRelation<I, O> for R
where
    R: SurrealSerializer + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    fn relate(self, i: I, o: O) -> Edge<I, Self, O> {
        Edge {
            r#in: Some(Link::Id(i.id())),
            out: Some(Link::Id(o.id())),
            data: self,
        }
    }
}

impl<I, R, O> Into<Thing> for Edge<I, R, O>
where
    R: SurrealSerializer + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    fn into(self) -> Thing {
        self.data.id()
    }
}

impl<I, R, O> Into<Value> for Edge<I, R, O>
where
    R: SurrealSerializer + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    fn into(self) -> Value {
        Value::from(self.id())
    }
}

impl<I, R, O> SurrealId for Edge<I, R, O>
where
    R: SurrealSerializer + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    fn id(&self) -> Thing {
        self.data.id()
    }
}

impl<I, R, O> Into<Thing> for &Edge<I, R, O>
where
    R: SurrealSerializer + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    fn into(self) -> Thing {
        (&self.data).id()
    }
}

impl<I, R, O> SurrealSerializer for Edge<I, R, O>
where
    R: SurrealSerializer + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    fn serialize(self) -> Value {
        Value::from(self.data.id())
    }
}

impl<I, R, O> SurrealDeserializer for Edge<I, R, O>
where
    R: SurrealSerializer + SurrealId + SurrealDeserializer,
    I: SurrealId + SurrealDeserializer,
    O: SurrealId + SurrealDeserializer,
{
    fn deserialize(value: &Value) -> Result<Self, SurrealResponseError> {
        let object = match value {
            Value::Object(obj) => obj,
            Value::Array(arr) => {
                if arr.len() != 1 {
                    return Err(
                        SurrealResponseError::ExpectedAnArrayWith1ItemToDeserializeToObject,
                    );
                } else if let Some(Value::Object(obj)) = arr.0.first() {
                    obj
                } else {
                    return Err(SurrealResponseError::ExpectedAnObject);
                }
            }
            _ => return Err(SurrealResponseError::ExpectedAnObject),
        };

        let in_value = object.get("in");
        let out_value = object.get("out");

        Ok(Self {
            r#in: match in_value {
                Some(value) => Some(SurrealDeserializer::deserialize(value)?),
                None => None,
            },
            r#out: match out_value {
                Some(value) => Some(SurrealDeserializer::deserialize(value)?),
                None => None,
            },
            data: SurrealDeserializer::deserialize(&Value::Object(object.clone()))?,
        })
    }
}
