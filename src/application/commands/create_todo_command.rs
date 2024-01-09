use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::Local;

use crate::{domain::models::todo::Todo, infrastructure::data::repositories::todo_repository::TodoRepository};

pub async fn create_todo_command(
    Json(mut body): Json<Todo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();
    
    if let Ok(todo) = repository.get_by_title(body.title.clone()).await {
        let json_response = serde_json::json!({
            "status": "error".to_string(),
            "message": "Todo already exists".to_string(),
            "data": todo,
        });

        return Err((StatusCode::BAD_REQUEST, Json(json_response)));
    }

    let datetime = Local::now();
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = body.to_owned();

    let todo = repository.create_todo(todo.clone()).await.unwrap()[0].to_owned();

    let json_response = serde_json::json!({
        "status": "success".to_string(),
        "data": todo,
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}
