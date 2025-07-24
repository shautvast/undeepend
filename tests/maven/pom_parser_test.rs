use undeepend::maven::pom_parser::get_pom;

#[test]
fn test_pom_parser_is_correct() {
    let test_xml = include_str!("../maven/resources/pom.xml");
    let pom = get_pom(test_xml).expect("failed to get document");
    assert_eq!(Some("Mockito".to_string()),pom.name);
    assert_eq!(Some("org.mockito".to_string()),pom.group_id);
    assert_eq!("mockito-core",pom.artifact_id);
    assert_eq!(Some("1.9.5".to_string()),pom.version);
    assert_eq!(Some("jar".to_string()),pom.packaging);
    assert_eq!(Some("http://www.mockito.org".to_string()),pom.url);

    assert_eq!(3, pom.properties.len());
    assert_eq!("17", pom.properties["maven.compiler.source"]);
    assert_eq!("21", pom.properties["maven.compiler.target"]);
    assert_eq!("1.1", pom.properties["hamcrest.version"]);

    assert_eq!(2, pom.dependencies.len());
    let hamcrest = &pom.dependencies[0];
    assert_eq!("org.hamcrest", hamcrest.group_id);
    assert_eq!("hamcrest-core", hamcrest.artifact_id);
    assert_eq!(None, hamcrest.version);

    let objenesis = &pom.dependencies[1];
    assert_eq!("org.objenesis", objenesis.group_id);
    assert_eq!("objenesis", objenesis.artifact_id);
    assert_eq!(Some("1.0".to_string()), objenesis.version);

    assert_eq!(2, pom.modules.len());
    assert_eq!("a", pom.module_names[0]);
    assert_eq!("b", pom.module_names[1]);

    assert_eq!(1, pom.dependency_management.len());
    let hamcrest = &pom.dependency_management[0];
    assert_eq!("org.hamcrest", hamcrest.group_id);
    assert_eq!("hamcrest-core", hamcrest.artifact_id);
    assert_eq!(Some("${hamcrest.version}".to_string()), hamcrest.version);
}
