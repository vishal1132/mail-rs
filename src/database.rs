use crate::user::User;
pub struct Database {
    db: sled::Db,
}

impl Database {
    pub fn new(db: sled::Db) -> Database {
        Database { db }
    }

    pub fn insert_user(&self, u: User) -> anyhow::Result<Option<sled::IVec>> {
        let s = u.str();
        Ok(self
            .db
            .insert(format!("user_{}", u.display_name), s.as_str())?)
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
        let default_user_id = self
            .db
            .insert("default_user", format!("user_{}", id).as_str())?;
        Ok(default_user_id)
    }

    pub fn get_users(&self) -> anyhow::Result<Vec<sled::IVec>> {
        let users: Vec<sled::IVec> = self
            .db
            .scan_prefix("user")
            .values()
            .map(|x| x.unwrap())
            .collect();
        Ok(users)
    }

    pub fn clear(&self) -> anyhow::Result<()> {
        self.db.clear()?;
        Ok(())
    }
}
