use std::ops::{Deref, DerefMut};
use std::vec;

use serde::{Deserialize, Serialize};
use surrealdb::method::Stats;
use surrealdb::opt::QueryResult;
use surrealdb::sql::{
    Array, Bytes, Datetime, Duration, Geometry, Number, Object, Strand, Thing, Uuid, Value,
};

use surrealdb::Response as QueryResponse;

use crate::proxy::default::SurrealDeserializer;
use crate::serialize::SurrealSerialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealQR(pub Value);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SurrealResponseError {
    ExpectedAnObject,
    ExpectedAnArray,
    ExpectedANumber,
    ExpectedANumberI64,
    ExpectedANumberF64,
    ExpectedANumberDecimal,
    ExpectedAStrand,
    ExpectedAThing,
    ExpectedADuration,
    ExpectedADatetime,
    ExpectedAGeometry,
    ExpectedABool,
    ExpectedAUuid,
    ExpectedABytes,
    ExpectedASet,
    UnexpectedValueType,
    OutOfRange,
}

pub enum RPath<'a> {
    Index(i32),
    Field(&'a str),
    Chain(Vec<RPath<'a>>)
}

impl<'a> RPath<'a> {
    pub fn from<T>(path: T) -> Self where T: Into<Self> {
        path.into()
    }

    pub fn to<T>(self, path: T) -> Self where T: Into<Self> {
        let path: RPath<'a> = path.into();
        match self {
            Self::Chain(mut chain) => {
                chain.push(path);
                Self::Chain(chain)
            },
            Self::Index(index) => {
                let mut chain: Vec<RPath<'a>> = vec![];
                chain.push(RPath::Index(index));
                Self::Chain(chain)
            },
            Self::Field(field) => {
                let mut chain: Vec<RPath<'a>> = vec![];
                chain.push(RPath::Field(field));
                Self::Chain(chain)
            }
        }
    }
}

impl<'a> Into<RPath<'a>> for &'a str {
    fn into(self) -> RPath<'a> {
        RPath::Field(self)
    }
}

impl Into<RPath<'static>> for i32 {
    fn into(self) -> RPath<'static> {
        RPath::Index(self)
    }
}

impl SurrealQR {
    pub fn optional(self) -> Option<Self> {
        match self.0 {
            Value::None => None,
            Value::Null => None,
            _ => Some(self),
        }
    }

    pub fn is_none(&self) -> bool {
        return self.0.is_none_or_null();
    }

    pub fn get<'a, T>(self, path: T) -> Result<Self, SurrealResponseError> where T: Into<RPath<'a>> + Sized {
        let path: RPath = path.into();
        if self.is_none() {
            return Ok(self);
        }

        match path {
            RPath::Index(index) => {
                let array = self.array()?;
                if None == array {
                    Ok(Self(Value::None))
                } else {
                    let mut array = array.unwrap().to_owned();
                    if array.len() - 1 < index as usize {
                        return Err(SurrealResponseError::OutOfRange);
                    }

                    let value = array.remove(index as usize);
                    Ok(Self(value))
                }
            }
            RPath::Field(field) => {
                let object = self.object()?;
                if None == object {
                    Ok(Self(Value::None))
                } else {
                    let object = object.unwrap();
                    let value = object.get(field);
                    if None == value {
                        Ok(Self(Value::None))
                    } else {
                        Ok(Self(value.unwrap().to_owned()))
                    }
                }
            },
            RPath::Chain(mut chain) => {
                if chain.is_empty() {
                    return Ok(Self(Value::None));
                }

                let mut item = self.get(chain.swap_remove(0))?;
                for path in chain.into_iter() {
                    item = item.get(path)?;
                }

                Ok(item)
            }
        }
    }

    pub fn object(self) -> Result<Option<Object>, SurrealResponseError> {
        match self.0 {
            Value::None => Ok(None),
            Value::Object(value) => Ok(Some(value)),
            Value::Array(value) => {
                if value.len() != 1 {
                    return Err(SurrealResponseError::ExpectedAnObject)
                }

                if let Some(Value::Object(obj)) = value.0.first() {
                    return Ok(Some(obj.clone()))
                }

                Err(SurrealResponseError::ExpectedAnObject)
            }
            _ => Err(SurrealResponseError::ExpectedAnObject),
        }
    }

    pub fn array(self) -> Result<Option<Array>, SurrealResponseError> {
        match self.0 {
            Value::None => Ok(None),
            Value::Array(value) => Ok(Some(value)),
            _ => Err(SurrealResponseError::ExpectedAnArray),
        }
    }

    pub fn deserialize<T>(&self) -> Result<T, SurrealResponseError> where T: SurrealDeserializer {
        SurrealDeserializer::deserialize(&self.0)
    }

    pub fn number(self) -> Result<Option<Number>, SurrealResponseError> {
        match self.0 {
            Value::None => Ok(None),
            Value::Number(value) => Ok(Some(value)),
            _ => Err(SurrealResponseError::ExpectedANumber),
        }
    }

    pub fn strand(self) -> Result<Option<Strand>, SurrealResponseError> {
        match self.0 {
            Value::None => Ok(None),
            Value::Strand(value) => Ok(Some(value)),
            _ => Err(SurrealResponseError::ExpectedAStrand),
        }
    }

