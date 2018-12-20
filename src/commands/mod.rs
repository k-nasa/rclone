use clap::{crate_authors, crate_description, crate_name, crate_version, App};

pub fn run() {
    let matches = build_app().get_matches();
}

fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
}
