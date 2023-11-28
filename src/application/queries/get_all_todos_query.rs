use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};

use crate::{
    response::TodoListResponse, domain::entities::todo::Todo, infrastructure::data::db_context::in_memory_db::InMemoryDB, application::queries::query_options::QueryOptions,
};

pub async fn get_all_todos_query(
    opts: Option<Query<QueryOptions>>,
    State(db): State<InMemoryDB>,
) -> impl IntoResponse {
    let todos = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };

    Json(json_response)
}
