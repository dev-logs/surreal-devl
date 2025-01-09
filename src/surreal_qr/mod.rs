use std::ops::{Deref, DerefMut};
use std::vec;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use surrealdb::opt::QueryResult;
use surrealdb::sql::{
    Array, Bytes, Datetime, Duration, Geometry, Number, Object, Strand, Thing, Uuid, Value,
};

use surrealdb::Response as QueryResponse;

use crate::proxy::default::SurrealDeserializer;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    CannotReadNoneValue,
    ExpectedAnArrayWith1ItemToDeserializeToObject,
}

impl From<SurrealResponseError> for surrealdb::error::Api {
    fn from(value: SurrealResponseError) -> Self {
        Self::ParseError(format!("{:?}", value))
    }
}

impl From<SurrealResponseError> for surrealdb::Error {
    fn from(value: SurrealResponseError) -> Self {
        Self::Api(surrealdb::error::Api::ParseError(format!("{:?}", value)))
    }
}

pub enum RPath<'a> {
    Index(usize),
    Field(&'a str),
    Chain(Vec<RPath<'a>>),
}

impl<'a> RPath<'a> {
    pub fn from<T>(path: T) -> Self
    where
        T: Into<Self>,
    {
        path.into()
    }

    pub fn get<T>(self, path: T) -> Self
    where
        T: Into<Self>,
    {
        let path: RPath<'a> = path.into();
        match self {
            Self::Chain(mut chain) => {
                chain.push(path);
                Self::Chain(chain)
            }
            Self::Index(index) => {
                let mut chain: Vec<RPath<'a>> = vec![];
                chain.push(RPath::Index(index));
                chain.push(path);
                Self::Chain(chain)
            }
            Self::Field(field) => {
                let mut chain: Vec<RPath<'a>> = vec![];
                chain.push(RPath::Field(field));
                chain.push(path);
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

impl Into<RPath<'static>> for usize {
    fn into(self) -> RPath<'static> {
        RPath::Index(self)
    }
}

impl<'a, T> Into<RPath<'a>> for Vec<T>
where
    T: Into<RPath<'a>>,
{
    fn into(self) -> RPath<'a> {
        RPath::Chain(self.into_iter().map(|it| it.into()).collect())
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

    pub fn get<'a, T>(&self, path: T) -> Result<Self, SurrealResponseError>
    where
        T: Into<RPath<'a>> + Sized,
    {
        let path: RPath = path.into();
        if self.is_none() {
            return Ok(Self(Value::None));
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
            }
            RPath::Chain(mut chain) => {
                if chain.is_empty() {
                    return Ok(self.to_owned());
                }

                self.get(chain.remove(0))?.get(chain)
            }
        }
    }

    pub fn object(&self) -> Result<Option<&Object>, SurrealResponseError> {
        match &self.0 {
            Value::None => Ok(None),
            Value::Object(value) => Ok(Some(value)),
            Value::Array(ref value) => {
                if value.is_empty() {
                    return Ok(None);
                }

                if value.len() != 1 {
                    return Err(SurrealResponseError::ExpectedAnArrayWith1ItemToDeserializeToObject);
                }

                if let Some(Value::Object(ref obj)) = value.0.first() {
                    return Ok(Some(obj));
                }

                Err(SurrealResponseError::ExpectedAnObject)
            }
            _ => Err(SurrealResponseError::ExpectedAnObject),
        }
    }

    pub fn array(&self) -> Result<Option<&Array>, SurrealResponseError> {
        match &self.0 {
            Value::None => Ok(None),
            Value::Array(ref value) => Ok(Some(value)),
            _ => Err(SurrealResponseError::ExpectedAnArray),
        }
    }

    pub fn deserialize<T>(&self) -> Result<T, SurrealResponseError>
    where
        T: SurrealDeserializer,
    {
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

impl<'a, T> QueryResult<Vec<T>> for RPath<'a>
where
    T: SurrealDeserializer + DeserializeOwned,
{
    fn query_result(self, response: &mut QueryResponse) -> surrealdb::Result<Vec<T>> {
        let value: SurrealQR = response.take(self)?;
        if value.is_none() {
            return Ok(vec![]);
        }

        let mut arr = value.array()?;

        if arr.is_none() {
            return Ok(vec![]);
        }

        let mut result = Vec::new();
        for item in arr.take().unwrap().iter() {
            result.push(SurrealDeserializer::deserialize(&item)?);
        }

        Ok(result)
    }
}

impl<'a, T> QueryResult<Option<T>> for RPath<'a>
where
    T: SurrealDeserializer + DeserializeOwned,
{
    fn query_result(self, response: &mut QueryResponse) -> surrealdb::Result<Option<T>> {
        let value: SurrealQR = response.take(self)?;
        if value.is_none_or_null() {
            return Ok(None);
        }

        if value.is_array() {
            let arr = value.array()?;
            if arr.is_none() || arr.unwrap().is_empty() {
                return Ok(None);
            }
        }

        return Ok(value.deserialize()?);
    }
}

impl<'a> QueryResult<SurrealQR> for RPath<'a> {
    fn query_result(self, response: &mut QueryResponse) -> surrealdb::Result<SurrealQR> {
        match self {
            Self::Index(index) => {
                let value: surrealdb::Value = response.take(index)?;
                Ok(SurrealQR(value.into_inner()))
            }
            Self::Chain(mut paths) => {
                if paths.is_empty() {
                    return Err(surrealdb::error::Api::ParseError(
                        "Chain cannot be empty".to_owned(),
                    )
                    .into());
                }

                let value: SurrealQR = match paths.remove(0) {
                    Self::Index(index) => {
                        let value: surrealdb::Value = response.take(index)?;
                        let core_value: Value = value.into_inner();
                        SurrealQR(core_value)
                    }
                    Self::Field(str) => {
                        let value: surrealdb::Value = response.take(0)?;
                        let core_value: Value = value.into_inner();
                        let value = SurrealQR(core_value);
                        value.get(str)?
                    }
                    Self::Chain(paths) => {
                        let value: surrealdb::Value = response.take(0)?;
                        let core_value: Value = value.into_inner();
                        let value = SurrealQR(core_value);
                        value.get(paths)?
                    }
                };

                Ok(value.get(RPath::Chain(paths))?)
            }
            Self::Field(str) => {
                let value: surrealdb::Value = response.take(0)?;
                return Ok(SurrealQR(value.into_inner()).get(RPath::from(str))?);
            }
        }
    }
}
