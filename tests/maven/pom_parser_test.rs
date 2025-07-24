use undeepend::maven::pom_parser::get_pom;
use undeepend::xml::dom_parser::get_document;

#[test]
fn test_pom_parser() {
    let test_xml = include_str!("../maven/resources/pom.xml");
    let pom = get_pom(test_xml).expect("failed to get document");
    assert_eq!("Mockito",pom.name);
    assert_eq!(Some("org.mockito".to_string()),pom.group_id);
    assert_eq!("mockito-core",pom.artifact_id);
    assert_eq!(Some("1.9.5".to_string()),pom.version);
    assert_eq!(Some("jar".to_string()),pom.packaging);
    assert_eq!(Some("http://www.mockito.org".to_string()),pom.url);
    assert_eq!(2, pom.properties.len());
    assert_eq!("17", pom.properties["maven.compiler.source"]);
    assert_eq!("21", pom.properties["maven.compiler.target"]);
}
