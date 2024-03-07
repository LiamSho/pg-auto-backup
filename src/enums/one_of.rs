use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOf<A, B> {
    TypeA(A),
    TypeB(B),
}
