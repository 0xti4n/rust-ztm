use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Constructor, Deserialize, Serialize, Clone)]
pub struct Hits(u64);

impl Hits {
    fn into_inner(self) -> u64 {
        self.0
    }
}
