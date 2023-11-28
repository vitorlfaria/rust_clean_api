use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{infrastructure::data::interfaces::idatabase::IDatabase, domain::entities::todo::Todo};

pub type InMemoryDB = Arc<Mutex<Vec<Todo>>>;

impl IDatabase for InMemoryDB {
    fn connect() -> Self {
        Arc::new(Mutex::new(Vec::new()))
    }
}
