use crate::maven::pom::{ArtifactId, GroupId, Version};

/// The Maven variant to parse poms
/// These structs is directly modelled after the XML because that is what strong-xml plugin requires
#[derive(PartialEq, Debug)]
pub struct Metadata {
    pub group_id: GroupId,
    pub artifact_id: ArtifactId,
    pub version: Version,
    pub versioning: Versioning,
}

#[derive(PartialEq, Debug)]
pub struct Versioning {
    pub snapshot: Snapshot,
    pub last_updated: LastUpdated,
    pub snapshot_versions: SnapshotVersions,
}

#[derive(PartialEq, Debug)]
pub struct Snapshot {
    pub timestamp: Timestamp,
    pub build_number: BuildNumber,
}

#[derive(PartialEq, Debug)]
pub struct SnapshotVersions {
    pub snapshot_versions: Vec<SnapshotVersion>,
}

#[derive(PartialEq, Debug)]
pub struct SnapshotVersion {
    pub classifier: Option<Classifier>,
    pub extension: Extension,
    pub value: Value,
    pub updated: Updated,
}

#[derive(PartialEq, Debug)]
pub struct Timestamp {
    pub value: String,
}

#[derive(PartialEq, Debug)]
pub struct BuildNumber {
    pub value: String,
}

#[derive(PartialEq, Debug)]
pub struct LastUpdated {
    pub value: String,
}

#[derive(PartialEq, Debug)]
pub struct Updated {
    pub value: String,
}

#[derive(PartialEq, Debug)]
pub struct Extension {
    pub value: String,
}

#[derive(PartialEq, Debug)]
pub struct Classifier {
    pub value: String,
}

#[derive(PartialEq, Debug)]
pub struct Value {
    pub value: String,
}
