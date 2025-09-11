use std::path::PathBuf;
use std::{env, fs};
use undeepend::maven::project::parse_project;
use undeepend::maven::reporter::report;
use undeepend::maven::settings::get_settings;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let dir = if args.len() == 1 {
        env::current_dir().expect("Could not access current directory")
    } else {
        PathBuf::from(&args[1])
    };
    let project = parse_project(&dir).unwrap();
    get_settings().unwrap();
    fs::write(
        PathBuf::from("index.html"),
        project.generate_dependency_html(),
    )
    .unwrap();

    report(&project);
}
