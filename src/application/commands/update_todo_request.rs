use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateTodoRequest {
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
}
