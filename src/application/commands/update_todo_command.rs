use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use chrono::Duration;

use crate::{
    application::responses::{SingleTodoResponse, TodoData},
    domain::entities::todo::Todo,
    infrastructure::data::repositories::todo_repository::TodoRepository,
};

use super::update_todo_request::UpdateTodoRequest;

pub async fn update_todo_command(
    Path(id): Path<String>,
    Json(body): Json<UpdateTodoRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();

    match repository.get_by_id(id.clone()).await {
        Ok(todo) => {
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

            let todo_response = repository.update(id, payload).await.unwrap();

            let json_response = SingleTodoResponse {
                status: "success".to_string(),
                data: TodoData {
                    todo: todo_response,
                },
            };

            Ok((StatusCode::OK, Json(json_response)))
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Todo with ID: {} not found", id)
            });

            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}
