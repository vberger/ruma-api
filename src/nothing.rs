use serde::{Serializer, Serialize, Deserialize, Deserializer};

/// A struct representing the absence of content.
///
/// It should be used for type parameters of the `Endpoint` trait
/// when there is no content associated to it.
///
/// Contrarily to `()`, this struct has special implementations of
/// serde traits, so that it is garanteed to be serialized as `"{}"` in JSON
/// and `""` in url-encoding, which is required by the rest of the API.
#[derive(Copy,Clone,Debug)]
pub struct Nothing;

impl Deserialize for Nothing {
    fn deserialize<D>(deserializer: &mut D) -> Result<Nothing, D::Error> where D: Deserializer {
        struct Visitor;
        impl ::serde::de::Visitor for Visitor {
            type Value = Nothing;
            #[inline]
            fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Nothing, V::Error>
                where V: ::serde::de::SeqVisitor
            {
                visitor.end()?;
                Ok(Nothing{})
            }
            #[inline]
            fn visit_map<V>(&mut self, mut visitor: V) -> Result<Nothing, V::Error>
                where V: ::serde::de::MapVisitor
            {
                visitor.end()?;
                Ok(Nothing{})
            }
        }
        const FIELDS: &'static [&'static str] = &[];
        deserializer.deserialize_struct("Nothing", FIELDS, Visitor)
    }
}

impl Serialize for Nothing {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        let state = serializer.serialize_struct("nothing", 0)?;
        serializer.serialize_struct_end(state)
    }
}
