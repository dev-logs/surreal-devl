use surrealdb::error::Db::RelateStatement;
use surrealdb::sql::Subquery::Relate;
use crate::serialize::SurrealSerialize;

/// Generate statement that include id and combo of set statements for each struct fields
/// USAGES:
/// ```
/// use surreal_derive::SurrealDerive;
/// use surrealdb::sql::serde;
/// use serde::Deserialize;
/// use serde::Serialize;
/// #[derive(Clone, Serialize, Deserialize, SurrealDerive)]
/// struct Person {
///     name: String,
///     age: i32
/// }
///
/// // It is necessary for a struct to specify what is its primary key
/// impl From<Person> for surrealdb::sql::Value {
///     fn from(value: Person) -> Self {
///         ("person", value.name);
///     }
/// }
///
/// fn main() {
///     use surreal_derive::surreal_quote;
///     let p = Person {name: "surrealdb".to_string(), age: 20};
///     let sql_statement = surreal_quote!("CREATE #record(&person)");
///     assert!(sql_statement, "CREATE person:surrealdb SET name='surrealdb', age=10");
/// }
/// ```
pub fn record<T> (target: &T) -> String where T: SurrealSerialize {
    format!("{} {}", target.into_id_expression(), target.into_set_expression())
}

pub fn multi<I> (targets: &Vec<I>) -> String where I: SurrealSerialize {
    if targets.is_empty() {
        return "".to_string();
    }

    let commands: Vec<String> = targets.iter().map(|target| record(target)).collect();
    commands.join("; ")
}

/// Generate statement that include id and combo of set statements for each struct fields
/// USAGES:
/// ```
/// use surreal_derive::SurrealDerive;
/// use surrealdb::sql::serde;
/// use serde::Deserialize;
/// use serde::Serialize;
/// #[derive(Clone, Serialize, Deserialize, SurrealDerive)]
/// struct Person {
///     name: String,
///     age: i32
/// }
///
/// // It is necessary for a struct to specify what is its primary key
/// impl From<Person> for surrealdb::sql::Value {
///     fn from(value: Person) -> Self {
///         ("person", value.name);
///     }
/// }
///
/// fn main() {
///     use surreal_derive::surreal_quote;
///     let p = Person {name: "surrealdb".to_string(), age: 20};
///     let sql_statement = surreal_quote!("CREATE #record(&person)");
///     assert!(sql_statement, "CREATE person:surrealdb SET name='surrealdb', age=10");
/// }
/// ```
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

pub fn array<T> (target: &[T]) -> String where surrealdb::sql::Value: From<T>, T: Clone {
    let array_value: Vec<surrealdb::sql::Value> = target.iter().map(|v| {
        surrealdb::sql::Value::from(v.clone())
    })
    .collect();

    surrealdb::sql::Array::from(array_value).to_string()
}

pub fn val<T> (target: &T) -> String where surrealdb::sql::Value: From<T>, T: Clone {
    surrealdb::sql::Value::from(target.clone()).to_string()
}

pub fn duration<T> (target: &T) -> String where surrealdb::sql::Duration: From<T>, T: Clone {
    surrealdb::sql::Value::Duration(surrealdb::sql::Duration::from(target.clone())).to_string()
}
