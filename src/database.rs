use std::error::Error;

use crate::user::{self, User};
pub struct database {
    db: sled::Db,
}

impl database {
    pub fn new(db: sled::Db) -> database {
        database { db }
    }

    pub fn insert_user(&self, u: User) -> anyhow::Result<Option<sled::IVec>> {
        let s = u.str();
        Ok(self.db.insert(u.display_name, s.as_str())?)
    }

    pub fn get_default_user_id(&self) -> anyhow::Result<Option<sled::IVec>> {
        let default_user_id = self.db.get("default_user")?;
        Ok(default_user_id)
    }

    pub fn get_user(&self, id: &str) -> anyhow::Result<Option<sled::IVec>> {
        let user = self.db.get(id)?;
        Ok(user)
    }

    pub fn make_default_user(&self, id: &str) -> anyhow::Result<Option<sled::IVec>> {
        let default_user_id = self.db.insert("default_user", id)?;
        Ok(default_user_id)
    }
}
