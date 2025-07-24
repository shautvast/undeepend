use std::path::Path;
use undeepend::maven::project::parse_project;

fn main() {
    let project = parse_project(Path::new("tests/maven/resources/sample_project")).unwrap();
    println!("{:?}", project.get_dependencies(&project.root));
}
