use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use chrono::Duration;
use serde_json::Value;
use uuid::Uuid;

use crate::{infrastructure::data::db_context::in_memory_db::InMemoryDB, domain::entities::todo::Todo, application::responses::GenericResponse};

pub async fn create_lots_of_todos_command(
    State(db): State<InMemoryDB>
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let mut vec = db.lock().await;

    for i in 1..=1000 {
        let id = Uuid::new_v4();
        let datetime = chrono::Utc::now()
            .checked_sub_signed(Duration::hours(3))
            .unwrap();

        let todo = Todo {
            id: Some(id.to_string()),
            title: i.to_string(),
            content: "content".to_string(),
            completed: Some(false),
            createdAt: Some(datetime),
            updatedAt: Some(datetime)
        };
        vec.push(todo);
    }
    
    let json_response = GenericResponse {
        status: "success".to_string(),
        message: "All todos created successfully!".to_string()
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}
