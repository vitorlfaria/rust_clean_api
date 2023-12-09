use crate::{domain::entities::todo::Todo, infrastructure::data::db_context::surreal_context::DB};
use surrealdb::{error::Db::Thrown, Error};

pub struct TodoRepository {
    table: String,
}

impl TodoRepository {
    pub fn new() -> Self {
        TodoRepository {
            table: "todo".to_string(),
        }
    }

    pub async fn get_by_query(&self, query: String) -> Result<Vec<Todo>, Error> {
        let result = DB.query(query).await?.take(0)?;
        Ok(result)
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let records = DB.select(&self.table).await?;
        Ok(records)
    }

    pub async fn get_by_title(&self, title: String) -> Result<Todo, Error> {
        if let Some(record) = DB
            .query("SELECT * FROM todo WHERE title = $title")
            .bind(("title", title))
            .await?
            .take(0)?
        {
            return Ok(record);
        }

        let error = Error::Db(Thrown("Record not found".to_string()));
        Err(error)
    }

    pub async fn get_by_id(&self, id: String) -> Result<Todo, Error> {
        if let Some(record) = DB.select((&self.table, id)).await? {
            return Ok(record);
        }

        let error = Error::Db(Thrown("Record not found".to_string()));
        Err(error)
    }

    pub async fn create(&self, content: Todo) -> Result<Vec<Todo>, Error> {
        let record = DB.create(&self.table).content(content).await?;
        Ok(record)
    }

    pub async fn update(&self, id: String, content: Todo) -> Result<Todo, Error> {
        let record = DB
            .update((&self.table, id))
            .content(content)
            .await?
            .unwrap();
        Ok(record)
    }

    pub async fn delete(&self, id: &str) -> Result<Todo, Error> {
        let result = DB.delete((&self.table, id)).await?.unwrap();
        Ok(result)
    }
}
