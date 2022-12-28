use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug,Clone, Eq, Hash, PartialEq,Serialize,Deserialize)]
pub struct StoredURL {
    pub id: UUID,
    pub https_addres: string,
}

impl std::fmt::Display for StoredUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.https_addres)
    }
}