use serde::{Serialize, ser::SerializeSeq};
use serde_json::Value;

#[derive(Debug)]
enum Items {
    Num(u32),
    Inner(Vec<Items>),
}

impl Serialize for Items {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        match self {
            Self::Num(n) => serializer.serialize_u32(*n),
            Self::Inner(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for element in v {
                    seq.serialize_element(element)?;
                }
                seq.end()
            }
        }
    }
}

#[derive(Debug)]
struct Packet(Vec<Items>);

impl Serialize for Packet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for element in &self.0 {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}