mod cmd_clone;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
use colored::*;

pub fn run() {
    let matches = build_app().get_matches();

    if matches.value_of("repo").is_some() {
        cmd_clone::run(&matches).unwrap_or_else(|e| println!("{}", e.to_string().red().bold()));
        return;
    }

    build_app().print_help().expect("faild print help");
}

fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("repo").help("git repository name"))
        .subcommand(SubCommand::with_name("cd").about("go to clone directory"))
        .subcommand(SubCommand::with_name("config").about("open config file"))
}
