use crate::config::Config;
use clap::ArgMatches;
use failure::{Error, Fail};
use regex::Regex;
use std::fs::DirBuilder;
use std::process::Command;

pub fn run(matches: &ArgMatches) -> Result<(), Error> {
    let repo = matches.value_of("repo").unwrap();
    let url = make_repository_url(repo)?;

    let config = Config::load_config()?;
    let dir_path = config.dir.unwrap();

    create_repos_dir(&dir_path)?;

    fetch_repo(&url, &dir_path)?;

    Ok(())
}
