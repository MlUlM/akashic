use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotSaveRequest<T> {
    pub snapshot: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<isize>,
}