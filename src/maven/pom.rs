use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// the maven object model

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

impl Pom {

}

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
}