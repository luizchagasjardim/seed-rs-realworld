use crate::entity::{Author, Timestamp};

#[derive(Clone)]
pub struct Comment {
    pub id: CommentId,
    pub body: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub author: Author,
}

#[derive(Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub struct CommentId(String);

impl CommentId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for CommentId {
    fn from(id: String) -> Self {
        Self(id)
    }
}
