use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::Duration;

use crate::{
    application::responses::{SingleTodoResponse, TodoData},
    domain::entities::todo::Todo,
    infrastructure::data::repositories::todo_repository::TodoRepository,
};

pub async fn create_todo_command(
    Json(mut body): Json<Todo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();

    if let Ok(todo) = repository.get_by_title(body.title.clone()).await {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with title: '{}' already exists", todo.title),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let datetime = chrono::Utc::now()
        .checked_sub_signed(Duration::hours(3))
        .unwrap();

    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = body.to_owned();

    let todo = repository.create(todo.clone()).await.unwrap()[0].clone();

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}
