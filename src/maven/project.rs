use crate::maven::pom::{Dependency, Pom};
use crate::maven::pom_parser::get_pom;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

static PROPERTY_EXPR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$\{(.+)}").unwrap());

pub fn parse_project(project_dir: &Path) -> Result<Project, String> {
    if !project_dir.is_dir() {
        return Err(format!("{:?} is not a directory", project_dir));
    }

    let mut pom_file = project_dir.to_path_buf();
    pom_file.push(Path::new("pom.xml"));

    let pom_file = fs::read_to_string(pom_file).map_err(|e| e.to_string())?;
    let mut root = get_pom(pom_file).map_err(|e| e.to_string())?;

    resolve_modules(project_dir, &mut root);
    Ok(Project { root })
}

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

fn read_module_pom(project_dir: &Path, module: &String) -> Pom {
    let mut module_dir = project_dir.to_path_buf();
    module_dir.push(Path::new(module));
    let mut module_file = module_dir.clone();
    module_file.push(Path::new("pom.xml"));
    let module_pom =
        fs::read_to_string(module_file).expect(format!("Cannot read file {}", module).as_str());

    let mut pom =
        get_pom(module_pom).expect(format!("Cannot create module pom {}", module).as_str());
    pom.directory = module_dir;
    pom
}

#[derive(Debug)]
pub struct Project {
    pub root: Pom,
}

impl Project {
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

    fn get_version(&self, pom: &Pom, group_id: &str, artifact_id: &str) -> Option<String> {
        pom.dependencies
            .iter()
            .find(|d| d.group_id == group_id && d.artifact_id == artifact_id)
            .and_then(|d| d.version.clone())
            .and_then(|version| {
                if PROPERTY_EXPR.is_match(&version) {
                    let property_name = &PROPERTY_EXPR.captures(&version).unwrap()[1];
                    self.get_property(pom, property_name)
                } else {
                    Some(version)
                }
            })
            .or_else(|| {
                pom.dependency_management
                    .iter()
                    .find(|d| d.group_id == group_id && d.artifact_id == artifact_id)
                    .and_then(|d| d.version.clone())
                    .and_then(|version| {
                        if PROPERTY_EXPR.is_match(&version) {
                            let property_name = &PROPERTY_EXPR.captures(&version).unwrap()[1];
                            self.get_property(pom, property_name)
                        } else {
                            Some(version)
                        }
                    })
            })
    }

    fn get_property(&self, pom: &Pom, name: &str) -> Option<String> {
        if pom.properties.contains_key(name) {
            pom.properties.get(name).cloned()
        } else if let Some(parent) = &pom.parent {
            if let Some(parent_pom) = self.get_pom(&parent.group_id, &parent.artifact_id) {
                self.get_property(parent_pom, name)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_pom<'a>(&'a self, group_id: &str, artifact_id: &str) -> Option<&'a Pom> {
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
