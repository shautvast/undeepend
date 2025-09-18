use crate::maven::pom::{Dependency, Pom};
use crate::maven::project::Project;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;

use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use zip::ZipArchive;

static CLASS_EXPR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.+)/.+\.class").unwrap());

// TODO should not be downloading dependencies
pub fn report(project: &Project) {
    let pom = &project.root; // TODO other modules
    for dep in &project.get_dependencies(pom) {
        let jar_file = File::open(dep.to_absolute_jar_path()).expect("Can't open jar file");
        let mut archive = ZipArchive::new(jar_file).expect("Can't read jar file");

        let mut packages = HashSet::new();

        for i in 0..archive.len() {
            let file = archive.by_index(i).expect("Can't read file");
            let name = file.name();
            if CLASS_EXPR.is_match(name) {
                let package = &CLASS_EXPR.captures(name).unwrap()[1];
                packages.insert(package.replace("/", ".").to_string());
            }
        }

        analyse_source(&packages, &new_path(&pom.directory, "src/main/java"));
        analyse_source(&packages, &new_path(&pom.directory, "src/test/java")); //TODO other src dirs, generated src
    }
}

fn new_path(dir: &PathBuf, child: &str) -> PathBuf {
    let mut new_dir = dir.clone();
    new_dir.push(child);
    new_dir
}

fn analyse_source(packages: &HashSet<String>, dir: &Path) {
    if dir.exists() {
        for entry in dir.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                analyse_source(packages, &path);
            } else {
                if path.extension().unwrap() == "java" {
                    analyse(packages, &path);
                }
            }
        }
    }
}

// TODO deal with import wildcards
fn analyse(packages: &HashSet<String>, path: &Path) {
    let content = std::fs::read_to_string(path).unwrap();
    let lines = content.lines();
    for line in lines {
        if line.contains("import") {
            for package in packages {
                if line.contains(package) {
                    println!("{:?}: {}", path, line);
                }
            }
        }
    }
}
