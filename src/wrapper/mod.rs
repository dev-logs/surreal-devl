use serde::{Deserialize, Serialize};
use surrealdb::method::Stats;
use surrealdb::opt::QueryResult;
use surrealdb::sql::{
    Array, Bytes, Datetime, Duration, Geometry, Number, Object, Strand, Thing, Uuid, Value,
};
use surrealdb::Response as QueryResponse;

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
    OutOfRange
}

pub enum QlPath<'a> {
    Index(i32),
    Field(&'a str)
}

impl SurrealQR {
    pub fn optional(self) -> Option<Self> {
        match self.0 {
            Value::None => None,
            Value::Null => None,
            _ => Some(self)
        }
    }

    pub fn is_none(&self) -> bool {
        return self.0.is_none_or_null()
    }

    pub fn get(self, path: QlPath) -> Result<Self, SurrealResponseError> {
        if self.is_none() {
            return Ok(self);
        }

        match path {
            QlPath::Index(index) => {
                let array = self.array()?;
                if None == array {
                    return Ok(Self(Value::None));
                }
                else {
                    let mut array = array.unwrap().to_owned();
                    if array.len() - 1 < index as usize {
                        return Err(SurrealResponseError::OutOfRange)
                    }

                    let value = array.remove(index as usize);
                    return Ok(Self(value));
                }
            },
            QlPath::Field(field) => {
                let object = self.object()?;
                if None == object {
                    return Ok(Self(Value::None));
                }
                else {
                    let object = object.unwrap();
                    let value = object.get(field);
                    if None == value {
                        return Ok(Self(Value::None));
                    }
                    else {
                       return Ok(Self(value.unwrap().to_owned()));
                    }
                }
            },
        }
    }

    pub fn object(self) -> Result<Option<Object>, SurrealResponseError> {
        match self.0 {
            Value::None => Ok(None),
            Value::Object(value) => Ok(Some(value)),
            Value::Array(value) => {
                if let Some(Value::Object(obj)) = value.0.first() {
                    Ok(Some(obj.clone()))
                } else {
                    Ok(None)
                }
            },
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
            _ => Err(SurrealResponseError::ExpectedANumberI64)
        }
    }

    pub fn as_f64(self) -> Result<f64, SurrealResponseError> {
        let number = self.number()?;
        match number {
            Some(Number::Float(value)) => Ok(value),
            _ => Err(SurrealResponseError::ExpectedANumberF64)
        }
    }
}

impl QueryResult<SurrealQR> for usize {
    fn query_result(self, response: &mut QueryResponse) -> surrealdb::Result<SurrealQR> {
        Ok(SurrealQR(response.take::<Value>(0)?))
    }

    fn stats(&self, _: &QueryResponse) -> Option<Stats> {
        None
    }
}
