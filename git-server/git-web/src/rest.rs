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

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateProjectResponse {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub nick: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub photo: Option<Vec<u8>>,
    pub ssh_pub_key: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub nick: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub photo: Option<Vec<u8>>,
}

#[derive(Debug, Deserialize)]
pub struct AssignUserToProjectRequest {
    pub nick: String,
    pub project: String,
}

#[derive(Debug, Serialize)]
pub struct AssignUserToProjectResponse {
    pub nick: String,
    pub project: String,
}

#[derive(Debug, Serialize)]
pub struct AppError {
    pub error: String,
}
