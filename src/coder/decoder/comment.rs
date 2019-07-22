use serde::Deserialize;
use crate::entity::{self, Credentials};
use crate::coder::decoder;
use std::convert::TryInto;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    id: usize,
    created_at: String,
    updated_at: String,
    body: String,
    author: decoder::Author,
}

impl Comment {
    pub fn try_into_comment<'a>(self, credentials: Option<Credentials>,) -> Result<entity::Comment<'a>, String> {
        let created_at = self.created_at.try_into()?;
        let updated_at = self.updated_at.try_into()?;

        Ok(entity::Comment {
            id: self.id.to_string().into(),
            body: self.body,
            created_at,
            updated_at,
            author: self.author.into_author(credentials),
        })
    }
}