use crate::xml::dom_parser::Node;

#[derive(Debug, Clone)]
pub struct Repository {
    pub releases: Option<RepositoryPolicy>,
    pub snapshots: Option<RepositoryPolicy>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub layout: String,
}

#[derive(Debug, Clone)]
pub struct RepositoryPolicy {
    pub enabled: bool,
    pub update_policy: Option<String>,
    pub checksum_policy: Option<String>,
}

pub fn get_repositories(element: Node) -> Vec<Repository> {
    let mut repositories = vec![];

    for child in element.children {
        match child.name.as_str() {
            "repository" => repositories.push(get_repository(child)),
            _ => {}
        }
    }
    repositories
}

fn get_repository(element: Node) -> Repository {
    let mut releases = None;
    let mut snapshots = None;
    let mut id = None;
    let mut name = None;
    let mut url = None;
    let mut layout = "default".to_owned();

    for child in element.children {
        match child.name.as_str() {
            "releases" => releases = Some(get_update_policy(child)),
            "snapshots" => snapshots = Some(get_update_policy(child)),
            "id" => id = child.text,
            "name" => name = child.text,
            "url" => url = child.text,
            "layout" => layout = child.text.unwrap_or("default".to_owned()),
            _ => {}
        }
    }
    Repository {
        releases,
        snapshots,
        id,
        name,
        url,
        layout,
    }
}

fn get_update_policy(element: Node) -> RepositoryPolicy {
    let mut enabled = true;
    let mut update_policy = None;
    let mut checksum_policy = None;

    for child in element.children {
        match child.name.as_str() {
            "enabled" => enabled = child.text.map(|b| b == "true").unwrap_or(true),
            "update_policy" => update_policy = child.text,
            "checksum_policy" => checksum_policy = child.text,
            _ => {}
        }
    }
    RepositoryPolicy {
        enabled,
        update_policy,
        checksum_policy,
    }
}
