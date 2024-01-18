use serde::ser::{Serialize, Serializer};

pub trait SerializeHashes<S: Serializer> {
    fn serialize_hashes(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

impl<S: Serializer> SerializeHashes<S> for Vec<String> {
    fn serialize_hashes(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.join("|").serialize(serializer)
    }
}

impl<S: Serializer> SerializeHashes<S> for Option<Vec<String>> {
    fn serialize_hashes(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            None => serializer.serialize_none(),
            Some(hashes) => hashes.join("|").serialize(serializer),
        }
    }
}
