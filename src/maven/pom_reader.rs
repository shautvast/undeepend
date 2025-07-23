use crate::maven::pom::{Dependency, Developer, Parent, Pom};
use crate::xml::SaxError;
use crate::xml::dom_parser::{Node, get_document};

pub fn get_pom(xml: &str) -> Result<Pom, SaxError> {
    let mut group_id = None;
    let mut artefact_id = None;
    let mut parent = None;
    let mut version = None;
    let mut name = None;
    let mut packaging = None;
    let mut url = None;
    let mut dependencies = vec![];
    let mut dependency_management = vec![];

    for child in get_document(xml)?.root.children {
        match child.name.as_str() {
            "groupId" => group_id = child.text,
            "artifactId" => artefact_id = child.text,
            "parent" => parent = Some(get_parent(&child)),
            "version" => version = child.text,
            "name" => name = child.text,
            "packaging" => packaging = child.text,
            "url" => url = child.text,
            "dependencies" => dependencies = get_dependencies(child),
            "dependencyManagement" => dependency_management = get_dependency_mgmt(child),
            _ => {}
        }
    }
    Ok(Pom {
        parent,
        group_id,
        artifact_id: artefact_id.unwrap(),
        version,
        name: name.unwrap(),
        packaging,
        url,
        dependencies,
        dependency_management
    })
}

fn get_dependency_mgmt(element: Node) -> Vec<Dependency> {
    if !element.children.is_empty(){
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
    let mut artefact_id = None;
    let mut version = None;
    for node in element.children {
        match node.name.as_str() {
            "groupId" => grouo_id = node.text,
            "artifactId" => artefact_id = node.text,
            "version" => version = node.text,
            _ => {}
        }
    }
    Dependency {
        group_id: grouo_id.unwrap(),
        artifact_id: artefact_id.unwrap(),
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
    let mut artefact_id = None;
    let mut version = None;
    for child in &element.children {
        match child.name.as_str() {
            "groupId" => group_id = child.text.clone(),
            "artefactId" => artefact_id = child.text.clone(),
            "version" => version = child.text.clone(),
            _ => {}
        }
    }
    Parent {
        group_id: group_id.unwrap(),
        artifact_id: artefact_id.unwrap(),
        version: version.unwrap(),
    }
}

struct PomReader {
    element_stack: Vec<String>,
    pom: Pom,
}
