use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use chrono::Local;

use crate::{
    infrastructure::data::repositories::todo_repository::TodoRepository, domain::models::todo::Todo,
};

use super::requests::update_todo_request::UpdateTodoRequest;


pub async fn update_todo_command(
    Path(id): Path<String>,
    Json(body): Json<UpdateTodoRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();

    match repository.get_by_id(id.clone()).await {
        Ok(todo) => {
            let datetime = Local::now();
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

            let todo_response = repository.update_todo(id, payload).await.unwrap();

            let json_response = serde_json::json!({
                "status": "success",
                "data": todo_response,
            });

            Ok((StatusCode::OK, Json(json_response)))
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Todo with ID: {} not found", id)
            });

            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}