    pub fn thing(self) -> Result<Option<Thing>, SurrealResponseError> {
        match self.0 {
            Value::None => Ok(None),
            Value::Thing(value) => Ok(Some(value)),
            _ => Err(SurrealResponseError::ExpectedAThing),
        }
    }

    pub fn duration(self) -> Result<Option<Duration>, SurrealResponseError> {
        match self.0 {
            Value::None => Ok(None),
            Value::Duration(value) => Ok(Some(value)),
            _ => Err(SurrealResponseError::ExpectedADuration),
        }
    }

    pub fn datetime(self) -> Result<Option<Datetime>, SurrealResponseError> {
        match self.0 {
            Value::None => Ok(None),
            Value::Datetime(value) => Ok(Some(value)),
            _ => Err(SurrealResponseError::ExpectedADatetime),
        }
    }

    pub fn geometry(self) -> Result<Option<Geometry>, SurrealResponseError> {
        match self.0 {
            Value::None => Ok(None),
            Value::Geometry(value) => Ok(Some(value)),
            _ => Err(SurrealResponseError::ExpectedAGeometry),
        }
    }

    pub fn boolean(self) -> Result<Option<bool>, SurrealResponseError> {
        match self.0 {
            Value::None => Ok(None),
            Value::Bool(value) => Ok(Some(value)),
            _ => Err(SurrealResponseError::ExpectedABool),
        }
    }

    pub fn uuid(self) -> Result<Option<Uuid>, SurrealResponseError> {
        match self.0 {
            Value::None => Ok(None),
            Value::Uuid(value) => Ok(Some(value)),
            _ => Err(SurrealResponseError::ExpectedAUuid),
        }
    }

    pub fn bytes(self) -> Result<Option<Bytes>, SurrealResponseError> {
        match self.0 {
            Value::None => Ok(None),
            Value::Bytes(value) => Ok(Some(value)),
            _ => Err(SurrealResponseError::ExpectedABytes),
        }
    }

    pub fn as_i64(self) -> Result<i64, SurrealResponseError> {
        let number = self.number()?;

        match number {
            Some(Number::Int(value)) => Ok(value),
            _ => Err(SurrealResponseError::ExpectedANumberI64),
        }
    }

    pub fn as_f64(self) -> Result<f64, SurrealResponseError> {
        let number = self.number()?;
        match number {
            Some(Number::Float(value)) => Ok(value),
            _ => Err(SurrealResponseError::ExpectedANumberF64),
        }
    }
}

impl Deref for SurrealQR {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SurrealQR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<Value> for SurrealQR {
    fn into(self) -> Value {
        self.0
    }
}

impl<T> TryFrom<SurrealQR> for Vec<T>
where
    T: SurrealDeserializer,
{
    type Error = SurrealResponseError;
    fn try_from(value: SurrealQR) -> Result<Self, Self::Error> {
        if value.is_none() {
            return Ok(vec![]);
        }

        let value: Value = value.0;
        let surrealqr = SurrealQR(value);
        let mut arr = surrealqr.array()?;

        if arr.is_none() {
            return Ok(vec![]);
        }

        let mut result = Vec::new();
        for item in arr.take().unwrap().into_iter() {
            result.push(T::deserialize(&item)?);
        }

        Ok(result)
    }
}

impl<T> TryFrom<SurrealQR> for Option<T>
where
    T: SurrealDeserializer,
{
    type Error = SurrealResponseError;
    fn try_from(surrealqr: SurrealQR) -> Result<Self, Self::Error> {
        if surrealqr.is_none() {
            return Ok(None);
        }

        if surrealqr.is_array() {
            let array = surrealqr.array()?;
            if array.is_none() {
                return Ok(None);
            }

            let mut array = array.unwrap();
            if array.is_empty() {
                return Ok(None);
            }

            let obj = array.swap_remove(0);
            return Ok(Some(T::deserialize(&obj)?));
        }

        if surrealqr.is_object() {
            let object = surrealqr.object();
            let object = object.unwrap();
            if object.is_none() {
                return Ok(None);
            } else {
                return Ok(Self::Some(T::deserialize(&Value::Object(object.unwrap()))?));
            }
        }

        let value: Value = surrealqr.into();
        Ok(Self::Some(T::deserialize(&value)?))
    }
}

#[derive(Serialize, Deserialize)]
pub struct SurrealRecord<T>(T)
where
    T: SurrealDeserializer + SurrealSerialize + Serialize;

impl<T> QueryResult<SurrealRecord<T>> for usize
where
    T: SurrealDeserializer + SurrealSerialize + Serialize + for<'de> Deserialize<'de>,
{
    fn query_result(self, response: &mut QueryResponse) -> surrealdb::Result<SurrealRecord<T>> {
        let value: SurrealQR = response.take(self)?;
        let value: T = value.try_into().unwrap();
    }

    fn stats(&self, _: &QueryResponse) -> Option<Stats> {
        None
    }
}

impl QueryResult<SurrealQR> for usize {
    fn query_result(self, response: &mut QueryResponse) -> surrealdb::Result<SurrealQR> {
        let value: surrealdb::Value = response.take(self)?;
        Ok(SurrealQR(value.into_inner()))
    }

    fn stats(&self, _: &QueryResponse) -> Option<Stats> {
        return None
    }
}

