use chrono::Utc;
use std::time::Duration;
use surrealdb::sql::Value;
use uuid::Uuid;

pub trait SurrealDeriveCustom: Into<Value> {}

pub struct SurrealDeriveProxy<T>(pub T);

impl From<Option<Value>> for SurrealDeriveProxy<Value> {
    fn from(value: Option<Value>) -> Self {
        match value {
            Some(value) => Self(value),
            None => Self(Value::None),
        }
    }
}

impl From<Value> for SurrealDeriveProxy<Value> {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl From<Option<&Value>> for SurrealDeriveProxy<Value> {
    fn from(value: Option<&Value>) -> Self {
        match value {
            Some(value) => Self(value.to_owned()),
            None => Self(Value::None),
        }
    }
}

impl<T> From<SurrealDeriveProxy<T>> for Value
where
    T: SurrealDeriveCustom,
{
    fn from(value: SurrealDeriveProxy<T>) -> Self {
        value.0.into()
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `Vec<T>`
impl<T> From<SurrealDeriveProxy<Value>> for Vec<T>
where
    T: From<SurrealDeriveProxy<Value>>,
{
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Array(arr) => arr
                .0
                .into_iter()
                .map(|val| T::from(SurrealDeriveProxy(val)))
                .collect(),
            _ => panic!("Expected type array"),
        }
    }
}

impl<T> From<SurrealDeriveProxy<Value>> for Option<Vec<T>>
where
    T: From<SurrealDeriveProxy<Value>>,
{
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::None | Value::Null => return None,
            _ => {}
        };

        Some(proxy.into())
    }
}

