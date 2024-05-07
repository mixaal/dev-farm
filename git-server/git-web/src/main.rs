use std::fs;

use git_web::{
    commands::mkdir,
    controller::create_project,
    git::create_repo,
    http_utils::{http_response, io_error, parse_error},
    rest::{AppError, CreateProjectRequest, CreateRepoRequest, CreateRepoResponse, RepoType},
};
use salvo::prelude::*;

#[handler]
async fn create_new_repository(req: &mut Request, res: &mut Response) {
    let r: Result<CreateRepoRequest, salvo::http::ParseError> =
        req.parse_json::<CreateRepoRequest>().await;
    if r.is_err() {
        let err = r.err().unwrap();
        parse_error(res, err);
    } else {
        let r = r.unwrap();
        let resp = create_repo(r);
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
    let r: Result<CreateProjectRequest, salvo::http::ParseError> =
        req.parse_json::<CreateProjectRequest>().await;
    if r.is_err() {
        let err = r.err().unwrap();
        parse_error(res, err);
    } else {
        let r = r.unwrap();

        let resp = create_project(r);
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
    tracing_subscriber::fmt().init();

    let router = Router::new().post(create_new_repository);
    let acceptor = TcpListener::new("0.0.0.0:7998").bind().await;
    Server::new(acceptor).serve(router).await;
}
