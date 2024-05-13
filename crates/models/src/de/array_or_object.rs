use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

// Deserializes a JavaScript array or object
// into a HashMap<usize, T>. Sometimes we get
// both types in a response, so it's necessary
// to ensure we use a HashMap<usize, T>.
pub fn array_or_object<'de, D, T>(
    deserializer: D,
) -> Result<HashMap<usize, T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Clone,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum ArrayOrObject<T> {
        Array(Vec<T>),
        Object(HashMap<usize, T>),
    }

    match ArrayOrObject::<T>::deserialize(deserializer)? {
        ArrayOrObject::Object(o) => Ok(o),
        ArrayOrObject::Array(a) => Ok(a.iter().cloned().enumerate().collect()),
    }
}
