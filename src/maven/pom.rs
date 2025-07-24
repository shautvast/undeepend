use std::collections::HashMap;

/// The Maven variant to parse poms
/// These structs is directly modelled after the XML because that is what strong-xml plugin requires
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
    pub modules: Vec<String>,
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

#[cfg(test)]
mod test {

    use crate::maven::pom::Pom;

    #[test]
    fn parse_should_not_fail() {}
}
