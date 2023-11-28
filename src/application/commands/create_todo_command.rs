use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use chrono::Duration;
use uuid::Uuid;

use crate::{infrastructure::data::db_context::in_memory_db::InMemoryDB, domain::entities::todo::Todo, application::responses::{SingleTodoResponse, TodoData}};


pub async fn create_todo_command(
    State(db): State<InMemoryDB>,
    Json(mut body): Json<Todo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    if let Some(todo) = vec.iter().find(|todo| todo.title == body.title) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with title: '{}' already exists", todo.title),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now()
        .checked_sub_signed(Duration::hours(3))
        .unwrap();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    let todo = body.to_owned();

    vec.push(body);

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo },
    };

    Ok((StatusCode::CREATED, Json(json_response)))
}
