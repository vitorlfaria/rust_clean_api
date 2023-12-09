use axum::{response::IntoResponse, Json};

use crate::{
    application::responses::TodoListResponse, domain::entities::todo::Todo,
    infrastructure::data::repositories::todo_repository::TodoRepository,
};

pub async fn get_all_todos_query() -> impl IntoResponse {
    let repository = TodoRepository::new();

    let mut todos: Vec<Todo> = Vec::new();
    if let Ok(result) = repository.get_all().await {
        todos = result;
    }

    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };

    Json(json_response)
}
