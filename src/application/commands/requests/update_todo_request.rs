use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateTodoRequest {
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
}
