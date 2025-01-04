use crate::surreal_id::SurrealId;
use surrealdb::sql::{Data, Operator, Thing};

pub trait SurrealSerialize
where
    Self: Sized + Clone,
{
    fn into_idiom_value(&self) -> Vec<(surrealdb::sql::Idiom, surrealdb::sql::Value)>;

    fn into_set_expression(&self) -> String {
        let expressions = self
            .into_idiom_value()
            .iter()
            .map(|it| {
                return (it.0.clone(), Operator::Equal, it.1.clone());
            })
            .collect();

        return Data::SetExpression(expressions).to_string();
    }

    fn into_id_expression(&self) -> String
    where
        Self: SurrealId,
    {
        let id: Thing = self.id();
        id.to_string()
    }

    fn into_content_expression(&self) -> String {
        self.into_idiom_value()
            .iter()
            .map(|it| format!("{}{}{}", it.0.clone(), Operator::Equal, it.1.clone()))
            .collect::<Vec<String>>()
            .join(",")
    }
}
