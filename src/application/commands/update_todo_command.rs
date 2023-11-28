use axum::{extract::{Path, State}, Json, response::IntoResponse, http::StatusCode};
use chrono::Duration;
use uuid::Uuid;

use crate::{infrastructure::data::db_context::in_memory_db::InMemoryDB, domain::entities::todo::Todo, response::{SingleTodoResponse, TodoData}};

use super::update_todo_request::UpdateTodoRequest;


pub async fn update_todo_command(
    Path(id): Path<Uuid>,
    State(db): State<InMemoryDB>,
    Json(body): Json<UpdateTodoRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(todo) = vec.iter_mut().find(|todo| todo.id == Some(id.clone())) {
        let datetime = chrono::Utc::now()
            .checked_sub_signed(Duration::hours(3))
            .unwrap();
        let title = body.title.to_owned();
        let content = body.content.to_owned();
        let completed = body.completed.unwrap_or(todo.completed.unwrap());
        let payload = Todo {
            id: todo.id.to_owned(),
            title: if !title.is_empty() {
                title
            } else {
                todo.title.to_owned()
            },
            content: if !content.is_empty() {
                content
            } else {
                todo.content.to_owned()
            },
            completed: Some(completed),
            createdAt: todo.createdAt,
            updatedAt: Some(datetime),
        };
        *todo = payload;

        let json_response = SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: todo.clone() },
        };
        Ok((StatusCode::OK, Json(json_response)))
    } else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with ID: {} not found", id)
        });

        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}
