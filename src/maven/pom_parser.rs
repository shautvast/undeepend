use crate::maven::common_model::get_repositories;
use crate::maven::pom::{Dependency, Developer, Parent, Pom};
use crate::xml::SaxError;
use crate::xml::dom_parser::{Node, get_document};
use std::collections::HashMap;
use std::path::PathBuf;

/// parse the pom.xml into a Pom object (struct)
pub fn get_pom(home_dir: PathBuf, xml: impl Into<String>) -> Result<Pom, SaxError> {
    let mut group_id = None;
    let mut artifact_id = None;
    let mut parent = None;
    let mut version = None;
    let mut name = None;
    let mut packaging = None;
    let mut url = None;
    let mut dependencies = vec![];
    let mut dependency_management = vec![];
    let mut properties = HashMap::new(); // useless assignments...
    let mut module_names = vec![]; // not useless assignment...
    let mut repositories = vec![]; // not useless assignment...

    for child in get_document(xml.into().as_str())?.root.children {
        match child.name.as_str() {
            "groupId" => group_id = child.text,
            "artifactId" => artifact_id = child.text,
            "parent" => parent = Some(get_parent(&child)),
            "version" => version = child.text,
            "name" => name = child.text,
            "packaging" => packaging = child.text,
            "url" => url = child.text,
            "dependencies" => dependencies = get_dependencies(child),
            "dependencyManagement" => dependency_management = get_dependency_mgmt(child),
            "properties" => properties = get_properties(child),
            "modules" => add_modules(child, &mut module_names),
            "repositories" => repositories = get_repositories(child),
            _ => {}
        }
    }

    // TODO before returning, calculate all
    // * dependency versions
    // * repositories
    // maybe put that in a separate model struct

    Ok(Pom {
        parent,
        group_id,
        artifact_id: artifact_id.unwrap(),
        version,
        name,
        packaging,
        url,
        dependencies,
        dependency_management,
        properties,
        module_names,
        modules: vec![],
        directory: home_dir,
        repositories,
    })
}

fn add_modules(element: Node, modules: &mut Vec<String>) {
    for module in element.children {
        modules.push(module.text.expect("Cannot read module name"));
    }
}

fn get_properties(element: Node) -> HashMap<String, String> {
    let mut properties = HashMap::new();
    for property in element.children {
        properties.insert(
            property.name.clone(),
            property
                .text
                .expect(format!("Cannot read property '{}'", property.name).as_str())
                .to_string(),
        );
    }
    properties
}

fn get_dependency_mgmt(element: Node) -> Vec<Dependency> {
    if !element.children.is_empty() {
        get_dependencies(element.children.first().unwrap().clone())
    } else {
        vec![]
    }
}

fn get_dependencies(element: Node) -> Vec<Dependency> {
    let mut dependencies = vec![];
    for node in element.children {
        if node.name == "dependency" {
            dependencies.push(get_dependency(node))
        }
    }
    dependencies
}

fn get_dependency(element: Node) -> Dependency {
    let mut grouo_id = None;
    let mut artifact_id = None;
    let mut version = None;
    for node in element.children {
        match node.name.as_str() {
            "groupId" => grouo_id = node.text,
            "artifactId" => artifact_id = node.text,
            "version" => version = node.text,
            _ => {}
        }
    }
    Dependency {
        group_id: grouo_id.unwrap(),
        artifact_id: artifact_id.unwrap(),
        version,
    }
}

fn get_developers(element: Node) -> Vec<Developer> {
    let mut developers = vec![];
    for node in element.children {
        if node.name == "developer" {
            developers.push(get_developer(node))
        }
    }
    developers
}

fn get_developer(element: Node) -> Developer {
    let mut id = None;
    let mut name = None;
    for node in element.children {
        match node.name.as_str() {
            "id" => id = node.text,
            "name" => name = node.text,
            _ => {}
        }
    }
    Developer {
        id,
        name: name.unwrap(),
    }
}

fn get_parent(element: &Node) -> Parent {
    let mut group_id = None;
    let mut artifact_id = None;
    let mut version = None;
    for child in &element.children {
        match child.name.as_str() {
            "groupId" => group_id = child.text.clone(),
            "artifactId" => artifact_id = child.text.clone(),
            "version" => version = child.text.clone(),
            _ => {}
        }
    }
    Parent {
        group_id: group_id.unwrap(),
        artifact_id: artifact_id.unwrap(),
        version: version.unwrap(),
    }
}

struct PomReader {
    element_stack: Vec<String>,
    pom: Pom,
}
