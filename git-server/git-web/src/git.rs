use std::{
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
    process::{Command, ExitStatus},
};


use crate::rest::{CreateRepoRequest, CreateRepoResponse};

const REPO_PATH: &str = "/repo";
const GIT_CMD: &str = "/usr/bin/git";
const CHOWN_CMD: &str = "/usr/bin/chown";
const GIT_USER: &str = "git";
const GIT_GROUP: &str = "git";

const BARE_REPO: &str = "[core]
	repositoryformatversion = 0
	filemode = true
	bare = true
	logallrefupdates = true

";

pub fn create_repo(req: CreateRepoRequest) -> std::io::Result<CreateRepoResponse> {
    let name = req.name;
    let kind = req.kind;
    let dir = format!("{}/{}", REPO_PATH, &name);
    let method_name = "create_repo";
    tracing::info!(name, dir, method_name);
    fs::create_dir(&dir)?;
    let r = git_cmd("init", &dir)?;
    if !r.success() {
        tracing::info!(method_name, dir, "git init failure");
        return Err(std::io::Error::new(
            ErrorKind::Other,
            format!("can't execute git init for : {dir}"),
        ));
    }

   

    let config = format!("{dir}/.git/config");
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(config)?;
    file.write_all(BARE_REPO.as_bytes())?;

    tracing::info!(method_name, dir, "configured as bare repository");

    let r = chown_cmd(GIT_USER, GIT_GROUP,&dir)?;
    if !r.success() {
        tracing::info!(method_name, dir, "chown failure");
        return Err(std::io::Error::new(
            ErrorKind::Other,
            format!("can't chown git user for : {dir}"),
        ));
    }

    tracing::info!(method_name, dir, "chown command executed");


    tracing::info!(method_name, dir, "repo created");
    Ok(CreateRepoResponse { name: name.clone(), kind, commands: vec![format!("git clone ssh://git@localhost:7999/repo/{}", name)] })
}

fn git_cmd(what: &str, cwd: &str) -> std::io::Result<ExitStatus> {
    Command::new(GIT_CMD).arg(what).current_dir(cwd).status()
}

fn chown_cmd(user: &str, group: &str, dir: &str) -> std::io::Result<ExitStatus> {
    Command::new(CHOWN_CMD).arg("-R").arg(format!("{}:{}", user, group)).arg(dir).status()
}
