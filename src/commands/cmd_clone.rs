use crate::config::Config;
use clap::ArgMatches;
use failure::{Error, Fail};
use regex::Regex;
use std::fs::DirBuilder;
use std::process::Command;

#[derive(Debug, Fail)]
#[fail(display = "Invalid repo")]
struct InvalidRepoError;

pub fn run(matches: &ArgMatches) -> Result<(), Error> {
    let repo = matches.value_of("repo").unwrap();
    let url = make_repository_url(repo)?;

    let config = Config::load_config()?;
    let dir_path = config.dir.unwrap();

    create_repos_dir(&dir_path)?;

    fetch_repo(&url, &dir_path)?;

    Ok(())
}

fn make_repository_url(repo: &str) -> Result<String, Error> {
    let url_pattern = Regex::new(r"https://[\w/:%#\$&\?\(\)~\.=\+\-]+")?;
    if url_pattern.is_match(repo) {
        return Ok(repo.to_string());
    }

    let repo_pattern = Regex::new(r".*/.*")?;
    if repo_pattern.is_match(repo) {
        let repo_url = format!("https://github.com/{}.git", repo);
        return Ok(repo_url);
    }

    Err(Error::from(InvalidRepoError))
}

fn fetch_repo(repo_url: &str, dir_path: &str) -> Result<(), Error> {
    let mut clone_process = Command::new("git")
        .current_dir(dir_path)
        .arg("clone")
        .arg(repo_url)
        .spawn()?;

    clone_process.wait()?;

    Ok(())
}
