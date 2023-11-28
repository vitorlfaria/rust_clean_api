use axum::{extract::{Path, State}, response::IntoResponse, http::StatusCode, Json};
use uuid::Uuid;

use crate::infrastructure::data::db_context::in_memory_db::InMemoryDB;


pub async fn delete_todo_command(
    Path(id): Path<Uuid>,
    State(db): State<InMemoryDB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(pos) = vec.iter().position(|todo| todo.id == Some(id.clone())) {
        vec.remove(pos);
        return Ok((StatusCode::NO_CONTENT, Json("")));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}
