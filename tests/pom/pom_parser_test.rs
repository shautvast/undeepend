use undeepend::maven::pom_reader::get_pom;
use undeepend::xml::dom_parser::get_document;

#[test]
fn test_pom_parser() {
    let test_xml = include_str!("../pom/resources/pom.xml");
    let pom = get_pom(test_xml).expect("failed to get document");
    println!("{:?}", pom);
}
