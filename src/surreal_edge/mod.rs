use crate::serialize::SurrealSerialize;
use crate::surreal_id::{Link, SurrealId};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use surrealdb::sql::{Idiom, Thing, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge<I, R, O>
where
    R: SurrealSerialize + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    pub r#in: Option<Link<I>>,
    pub out: Option<Link<O>>,
    #[serde(flatten)]
    pub data: R,
}

impl<I, R, O> Deref for Edge<I, R, O>
where
    R: SurrealSerialize + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<I, R, O> SurrealSerialize for Edge<I, R, O>
where
    I: SurrealId + Clone,
    R: SurrealSerialize + Clone + SurrealId,
    O: SurrealId + Clone,
{
    fn into_idiom_value(&self) -> Vec<(Idiom, Value)> {
        let mut result: Vec<(Idiom, Value)> = vec![];

        result.push((
            Idiom::from("in".to_string()),
            Value::from(
                self.r#in
                    .clone()
                    .map(|it| Value::from(Into::<Thing>::into(it))),
            ),
        ));
        self.data.clone().into_idiom_value();

        let mut data = self.data.clone().into_idiom_value();
        result.append(&mut data);

        result
    }
}

pub trait IntoRelation<I, O>
where
    Self: SurrealSerialize + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    fn relate(self, i: I, o: O) -> Edge<I, Self, O>;
}

impl<I, R, O> IntoRelation<I, O> for R
where
    R: SurrealSerialize + SurrealId,
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
    R: SurrealSerialize + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    fn into(self) -> Thing {
        self.data.id()
    }
}

impl<I, R, O> Into<Value> for Edge<I, R, O>
where
    R: SurrealSerialize + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    fn into(self) -> Value {
        Value::from(self.id())
    }
}

impl<I, R, O> SurrealId for Edge<I, R, O>
where
    R: SurrealSerialize + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    fn id(&self) -> Thing {
        self.data.id()
    }
}

impl<I, R, O> Into<Thing> for &Edge<I, R, O>
where
    R: SurrealSerialize + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    fn into(self) -> Thing {
        (&self.data).id()
    }
}

impl<I, R, O> From<Value> for Edge<I, R, O>
where
    R: SurrealSerialize + SurrealId,
    I: SurrealId,
    O: SurrealId,
{
    fn from(value: Value) -> Self {
        match value {
            Value::Object(obj) => {
                let in_value = obj.get("in");
                let out_value = obj.get("out");

                Self {
                    r#in: in_value.clone().to_owned().map(|it| it.to_owned().into()),
                    r#out: out_value.clone().map(|it| it.to_owned().into()),
                    data: Value::from(obj).into(),
                }
            }
            _ => {
                panic!("Expected edge must be an object")
            }
        }
    }
}
