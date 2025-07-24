use undeepend::xml::dom_parser::get_document;

#[test]
fn test_dom_parser() {
    let test_xml = include_str!("../maven/resources/pom.xml");
    let doc = get_document(test_xml).expect("failed to get document");
    println!("{:?}",doc);
}