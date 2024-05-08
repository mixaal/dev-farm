use git_web::{
    controller, git,
    http_utils::{http_response, io_error, parse_error},
    rest::{
        AssignUserToProjectRequest, CreateProjectRequest, CreateRepoRequest, CreateUserRequest,
    },
};
use salvo::prelude::*;

#[handler]
async fn create_new_repository(req: &mut Request, res: &mut Response) {
    let r = req.parse_json::<CreateRepoRequest>().await;
    if r.is_err() {
        let err = r.err().unwrap();
        parse_error(res, err);
    } else {
        let r = r.unwrap();
        let resp = git::create_repo(r);
        if resp.is_err() {
            let err = resp.err().unwrap();
            io_error(res, err);
        } else {
            http_response(res, StatusCode::CREATED, &resp.unwrap());
        }
    }
}

#[handler]
async fn create_new_project(req: &mut Request, res: &mut Response) {
    let r = req.parse_json::<CreateProjectRequest>().await;
    if r.is_err() {
        let err = r.err().unwrap();
        parse_error(res, err);
    } else {
        let r = r.unwrap();

        let resp = controller::create_project(r);
        if resp.is_err() {
            let err = resp.err().unwrap();
            io_error(res, err);
        } else {
            http_response(res, StatusCode::CREATED, &resp.unwrap());
        }
    }
}

#[handler]
async fn create_user(req: &mut Request, res: &mut Response) {
    let r = req.parse_json::<CreateUserRequest>().await;
    if r.is_err() {
        let err = r.err().unwrap();
        parse_error(res, err);
    } else {
        let r = r.unwrap();

        let resp = controller::add_user(r);
        if resp.is_err() {
            let err = resp.err().unwrap();
            io_error(res, err);
        } else {
            http_response(res, StatusCode::CREATED, &resp.unwrap());
        }
    }
}

#[handler]
async fn assign_user_to_project(req: &mut Request, res: &mut Response) {
    let r = req.parse_json::<AssignUserToProjectRequest>().await;
    if r.is_err() {
        let err = r.err().unwrap();
        parse_error(res, err);
    } else {
        let r = r.unwrap();

        let resp = controller::add_user_to_project(r);
        if resp.is_err() {
            let err = resp.err().unwrap();
            io_error(res, err);
        } else {
            http_response(res, StatusCode::CREATED, &resp.unwrap());
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let repo_router = Router::with_path("<id>").post(create_new_repository);
    let project_router = Router::with_path("/projects")
        .post(create_new_project)
        .push(repo_router.push(Router::with_path("users").post(assign_user_to_project)));
    let users_router = Router::with_path("/users").post(create_user);
    let acceptor = TcpListener::new("0.0.0.0:7998").bind().await;
    let main_router = Router::new().push(project_router).push(users_router);
    tracing::info!("{}", format!("{:?}", main_router));
    Server::new(acceptor).serve(main_router).await;
}
