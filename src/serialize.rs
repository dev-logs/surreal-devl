use surrealdb::opt::RecordId;
use surrealdb::sql::{Data, Operator};

pub trait SurrealSerialize where Self: Sized + Into<RecordId> + Clone {
    fn into_idiom_value(&self) -> Vec<(surrealdb::sql::Idiom, surrealdb::sql::Value)>;

    fn into_set_expression(&self) -> String {
        let expressions = self.into_idiom_value().iter().map(|it| {
            return (it.0.clone(), Operator::Equal, it.1.clone());
        }).collect();

        return Data::SetExpression(expressions).to_string();
    }

    fn into_id_expression(&self) -> String {
        let id: RecordId = self.clone().into();
        id.to_string()
    }

    fn into_content_expression(&self) -> String {
        self.into_idiom_value().iter().map(|it| {
            return format!("{}{}{}", it.0.clone(), Operator::Equal, it.1.clone());
        }).collect::<Vec<String>>().join(",")
    }
}
