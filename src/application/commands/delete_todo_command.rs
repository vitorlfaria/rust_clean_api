use axum::{extract::Path, response::IntoResponse, http::StatusCode, Json};

use crate::infrastructure::data::repositories::todo_repository::TodoRepository;

pub async fn delete_todo_command(
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();
    let id = id.to_string();

    if let Ok(_) = repository.get_by_id(id.clone()).await {
        let _ = repository.delete_todo(id.clone()).await.unwrap();

        return Ok(StatusCode::NO_CONTENT);
    }

    let error_response = serde_json::json!({
        "status": "error",
        "message": format!("Todo with ID: {} not found", id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}