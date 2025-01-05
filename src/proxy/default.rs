use chrono::{DateTime, Utc};
use std::time::Duration;
use surrealdb::sql::Value;
use uuid::Uuid;

pub trait SurrealSerializer {
    fn serialize(self) -> Value;
}

pub trait SurrealDeserializer where Self: Sized {
    fn from_option(value: Option<&Value>) -> Self {
        match value {
            None => Self::deserialize(&Value::None),
            Some(value) => Self::deserialize(value)
        }
    }

    fn deserialize(value: &Value) -> Self;
}

pub fn test() {
    let val: Option<i32> = <Option::<i32> as SurrealDeserializer>::from_option(Some(&Value::None));
    let val2 = <i32 as SurrealSerializer>::serialize(val.unwrap());
}

impl SurrealSerializer for i32 {
    fn serialize(self) -> Value {
        Value::from(self)
    }
}

impl SurrealDeserializer for i32 {
    fn deserialize(value: &Value) -> i32 {
        if let Value::Number(n) = value {
            n.as_int() as i32
        }
        else {
            panic!("")     
        }
    }
}

impl SurrealSerializer for i64 {
    fn serialize(self) -> Value {
        Value::from(self)
    }
}

impl SurrealDeserializer for i64 {
    fn deserialize(value: &Value) -> i64 {
        if let Value::Number(n) = value {
            n.as_int()
        } else {
            0
        }
    }
}

impl SurrealSerializer for f32 {
    fn serialize(self) -> Value {
        Value::from(self as f64)
    }
}

impl SurrealDeserializer for f32 {
    fn deserialize(value: &Value) -> f32 {
        if let Value::Number(n) = value {
            n.as_float() as f32
        } else {
            0.0
        }
    }
}

impl SurrealSerializer for f64 {
    fn serialize(self) -> Value {
        Value::from(self)
    }
}

impl SurrealDeserializer for f64 {
    fn deserialize(value: &Value) -> f64 {
        if let Value::Number(n) = value {
            n.as_float()
        } else {
            0.0
        }
    }
}

impl SurrealSerializer for bool {
    fn serialize(self) -> Value {
        Value::from(self)
    }
}

impl SurrealDeserializer for bool {
    fn deserialize(value: &Value) -> bool {
        if let Value::Bool(b) = value {
            *b
        } else {
            false
        }
    }
}

impl<T> SurrealSerializer for Vec<T>
where
    T: SurrealSerializer,
{
    fn serialize(self) -> Value {
        Value::Array(self.into_iter().map(|item| item.serialize()).collect())
    }
}

impl<T> SurrealDeserializer for Vec<T>
where
    T: SurrealDeserializer,
{
    fn deserialize(value: &Value) -> Vec<T> {
        if let Value::Array(array) = value {
            array.iter().map(T::deserialize).collect()
        } else {
            Vec::new()
        }
    }
}

impl<T> SurrealSerializer for Option<T>
where
    T: SurrealSerializer,
{
    fn serialize(self) -> Value {
        match self {
            Some(value) => value.serialize(),
            None => Value::None,
        }
    }
}

impl<T> SurrealDeserializer for Option<T>
where
    T: SurrealDeserializer,
{
    fn deserialize(value: &Value) -> Option<T> {
        if value.is_none() {
            None
        } else {
            Some(T::deserialize(value))
        }
    }
}

// Example implementation for String
impl SurrealSerializer for String {
    fn serialize(self) -> Value {
        Value::from(self.clone())
    }
}

impl SurrealDeserializer for String {
    fn deserialize(value: &Value) -> String {
        if let Value::Strand(s) = value {
            s.0.clone()
        } else {
            "".to_owned()
        }
    }
}

// Example implementations for Uuid and Duration
impl SurrealSerializer for Uuid {
    fn serialize(self) -> Value {
        Value::from(self.to_string())
    }
}

impl SurrealDeserializer for Uuid {
    fn deserialize(value: &Value) -> Uuid {
        if let Value::Uuid(uuid) = value {
            uuid.0
        } else {
            Uuid::nil()
        }
    }
}

impl SurrealSerializer for Duration {
    fn serialize(self) -> Value {
        Value::from(self)
    }
}

impl SurrealDeserializer for Duration {
    fn deserialize(value: &Value) -> Duration {
        if let Value::Duration(duration) = value {
            duration.0
        }
        else {
            Duration::default()
        }
    }
}

// Implementation for chrono::DateTime<Utc>
impl SurrealSerializer for DateTime<Utc> {
    fn serialize(self) -> Value {
        Value::from(self)
    }
}

impl SurrealDeserializer for DateTime<Utc> {
    fn deserialize(value: &Value) -> DateTime<Utc> {
        if let Value::Datetime(datetime) = value {
            datetime.0
        } else {
            Utc::now()
        }
    }
}

impl<T> SurrealSerializer for Box<T>
where
    T: SurrealSerializer,
{
    fn serialize(self) -> Value {
        (*self).serialize()
    }
}

impl<T> SurrealDeserializer for Box<T>
where
    T: SurrealDeserializer,
{
    fn deserialize(value: &Value) -> Box<T> {
        Box::new(T::deserialize(value))
    }
}

