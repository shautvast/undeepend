use crate::maven::pom::Dependency;
use crate::maven::project::Project;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use zip::ZipArchive;

static CLASS_EXPR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.+)/.+\.class").unwrap());
const MAVEN_CENTRAL: &str = "https://repo1.maven.org/maven2/";

pub fn report(project: &Project) {
    let pom = &project.root;
    for dep in &project.get_dependencies(pom) {
        let path = PathBuf::from(dep.to_absolute_jar_path());
        if !path.exists() {
            download(dep).expect(&format!("Can't download jar file {}", dep));
        }
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

        let mut src_main_java = pom.directory.clone();
        src_main_java.push("src/main/java"); //TODO other src directories

        traverse(&packages, &src_main_java);

        let mut src_test_java = pom.directory.clone();
        src_test_java.push("src/test/java"); //TODO other src directories

        traverse(&packages, &src_test_java);
    }
}

use reqwest::blocking::Client;

fn download(dep: &Dependency) -> Result<(), String> {
    //TODO inspect settings.xml

    let url = format!("{}{}.jar", MAVEN_CENTRAL, dep);

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30)) // Ruime timeout instellen
        .build()
        .map_err(|e| e.to_string())?;

    println!("Downloading {}", &url);
    let response = client
        .get(&url)
        .header("User-Agent", "Maven/1.0") // Goede practice om een User-Agent te sturen
        .send()
        .map_err(|e| e.to_string())?;
    if response.status().is_success() {
        let bytes = response.bytes().map_err(|e| e.to_string())?;
        let mut buf_writer =
            BufWriter::new(File::create(dep.to_absolute_jar_path()).map_err(|e| e.to_string())?);

        buf_writer.write_all(&bytes).map_err(|e| e.to_string())?;
        buf_writer.flush().map_err(|e| e.to_string())?;
        println!("Downloaded {}", &url);
    }
    Ok(())
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
