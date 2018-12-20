mod cmd_cd;
mod cmd_clone;
mod cmd_config;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

pub fn run() {
    let matches = build_app().get_matches();

    if let Some(_) = matches.value_of("repo") {
        cmd_clone::run(&matches);
        return;
    }

    match matches.subcommand() {
        ("cd", Some(_)) => cmd_cd::run(),
        ("config", Some(_)) => cmd_config::run(),
        _ => build_app().print_help().expect("faild print help"),
    }
}

fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("repo").help("git repository name"))
        .subcommand(SubCommand::with_name("cd").about("go to clone directory"))
        .subcommand(SubCommand::with_name("config").about("open config file"))
}
