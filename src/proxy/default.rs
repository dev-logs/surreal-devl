use chrono::{DateTime, Utc};
use std::time::Duration;
use surrealdb::sql::Value;
use uuid::Uuid;

use crate::surreal_qr::SurrealResponseError;

pub trait SurrealSerializer {
    fn serialize(self) -> Value;
}

pub trait SurrealDeserializer
where
    Self: Sized,
{
    fn from_option(value: Option<&Value>) -> Result<Self, SurrealResponseError> {
        match value {
            None => Self::deserialize(&Value::None),
            Some(value) => Self::deserialize(value),
        }
    }

    fn deserialize(value: &Value) -> Result<Self, SurrealResponseError>;
}

impl SurrealSerializer for i32 {
    fn serialize(self) -> Value {
        Value::from(self)
    }
}

impl SurrealDeserializer for i32 {
    fn deserialize(value: &Value) -> Result<i32, SurrealResponseError> {
        if let Value::Number(n) = value {
            Ok(n.as_int() as i32)
        } else {
            Err(SurrealResponseError::ExpectedANumberI64)
        }
    }
}

impl SurrealSerializer for u64 {
    fn serialize(self) -> Value {
        Value::from(self)
    }
}

impl SurrealDeserializer for u64 {
    fn deserialize(value: &Value) -> Result<u64, SurrealResponseError> {
        if let Value::Number(n) = value {
            Ok(n.as_int() as u64)
        } else {
            Err(SurrealResponseError::ExpectedANumberI64)
        }
    }
}

impl SurrealSerializer for i64 {
    fn serialize(self) -> Value {
        Value::from(self)
    }
}

impl SurrealDeserializer for i64 {
    fn deserialize(value: &Value) -> Result<i64, SurrealResponseError> {
        if let Value::Number(n) = value {
            Ok(n.as_int())
        } else {
            Err(SurrealResponseError::ExpectedANumberI64)
        }
    }
}

impl SurrealSerializer for f32 {
    fn serialize(self) -> Value {
        Value::from(self as f64)
    }
}

impl SurrealDeserializer for f32 {
    fn deserialize(value: &Value) -> Result<f32, SurrealResponseError> {
        if let Value::Number(n) = value {
            Ok(n.as_float() as f32)
        } else {
            Err(SurrealResponseError::ExpectedANumberF64)
        }
    }
}

impl SurrealSerializer for f64 {
    fn serialize(self) -> Value {
        Value::from(self)
    }
}

impl SurrealDeserializer for f64 {
    fn deserialize(value: &Value) -> Result<f64, SurrealResponseError> {
        if let Value::Number(n) = value {
            Ok(n.as_float())
        } else {
            Err(SurrealResponseError::ExpectedANumberF64)
        }
    }
}

impl SurrealSerializer for bool {
    fn serialize(self) -> Value {
        Value::from(self)
    }
}

impl SurrealDeserializer for bool {
    fn deserialize(value: &Value) -> Result<bool, SurrealResponseError> {
        if let Value::Bool(b) = value {
            Ok(*b)
        } else {
            Err(SurrealResponseError::ExpectedABool)
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
    fn deserialize(value: &Value) -> Result<Vec<T>, SurrealResponseError> {
        if let Value::Array(array) = value {
            array.iter().map(T::deserialize).collect()
        } else {
            Err(SurrealResponseError::ExpectedAnArray)
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
    fn deserialize(value: &Value) -> Result<Option<T>, SurrealResponseError> {
        if value.is_none() {
            Ok(None)
        } else {
            let result = T::deserialize(value);
            match result {
                Ok(it) => Ok(Some(it)),
                Err(e) => match e {
                    SurrealResponseError::CannotReadNoneValue => Ok(None),
                    e => Err(e),
                },
            }
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
    fn deserialize(value: &Value) -> Result<String, SurrealResponseError> {
        if let Value::Strand(s) = value {
            Ok(s.0.clone())
        } else {
            Err(SurrealResponseError::ExpectedAStrand)
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
    fn deserialize(value: &Value) -> Result<Uuid, SurrealResponseError> {
        if let Value::Uuid(uuid) = value {
            Ok(uuid.0)
        } else {
            Err(SurrealResponseError::ExpectedAUuid)
        }
    }
}

impl SurrealSerializer for Duration {
    fn serialize(self) -> Value {
        Value::from(self)
    }
}

impl SurrealDeserializer for Duration {
    fn deserialize(value: &Value) -> Result<Duration, SurrealResponseError> {
        if let Value::Duration(duration) = value {
            Ok(duration.0)
        } else {
            Err(SurrealResponseError::ExpectedADuration)
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
    fn deserialize(value: &Value) -> Result<DateTime<Utc>, SurrealResponseError> {
        if let Value::Datetime(datetime) = value {
            Ok(datetime.0)
        } else {
            Err(SurrealResponseError::ExpectedADatetime)
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
    fn deserialize(value: &Value) -> Result<Box<T>, SurrealResponseError> {
        Ok(Box::new(T::deserialize(value)?))
    }
}
