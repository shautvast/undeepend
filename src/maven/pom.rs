/// The Maven variant to parse poms
/// These structs is directly modelled after the XML because that is what strong-xml plugin requires
#[derive(PartialEq, Debug)]
pub struct Pom {
    pub(crate) parent: Option<Parent>,
    pub(crate) group_id: Option<String>,
    pub(crate) artifact_id: String,
    pub(crate) version: Option<String>,
    pub(crate) name: String,
    pub(crate) packaging: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) dependencies: Vec<Dependency>,
    pub(crate) dependency_management: Vec<Dependency>,
}

impl Pom {

}

#[derive(PartialEq, Debug)]
pub struct License {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) distribution: Option<String>,
}

#[derive(PartialEq, Debug)]
pub struct Parent {
    pub(crate) group_id: String,
    pub(crate) artifact_id: String,
    pub(crate) version: String,
}

#[derive(PartialEq, Debug)]
pub struct Developer {
    pub(crate) id: Option<String>,
    pub(crate) name: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Dependency {
    pub(crate) group_id: String,
    pub(crate) artifact_id: String,
    pub(crate) version: Option<String>,
}

#[cfg(test)]
mod test {

    use crate::maven::pom::Pom;

    #[test]
    fn parse_should_not_fail() {}
}
