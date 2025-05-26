use std::sync::{Arc, Mutex};
use sqlx::{Pool, Sqlite};

pub struct DB {
    name: Arc<Mutex<Option<String>>>,
    db : Option<Pool<Sqlite>>
}

impl DB {
    pub fn new() -> Self {
        Self {
            name: Arc::new(Mutex::new(None)),
            db: None
        }
    }
}