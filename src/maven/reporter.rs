use crate::maven::project::Project;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use std::sync::LazyLock;
use zip::ZipArchive;

static CLASS_EXPR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.+)/.+\.class").unwrap());

pub fn report(project: &Project) {
    let pom = &project.root;
    for dep in &project.get_dependencies(pom) {
        println!("{:?}", dep);
        //TODO all modules
        let jar_file = File::open(dep.to_absolute_path()).expect("Can't open jar file");
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

        let mut src_main_java = pom.directory.clone();
        src_main_java.push("src/main/java"); //TODO other src directories

        traverse(&packages, &src_main_java);

        let mut src_test_java = pom.directory.clone();
        src_test_java.push("src/test/java"); //TODO other src directories

        traverse(&packages, &src_test_java);
    }
}

fn traverse(packages: &HashSet<String>, dir: &Path) {
    if dir.exists() {
        for entry in dir.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                traverse(packages, &path);
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
