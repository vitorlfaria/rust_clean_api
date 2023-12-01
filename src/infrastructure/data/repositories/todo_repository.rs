use surrealdb::{Error, engine::remote::ws::Ws, opt::auth::Root};
use crate::{domain::entities::todo::Todo, infrastructure::data::db_context::surreal_context::DB};

pub struct TodoRepository {
    table: String
}

impl TodoRepository {
    pub fn new() -> Self {
        TodoRepository { table: "todo".to_string() }
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let records = DB.select(&self.table).await?;
        Ok(records)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Todo, Error> {
        let _ = DB.connect::<Ws>("localhost:8000").await;
        let _ = DB.signin(Root {
            username: "root",
            password: "root",
        })
        .await;
        let _ = DB.use_ns("todo").use_db("todo").await;
        let record = DB.select((&self.table, id)).await?.unwrap();
        Ok(record)
    }

    pub async fn create(&self, content: Todo) -> Result<Vec<Todo>, Error> {
        let record = DB.create(&self.table).content(content).await?;
        Ok(record)
    }

    pub async fn update(&self, id: &str, content: Todo) -> Result<Todo, Error> {
        let record = DB.update((&self.table, id)).content(content).await?.unwrap();
        Ok(record)
    }

    pub async fn delete(&self, id: &str) -> Result<Todo, Error> {
        let result = DB.delete((&self.table, id)).await?.unwrap();
        Ok(result)
    }
}
