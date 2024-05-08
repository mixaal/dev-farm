use std::{
    fs::{self, OpenOptions},
    io::Write,
    process::{Command, ExitStatus, Stdio},
};

use rand::distributions::{Alphanumeric, DistString};

const ADD_USER: &str = "/usr/sbin/adduser";
const ADD_GROUP: &str = "/usr/sbin/addgroup";
const DEL_GROUP: &str = "/usr/sbin/delgroup";
const CHPASSWD: &str = "/usr/sbin/chpasswd";
const GIT_SHELL: &str = "/usr/bin/git-shell";
const CHMOD: &str = "/bin/chmod";
const CHOWN: &str = "/usr/bin/chown";

pub fn add_user(user: &str, home_dir: &str) -> std::io::Result<ExitStatus> {
    tracing::info!(user, home_dir, "add_user");
    Command::new(ADD_USER)
        .arg("-S")
        .arg(user)
        .arg("-h")
        .arg(home_dir)
        .arg("-s")
        .arg(GIT_SHELL)
        .status()
}

pub fn add_group(group: &str) -> std::io::Result<ExitStatus> {
    tracing::info!(group, "add_group");
    Command::new(ADD_GROUP).arg(group).status()
}

pub fn add_user_to_group(user: &str, group: &str) -> std::io::Result<ExitStatus> {
    tracing::info!(user, group, "add_user_to_group");
    Command::new(ADD_GROUP).arg(user).arg(group).status()
}

pub fn del_user_from_group(user: &str, group: &str) -> std::io::Result<ExitStatus> {
    tracing::info!(user, group, "del_user_from_group");
    Command::new(DEL_GROUP).arg(user).arg(group).status()
}

pub fn mkdir(dir: &str) -> std::io::Result<()> {
    tracing::info!(dir, "mkdir");
    fs::create_dir(dir)?;
    Ok(())
}

pub fn chmod(dir: &str, mode: &str) -> std::io::Result<ExitStatus> {
    tracing::info!(dir, mode, "chmod");
    Command::new(CHMOD).arg(mode).arg(dir).status()
}

pub fn chown(user: &str, group: &str, dir: &str) -> std::io::Result<ExitStatus> {
    tracing::info!(user, group, dir, "chown");
    Command::new(CHOWN)
        .arg("-R")
        .arg(format!("{}:{}", user, group))
        .arg(dir)
        .status()
}

pub fn save(file_name: &str, content: String) -> std::io::Result<()> {
    tracing::info!(file_name, content, "save start");
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_name)?;
    tracing::trace!(file_name, content, "file opened");
    let _ = f.write_all(content.as_bytes());
    tracing::trace!(file_name, content, "content written");
    Ok(())
}

pub fn set_random_passwd(user: &str) -> std::io::Result<ExitStatus> {
    let passwd = Alphanumeric.sample_string(&mut rand::thread_rng(), 13);
    set_passwd(user, &passwd)
}

pub fn set_passwd(user: &str, passwd: &str) -> std::io::Result<ExitStatus> {
    tracing::info!(user, passwd, "set_passwd start");
    let mut child = Command::new(CHPASSWD)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()?;
    tracing::info!(user, passwd, "set_passwd spawned");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    tracing::info!(user, passwd, "set_passwd took stdin");
    let chpasswd = format!("{user}:{passwd}");

    let _ = stdin.write_all(chpasswd.as_bytes());
    tracing::info!(user, passwd, "set_passwd write to stdin");
    let r = child.wait();
    tracing::info!(user, passwd, "set_passwd finished");
    r
}
