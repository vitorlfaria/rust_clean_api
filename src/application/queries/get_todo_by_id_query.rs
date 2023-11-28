use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{infrastructure::data::db_context::in_memory_db::InMemoryDB, application::responses::{SingleTodoResponse, TodoData}};

pub async fn get_todo_by_id_query(
    Path(id): Path<Uuid>,
    State(db): State<InMemoryDB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = db.lock().await;

    if let Some(todo) = vec.iter().find(|todo| todo.id == Some(id.to_owned())) {
        let json_response = SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: todo.clone() },
        };
        return Ok((StatusCode::OK, Json(json_response)));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}
