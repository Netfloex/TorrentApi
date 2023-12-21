use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct IntScalar<T>(T);

impl Serialize for IntScalar<u64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(*self.get())
    }
}

impl<'de> Deserialize<'de> for IntScalar<u64> {
    fn deserialize<D>(deserializer: D) -> Result<IntScalar<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Ok(IntScalar(value))
    }
}

impl Serialize for IntScalar<i64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(*self.get())
    }
}

impl<'de> Deserialize<'de> for IntScalar<i64> {
    fn deserialize<D>(deserializer: D) -> Result<IntScalar<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i64::deserialize(deserializer)?;
        Ok(IntScalar(value))
    }
}

impl<T> IntScalar<T> {
    pub fn get(&self) -> &T {
        &self.0
    }
    pub fn from(i: T) -> Self {
        Self(i)
    }
}

#[juniper::graphql_scalar(name = "u64")]
impl<S> GraphQLScalar for IntScalar<u64>
where
    S: juniper::ScalarValue,
{
    fn resolve(&self) -> juniper::Value {
        juniper::Value::scalar(self.0.to_string())
    }

    fn from_input_value(value: &juniper::InputValue) -> Option<IntScalar<u64>> {
        if let Some(val) = value.as_string_value() {
            if let Ok(parsed) = val.parse() {
                Some(IntScalar(parsed))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn from_str<'a>(value: juniper::ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}

#[juniper::graphql_scalar(name = "i64")]
impl<S> GraphQLScalar for IntScalar<i64>
where
    S: juniper::ScalarValue,
{
    fn resolve(&self) -> juniper::Value {
        juniper::Value::scalar(self.0.to_string())
    }

    fn from_input_value(value: &juniper::InputValue) -> Option<IntScalar<i64>> {
        if let Some(val) = value.as_string_value() {
            if let Ok(parsed) = val.parse() {
                Some(IntScalar(parsed))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn from_str<'a>(value: juniper::ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}
