use regex::Regex;

use crate::maven::pom::{Dependency, Parent, Pom};

/// offers a (non-mutable) view on the pom-as-xml-representation
/// the main use of this is that it resolves the parent information when needed
///

#[derive(Debug)]
pub struct Artifact {
    pub group: String,
    pub name: String,
    pub version: String,
    pub path: String,
}

impl Artifact {
    pub fn new(group: &str, name: &str, version: &str) -> Self {
        Self {
            group: group.into(),
            name: name.into(),
            version: version.into(),
            path: format!("{}/{}/{}", group.replace(".", "/"), name, version),
        }
    }

    pub fn is_snapshot(&self) -> bool {
        self.version.ends_with("-SNAPSHOT")
    }
}
