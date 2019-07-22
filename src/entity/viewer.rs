use crate::entity::{Avatar, Username, Profile};
use crate::storage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Viewer {
    pub profile: Profile,
    pub auth_token: String
}

impl Viewer {
    pub fn username(&self) -> &Username {
        &self.profile.username
    }

    pub fn avatar(&self) -> &Avatar {
        &self.profile.avatar
    }

    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    pub fn store(&self) {
        storage::store_viewer(self);
    }
}