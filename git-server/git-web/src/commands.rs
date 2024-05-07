use std::{
    fs::{self, OpenOptions},
    io::Write,
    process::{Command, ExitStatus},
};

const ADD_USER: &str = "/usr/sbin/adduser";
const ADD_GROUP: &str = "/usr/bin/addgroup";
const MKDIR: &str = "/bin/mkdir";
const GIT_SHELL: &str = "/usr/bin/git-shell";
const CHMOD: &str = "/bin/chmod";
const CHOWN_CMD: &str = "/usr/bin/chown";

pub fn add_user(user: &str, home_dir: &str) -> std::io::Result<ExitStatus> {
    Command::new(ADD_USER)
        .arg(user)
        .arg("-h")
        .arg(home_dir)
        .arg("-s")
        .arg(GIT_SHELL)
        .status()
}

pub fn add_group(group: &str) -> std::io::Result<ExitStatus> {
    Command::new(ADD_GROUP).arg(group).status()
}

pub fn add_user_to_group(user: &str, group: &str) -> std::io::Result<ExitStatus> {
    Command::new(ADD_USER)
        .arg(user)
        .arg("-G")
        .arg(group)
        .status()
}

pub fn mkdir(dir: &str) -> std::io::Result<()> {
    fs::create_dir(dir)?;
    Ok(())
}

pub fn chmod(dir: &str, mode: &str) -> std::io::Result<ExitStatus> {
    Command::new(CHMOD).arg(mode).arg(dir).status()
}

pub fn chown_cmd(user: &str, group: &str, dir: &str) -> std::io::Result<ExitStatus> {
    Command::new(CHOWN_CMD)
        .arg("-R")
        .arg(format!("{}:{}", user, group))
        .arg(dir)
        .status()
}

pub fn save(file_name: &str, content: String) -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_name)?;
    let _ = f.write_all(content.as_bytes());
    Ok(())
}
