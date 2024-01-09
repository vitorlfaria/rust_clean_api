use axum::{extract::Path, response::IntoResponse, Json, http::StatusCode};

use crate::infrastructure::data::repositories::todo_repository::TodoRepository;

pub async fn get_todo_by_id_query(
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();
    let id = id.to_string();

    let todo = repository.get_by_id(id).await;

    return Ok((StatusCode::OK, Json(todo)));
}
