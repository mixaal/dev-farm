use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum RepoType {
    PYTHON,
    RUST,
    C,
}

#[derive(Debug, Deserialize)]
pub struct CreateRepoRequest {
    pub name: String,
    pub kind: RepoType,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateRepoResponse {
    pub name: String,
    pub kind: RepoType,
    pub commands: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AppError {
    pub error: String,
}
