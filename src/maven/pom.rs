/// The Maven variant to parse poms
/// These structs is directly modelled after the XML because that is what strong-xml plugin requires
#[derive(PartialEq, Debug)]
pub struct Pom {
    pub(crate) model_version: ModelVersion,
    pub(crate) parent: Option<Parent>,
    pub(crate) group_id: Option<GroupId>,
    pub(crate) artifact_id: ArtifactId,
    pub(crate) version: Option<Version>,
    pub(crate) name: Name,
    pub(crate) packaging: Option<Packaging>,
    pub(crate) url: Option<Url>,
    pub(crate) description: Description,
    pub(crate) licences: Option<Licenses>,
    pub(crate) scm: Option<Scm>,
    pub(crate) developers: Option<Developers>,
    pub(crate) dependencies: Option<Dependencies>,
    pub(crate) dependency_management: Option<DependencyManagement>,
}

#[derive(PartialEq, Debug)]
pub struct ModelVersion {
    pub value: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct GroupId {
    pub(crate) value: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ArtifactId {
    pub(crate) value: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Version {
    pub(crate) value: String,
}

#[derive(PartialEq, Debug)]
pub struct Name {
    pub(crate) value: String,
}

#[derive(PartialEq, Debug)]
pub struct Id {
    pub(crate) value: String,
}

#[derive(PartialEq, Debug)]
pub struct Packaging {
    pub(crate) value: String,
}

#[derive(PartialEq, Debug)]
pub struct Url {
    pub(crate) value: String,
}

#[derive(PartialEq, Debug)]
pub struct Description {
    pub(crate) value: String,
}

#[derive(PartialEq, Debug)]
pub struct Licenses {
    pub(crate) licenses: Vec<License>,
}

#[derive(PartialEq, Debug)]
pub struct Distribution {
    pub(crate) value: String,
}

#[derive(PartialEq, Debug)]
pub struct License {
    pub(crate) name: Name,
    pub(crate) url: Url,
    pub(crate) distribution: Option<Distribution>,
}

#[derive(PartialEq, Debug)]
pub struct Parent {
    pub(crate) group_id: GroupId,
    pub(crate) artifact_id: ArtifactId,
    pub(crate) version: Version,
}

#[derive(PartialEq, Debug)]
pub struct Scm {
    pub(crate) url: Url,
}

#[derive(PartialEq, Debug)]
pub struct Developers {
    pub(crate) developers: Vec<Developer>,
}

#[derive(PartialEq, Debug)]
struct Developer {
    pub(crate) id: Option<Id>,
    pub(crate) name: Name,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Dependencies {
    pub(crate) value: Vec<Dependency>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct DependencyManagement {
    pub(crate) value: Dependencies,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Dependency {
    pub(crate) group_id: GroupId,
    pub(crate) artifact_id: ArtifactId,
    pub(crate) version: Option<Version>,
}

#[cfg(test)]
mod test {

    use crate::maven::pom::Pom;

    #[test]
    fn parse_should_not_fail() {

    }
}
