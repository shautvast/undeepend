use crate::maven::pom::{Dependency, Pom};
use crate::maven::pom_parser::get_pom;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

static PROPERTY_EXPR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$\{(.+)}").unwrap());

/// Loads all poms from a given project directory.
/// A POM (project object model) is a description of the project to build written in XML.
/// It has modules which are also a pom.xml in a subdirectory of the project root
/// (nesting is in theory infinite, but in practice you'll have 2 or maybe 3 levels)
pub fn parse_project(project_dir: &Path) -> Result<Project, String> {
    if !project_dir.is_dir() {
        return Err(format!("{:?} is not a directory", project_dir));
    }

    let mut pom_file = project_dir.to_path_buf();
    pom_file.push(Path::new("pom.xml"));
    if !pom_file.exists() {
        return Err(format!(
            "Directory {} does not contain pom.xml",
            project_dir.to_str().unwrap()
        ));
    }

    let pom_file = fs::read_to_string(pom_file).map_err(|e| e.to_string())?;
    let mut root = get_pom(project_dir.to_path_buf(), pom_file).map_err(|e| e.to_string())?;

    resolve_modules(project_dir, &mut root);
    let project_home = project_dir.to_str().unwrap_or_else(|| "?").to_string(); //TODO unwrap can fail??

    Ok(Project { project_home, root })
}

// examines modules in pom and loads them
fn resolve_modules(project_dir: &Path, pom: &mut Pom) {
    let mut modules = pom
        .module_names
        .iter()
        .map(|module| read_module_pom(project_dir, module))
        .collect();
    for module in &mut modules {
        resolve_modules(project_dir, module);
    }
    pom.modules.append(&mut modules);
}

// loads module pom
fn read_module_pom(project_dir: &Path, module: &String) -> Pom {
    let mut module_dir = project_dir.to_path_buf();
    module_dir.push(Path::new(module));
    let mut module_file = module_dir.clone();
    module_file.push(Path::new("pom.xml"));
    let module_pom =
        fs::read_to_string(module_file).expect(format!("Cannot read file {}", module).as_str());

    get_pom(module_dir, module_pom).expect(format!("Cannot create module pom {}", module).as_str())
}

//main entry to project
//the (root) pom holds the child references to modules
#[derive(Debug)]
pub struct Project {
    pub project_home: String,
    pub root: Pom,
}

impl Project {
    /// get a list of dependencies for a pom in the project
    pub fn get_dependencies(&self, pom: &Pom) -> Vec<Dependency> {
        pom.dependencies
            .iter()
            .map(|dependency| {
                let version = self.get_version(pom, &dependency.group_id, &dependency.artifact_id);
                Dependency {
                    group_id: dependency.group_id.clone(),
                    artifact_id: dependency.artifact_id.clone(),
                    version,
                }
            })
            .collect()
    }

    // determining a version of a dependency can be done in different ways
    // 1. version element below dependency, containing the version
    // 2. version element below dependency, containing a property name that is declared in the pom, or a parent which contains the version
    // 3. there is no version. In that case in the pom hierarchy there must be a dependencyManagement element in which the version is set
    // 4. combination of 2 and 3. This is what I typically see in enterprise software. The root pom contains a list of version properties, so all versions are kept in the same place. Takes some diligence to maintain though.
    fn get_version(&self, pom: &Pom, group_id: &str, artifact_id: &str) -> Option<String> {
        pom.dependencies
            .iter()
            // find to dependency
            .find(|d| d.group_id == group_id && d.artifact_id == artifact_id)
            // extract the version
            .and_then(|d| d.version.clone())
            .or_else(|| {
                // version not set, try dependencyManagement
                self.collect_managed_dependencies(pom, group_id, artifact_id)
                    .iter()
                    .find(|d| d.version.is_some())
                    .and_then(|d| d.version.clone())
            })
            .and_then(|v| {
                if PROPERTY_EXPR.is_match(v.as_str()) {
                    let property_name = &PROPERTY_EXPR.captures(&v).unwrap()[1];
                    // search property in project hierarchy
                    self.get_property(pom, property_name).ok()
                } else {
                    Some(v)
                }
            })
    }

    // searches in managed_dependencies for dependencies
    fn collect_managed_dependencies<'a>(
        &self,
        pom: &'a Pom,
        group_id: &str,
        artifact_id: &str,
    ) -> Vec<Dependency> {
        fn collect<'a>(
            project: &'a Project,
            pom: &'a Pom,
            deps: &mut Vec<Dependency>,
            group_id: &str,
            artifact_id: &str,
        ) {
            deps.append(
                &mut pom
                    .dependency_management
                    .iter()
                    .filter(|d| d.group_id == group_id && d.artifact_id == artifact_id)
                    .map(|d| d.clone())
                    .collect::<Vec<Dependency>>(),
            );
            if let Some(parent) = &pom.parent {
                if let Some(parent_pom) = project.get_pom(&parent.group_id, &parent.artifact_id) {
                    collect(project, parent_pom, deps, group_id, artifact_id);
                }
            }
        }

        let mut dependencies = Vec::new();
        collect(self, pom, &mut dependencies, group_id, artifact_id);
        dependencies
    }

    // recursively searches a property going up the chain towards parents
    fn get_property(&self, pom: &Pom, name: &str) -> Result<String, String> {
        if pom.properties.contains_key(name) {
            pom.properties
                .get(name)
                .cloned()
                .ok_or(format!("Unknown property {}", name))
        } else if let Some(parent) = &pom.parent {
            if let Some(parent_pom) = self.get_pom(&parent.group_id, &parent.artifact_id) {
                self.get_property(parent_pom, name)
            } else {
                Err(format!("Unknown property {}", name))
            }
        } else {
            Err(format!("Unknown property {}", name))
        }
    }

    // look up a pom in the project
    fn get_pom<'a>(&'a self, group_id: &str, artifact_id: &str) -> Option<&'a Pom> {
        // inner function to match poms (by artifactId and groupId)
        // (extract if needed elsewhere)
        fn is_same(pom: &Pom, group_id: &str, artifact_id: &str) -> bool {
            if pom.artifact_id == artifact_id {
                if let Some(pom_group_id) = &pom.group_id {
                    pom_group_id == group_id
                } else {
                    false
                }
            } else {
                false
            }
        }

        // inner function for recursion
        fn get_project_pom<'a>(pom: &'a Pom, group_id: &str, artifact_id: &str) -> Option<&'a Pom> {
            if is_same(pom, group_id, artifact_id) {
                return Some(pom);
            } else {
                for module in &pom.modules {
                    return get_project_pom(module, group_id, artifact_id);
                }
            }
            None
        }

        get_project_pom(&self.root, group_id, artifact_id)
    }
}

pub struct PomIterator<'a> {
    project: &'a Project,
    idx: usize,
}

impl<'a> PomIterator<'a> {
    pub fn new(project: &'a Project) -> Self {
        PomIterator {
            project,
            idx: 0,
        }
    }
}

impl<'a> Iterator for PomIterator<'a> {
    type Item = &'a Pom;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.project.root.modules.len() {
            let module = &self.project.root.modules[self.idx];
            self.idx += 1;
            Some(module)
        } else {
            None
        }
    }
}