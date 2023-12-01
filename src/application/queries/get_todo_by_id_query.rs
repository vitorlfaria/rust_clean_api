use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::infrastructure::data::repositories::todo_repository::TodoRepository;

pub async fn get_todo_by_id_query(
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();
    let id = &id.to_string();

    let result = repository.get_by_id(id).await;

    return Ok((StatusCode::OK, Json(result)));
}