impl From<SurrealDeriveProxy<Value>> for bool {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Bool(bool) => bool,
            _ => panic!("Expected type bool"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `Option<String>`
impl From<SurrealDeriveProxy<Value>> for Option<bool> {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Bool(bool) => Some(bool),
            Value::Null | Value::None => None,
            _ => panic!("Expected type bool or null"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `String`
impl From<SurrealDeriveProxy<Value>> for String {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Strand(strand) => strand.0,
            _ => panic!("Expected type string"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `Option<String>`
impl From<SurrealDeriveProxy<Value>> for Option<String> {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Strand(strand) => Some(strand.0),
            Value::Null | Value::None => None,
            _ => panic!("Expected type string or null"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `Uuid`
impl From<SurrealDeriveProxy<Value>> for Uuid {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Uuid(uuid) => uuid.0,
            _ => panic!("Expected type UUID string"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `Option<Uuid>`
impl From<SurrealDeriveProxy<Value>> for Option<Uuid> {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Uuid(uuid) => Some(uuid.0),
            Value::Null | Value::None => None,
            _ => panic!("Expected type UUID string or null"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `Duration`
impl From<SurrealDeriveProxy<Value>> for Duration {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Duration(duration) => duration.0,
            _ => panic!("Expected type duration in seconds as a number"),
        }
    }
}

impl<T> From<SurrealDeriveProxy<Value>> for Option<T>
where
    T: SurrealDeriveCustom + From<SurrealDeriveProxy<Value>>,
{
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Null | Value::None => return None,
            value => Some(SurrealDeriveProxy(value).into()),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `Option<Duration>`
impl From<SurrealDeriveProxy<Value>> for Option<Duration> {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Duration(duration) => Some(duration.0),
            Value::Null | Value::None => None,
            _ => panic!("Expected type duration in seconds or null"),
        }
    }
}

impl From<SurrealDeriveProxy<Value>> for u32 {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Number(num) => num.try_into().expect("Number from db is not u32"),
            _ => panic!("Expected type u32 number"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `Option<u32>`
impl From<SurrealDeriveProxy<Value>> for Option<u32> {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Number(num) => Some(num.try_into().expect("Number from db is not u32")),
            Value::Null | Value::None => None,
            _ => panic!("Expected type u32 number or null"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `f64`
impl From<SurrealDeriveProxy<Value>> for f64 {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Number(num) => num.try_into().expect("Invalid f64 format"),
            _ => panic!("Expected type f64 number"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `Option<f64>`
impl From<SurrealDeriveProxy<Value>> for Option<f64> {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Number(num) => Some(num.try_into().expect("Invalid f64 format")),
            Value::Null | Value::None => None,
            _ => panic!("Expected type f64 number or null"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `i32`
impl From<SurrealDeriveProxy<Value>> for i32 {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Number(num) => num.try_into().expect("Invalid i32 format"),
            _ => panic!("Expected type i32 number"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `Option<i32>`
impl From<SurrealDeriveProxy<Value>> for Option<i32> {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Number(num) => Some(num.try_into().expect("Invalid i32 format")),
            Value::Null | Value::None => None,
            _ => panic!("Expected type i32 number or null"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `i64`
impl From<SurrealDeriveProxy<Value>> for i64 {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Number(num) => num.try_into().expect("Invalid i64 format"),
            _ => panic!("Expected type i64 number"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `Option<i64>`
impl From<SurrealDeriveProxy<Value>> for Option<i64> {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Number(num) => Some(num.try_into().expect("Invalid i64 format")),
            Value::Null | Value::None => None,
            _ => panic!("Expected type i64 number or null"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `u64`
impl From<SurrealDeriveProxy<Value>> for u64 {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Number(num) => num.try_into().expect("Invalid u64 format"),
            _ => panic!("Expected type u64 number"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `Option<u64>`
impl From<SurrealDeriveProxy<Value>> for Option<u64> {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Number(num) => Some(num.try_into().expect("Invalid u64 format")),
            Value::Null | Value::None => None,
            _ => panic!("Expected type u64 number or null"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `u64`
impl From<SurrealDeriveProxy<Value>> for chrono::DateTime<Utc> {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Datetime(datetime) => datetime.0,
            _ => panic!("Expected type u64 number"),
        }
    }
}

/// Implement `From` for `SurrealDeriveProxy<Value>` to `Option<u64>`
impl From<SurrealDeriveProxy<Value>> for Option<chrono::DateTime<Utc>> {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Datetime(datetime) => Some(datetime.0),
            Value::Null | Value::None => None,
            _ => panic!("Expected type u64 number or null"),
        }
    }
}

impl From<SurrealDeriveProxy<Value>> for Vec<u8> {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Bytes(bytes) => bytes.into_inner(),
            _ => panic!("Expected type vec<u8> as bytes"),
        }
    }
}

impl From<SurrealDeriveProxy<Value>> for Option<Vec<u8>> {
    fn from(proxy: SurrealDeriveProxy<Value>) -> Self {
        match proxy.0 {
            Value::Bytes(bytes) => Some(bytes.into_inner()),
            Value::Null | Value::None => None,
            _ => panic!("Expected type vec<8> as bytes"),
        }
    }
}

impl<T> From<SurrealDeriveProxy<Vec<T>>> for Value
where
    T: Into<Value>,
{
    fn from(value: SurrealDeriveProxy<Vec<T>>) -> Self {
        let values: Vec<Value> = value.0.into_iter().map(|it| it.into()).collect();
        Self::from(values)
    }
}

impl<T> From<SurrealDeriveProxy<Option<Vec<T>>>> for Value
where
    T: Into<Value>,
{
    fn from(value: SurrealDeriveProxy<Option<Vec<T>>>) -> Self {
        match value.0 {
            Some(values) => {
                let values: Vec<Value> = values.into_iter().map(|it| it.into()).collect();
                Self::from(values)
            }
            None => Self::None,
        }
    }
}

impl<T> From<SurrealDeriveProxy<Option<T>>> for Value
where
    T: SurrealDeriveCustom,
{
    fn from(value: SurrealDeriveProxy<Option<T>>) -> Self {
        match value.0 {
            None => Self::None,
            Some(v) => v.into(),
        }
    }
}

/// Implement `From<SurrealDeriveProxy<T>>` for `Value` for `i64`
impl From<SurrealDeriveProxy<i64>> for Value {
    fn from(proxy: SurrealDeriveProxy<i64>) -> Self {
        Value::from(proxy.0)
    }
}

/// Implement `From<SurrealDeriveProxy<T>>` for `Value` for `Option<i64>`
impl From<SurrealDeriveProxy<Option<i64>>> for Value {
    fn from(proxy: SurrealDeriveProxy<Option<i64>>) -> Self {
        match proxy.0 {
            Some(value) => Value::from(value),
            None => Value::Null,
        }
    }
}

/// Implement `From<SurrealDeriveProxy<T>>` for `Value` for `String`
impl From<SurrealDeriveProxy<String>> for Value {
    fn from(proxy: SurrealDeriveProxy<String>) -> Self {
        Value::Strand(proxy.0.into())
    }
}

/// Implement `From<SurrealDeriveProxy<T>>` for `Value` for `Option<String>`
impl From<SurrealDeriveProxy<Option<String>>> for Value {
    fn from(proxy: SurrealDeriveProxy<Option<String>>) -> Self {
        match proxy.0 {
            Some(value) => Value::Strand(value.into()),
            None => Value::Null,
        }
    }
}

/// Implement `From<SurrealDeriveProxy<T>>` for `Value` for `Uuid`
impl From<SurrealDeriveProxy<Uuid>> for Value {
    fn from(proxy: SurrealDeriveProxy<Uuid>) -> Self {
        Value::Uuid(proxy.0.into())
    }
}

/// Implement `From<SurrealDeriveProxy<T>>` for `Value` for `Option<Uuid>`
impl From<SurrealDeriveProxy<Option<Uuid>>> for Value {
    fn from(proxy: SurrealDeriveProxy<Option<Uuid>>) -> Self {
        match proxy.0 {
            Some(uuid) => Value::Uuid(uuid.into()),
            None => Value::Null,
        }
    }
}

/// Implement `From<SurrealDeriveProxy<T>>` for `Value` for `Duration`
impl From<SurrealDeriveProxy<Duration>> for Value {
    fn from(proxy: SurrealDeriveProxy<Duration>) -> Self {
        Value::Duration(proxy.0.into())
    }
}

/// Implement `From<SurrealDeriveProxy<T>>` for `Value` for `Option<Duration>`
impl From<SurrealDeriveProxy<Option<Duration>>> for Value {
    fn from(proxy: SurrealDeriveProxy<Option<Duration>>) -> Self {
        match proxy.0 {
            Some(duration) => Value::Duration(duration.into()),
            None => Value::Null,
        }
    }
}

/// Implement `From<SurrealDeriveProxy<T>>` for `Value` for `f64`
impl From<SurrealDeriveProxy<f64>> for Value {
    fn from(proxy: SurrealDeriveProxy<f64>) -> Self {
        Value::from(proxy.0)
    }
}

/// Implement `From<SurrealDeriveProxy<T>>` for `Value` for `Option<f64>`
impl From<SurrealDeriveProxy<Option<f64>>> for Value {
    fn from(proxy: SurrealDeriveProxy<Option<f64>>) -> Self {
        match proxy.0 {
            Some(value) => Value::from(value),
            None => Value::Null,
        }
    }
}

impl From<SurrealDeriveProxy<i32>> for Value {
    fn from(proxy: SurrealDeriveProxy<i32>) -> Self {
        Value::from(proxy.0)
    }
}

/// Implement `From<SurrealDeriveProxy<T>>` for `Value` for `Option<f64>`
impl From<SurrealDeriveProxy<Option<i32>>> for Value {
    fn from(proxy: SurrealDeriveProxy<Option<i32>>) -> Self {
        match proxy.0 {
            Some(value) => Value::from(value),
            None => Value::Null,
        }
    }
}

/// Implement `From<SurrealDeriveProxy<T>>` for `Value` for `DateTime<Utc>`
impl From<SurrealDeriveProxy<chrono::DateTime<Utc>>> for Value {
    fn from(proxy: SurrealDeriveProxy<chrono::DateTime<Utc>>) -> Self {
        Value::Datetime(proxy.0.into())
    }
}

/// Implement `From<SurrealDeriveProxy<T>>` for `Value` for `Option<DateTime<Utc>>`
impl From<SurrealDeriveProxy<Option<chrono::DateTime<Utc>>>> for Value {
    fn from(proxy: SurrealDeriveProxy<Option<chrono::DateTime<Utc>>>) -> Self {
        match proxy.0 {
            Some(datetime) => Value::Datetime(datetime.into()),
            None => Value::Null,
        }
    }
}
