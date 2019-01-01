mod cmd_clone;

use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};
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
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ColoredHelp)
        .arg(Arg::with_name("repo").help("git repository name"))
}
