use crate::{
    commands,
    rest::{
        AssignUserToProjectRequest, AssignUserToProjectResponse, CreateProjectRequest,
        CreateProjectResponse, CreateUserRequest, CreateUserResponse,
    },
};

pub fn create_project(r: CreateProjectRequest) -> std::io::Result<CreateProjectResponse> {
    let meta = serde_json::to_string(&r).expect("can't serialize project");
    let path = format!("/projects/{}", r.name);
    commands::add_group(&r.name)?;
    commands::mkdir(&path)?;
    commands::chmod(&path, "0770")?;
    commands::chown("git", &r.name, &path)?;
    commands::save(&format!("{path}/.meta.json"), meta)?;
    Ok(CreateProjectResponse {
        name: r.name,
        description: r.description,
    })
}

pub fn add_user(r: CreateUserRequest) -> std::io::Result<CreateUserResponse> {
    let meta = serde_json::to_string(&r).expect("can't serialize user");
    let home_dir = format!("/users/{}", r.nick);
    commands::add_user(&r.nick, &home_dir)?;
    // commands::set_random_passwd(&r.nick)?;
    commands::save(&format!("{home_dir}/.meta.json"), meta)?;
    Ok(CreateUserResponse {
        nick: r.nick,
        name: r.name,
        email: r.email,
        photo: r.photo,
    })
}

pub fn add_user_to_project(
    r: AssignUserToProjectRequest,
) -> std::io::Result<AssignUserToProjectResponse> {
    commands::add_user_to_group(&r.nick, &r.project)?;

    Ok(AssignUserToProjectResponse {
        nick: r.nick,
        project: r.project,
    })
}
