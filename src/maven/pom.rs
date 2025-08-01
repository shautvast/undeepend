use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::LazyLock;

/// the maven object model

const HOME: LazyLock<String> = LazyLock::new(|| env::var("HOME").unwrap());

#[derive(PartialEq, Debug)]
pub struct Pom {
    pub parent: Option<Parent>,
    pub group_id: Option<String>,
    pub artifact_id: String,
    pub version: Option<String>,
    pub name: Option<String>,
    pub packaging: Option<String>,
    pub url: Option<String>,
    pub dependencies: Vec<Dependency>,
    pub dependency_management: Vec<Dependency>,
    pub properties: HashMap<String, String>,
    pub module_names: Vec<String>,
    pub modules: Vec<Pom>,
    pub directory: PathBuf,
}

impl Pom {}

#[derive(PartialEq, Debug)]
pub struct License {
    pub name: String,
    pub url: String,
    pub distribution: Option<String>,
}

#[derive(PartialEq, Debug)]
pub struct Parent {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String,
}

#[derive(PartialEq, Debug)]
pub struct Developer {
    pub id: Option<String>,
    pub name: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Dependency {
    pub group_id: String,
    pub artifact_id: String,
    pub version: Option<String>,
    // pub scope: Option<String>, // TODO need this?
}

impl Dependency {
    /// returns a relative path to the dependency location
    pub fn to_jar_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(self.group_id.replace(".", "/"));
        path.push(&self.artifact_id);
        let version = self.version.clone().unwrap_or_else(|| "latest".to_string());
        path.push(&version);
        path.push(format!("{}-{}.jar", &self.artifact_id, &version));
        path
        // why is the version (in the filename) wrong when I use PathBuf::set_extension("jar") ???
    }

    /// returns an absolute path based on the default maven localRepository location
    // useful?
    pub fn to_absolute_jar_path(&self) -> PathBuf {
        let mut absolute_path = PathBuf::from(HOME.as_str());
        absolute_path.push(".m2/repository");
        absolute_path.push(self.to_jar_path());
        absolute_path
    }
}

use std::fmt;

impl Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version = self.version.clone().unwrap_or_else(|| "latest".to_string());
        write!(
            f,
            "{}/{}/{}/{}-{}",
            self.group_id.replace(".","/"), self.artifact_id, version, self.artifact_id, version
        )
    }
}
