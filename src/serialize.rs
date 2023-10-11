use surrealdb::opt::RecordId;
use surrealdb::sql::{Data, Operator};

pub trait SurrealSerialize where Self: Sized + Into<RecordId> {
    fn into_idiom_value(self) -> Vec<(surrealdb::sql::Idiom, surrealdb::sql::Value)>;

    fn into_set_expression(self) -> String {
        let expressions = self.into_idiom_value().iter().map(|it| {
            return (it.0.clone(), Operator::Equal, it.1.clone());
        }).collect();

        return Data::SetExpression(expressions).to_string();
    }
}
