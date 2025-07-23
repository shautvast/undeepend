use undeepend::xml::sax_parser::parse_string;
use undeepend::xml::{Attribute, SaxError, SaxHandler};

#[test]
fn test_xml_header() {
    let test_xml = include_str!("resources/header.xml");
    let mut testhandler = TestHandler::new();
    parse_string(test_xml.to_string(), Box::new(&mut testhandler))
        .expect("Failed to parse test xml");
    println!("{:?}", testhandler);
    assert_eq!(testhandler.start_document_called, 1);
    assert_eq!(testhandler.end_document_called, 1);
}

#[test]
fn test_single_element_short() {
    let test_xml = include_str!("resources/header.xml");
    let mut testhandler = TestHandler::new();
    parse_string(test_xml.to_string(), Box::new(&mut testhandler))
        .expect("Failed to parse test xml");
    assert_eq!(testhandler.start_document_called, 1);
    assert_eq!(testhandler.start_element_called, 1);
    assert!(!testhandler.elements.is_empty());
    assert_eq!(testhandler.elements[0], "<xml>");
    assert_eq!(testhandler.end_document_called, 1);
}

#[test]
fn test_single_element() {
    let test_xml = include_str!("resources/element.xml");
    let mut testhandler = TestHandler::new();
    parse_string(test_xml.to_string(), Box::new(&mut testhandler))
        .expect("Failed to parse test xml");
    assert_eq!(testhandler.start_document_called, 1);
    assert_eq!(testhandler.start_element_called, 1);
    assert!(!testhandler.elements.is_empty());
    assert_eq!(testhandler.elements[0], "<element>");
    assert_eq!(testhandler.end_document_called, 1);
}

#[test]
fn test_single_element_single_attribute() {
    let test_xml = include_str!("resources/element_with_attribute.xml");
    let mut testhandler = TestHandler::new();
    parse_string(test_xml.to_string(), Box::new(&mut testhandler))
        .expect("Failed to parse test xml");
    assert_eq!(testhandler.start_document_called, 1);
    assert_eq!(testhandler.start_element_called, 1);
    assert!(!testhandler.elements.is_empty());
    assert_eq!(testhandler.elements[0], r#"<element a="1">"#);
    assert_eq!(testhandler.end_element_called, 1);
    assert_eq!(testhandler.end_document_called, 1);
}

#[test]
fn test_ignore_comment() {
    let test_xml = include_str!("resources/comment.xml");
    let mut testhandler = TestHandler::new();
    parse_string(test_xml.to_string(), Box::new(&mut testhandler))
        .expect("Failed to parse test xml");
    assert_eq!(testhandler.start_document_called, 1);
    assert_eq!(testhandler.start_element_called, 1);
    assert!(!testhandler.elements.is_empty());
    assert_eq!(
        testhandler.elements[0],
        r#"<http://example.com/books:bookstore xmlns="http://example.com/books">"#
    );
    assert_eq!(testhandler.end_element_called, 1);
    assert_eq!(testhandler.end_document_called, 1);
}

#[test]
fn test_bad_comment() {
    let test_xml = include_str!("resources/illegal_dashes_comment.xml");
    let mut testhandler = TestHandler::new();
    match parse_string(test_xml.to_string(), Box::new(&mut testhandler)) {
        Err(e) => assert_eq!(e, SaxError::BadCharacter),
        Ok(_) => assert!(false),
    }
}

#[test]
fn test_namespaces() {
    let test_xml = include_str!("resources/namespaces.xml");
    let mut testhandler = TestHandler::new();
    parse_string(test_xml.to_string(), Box::new(&mut testhandler))
        .expect("Failed to parse test xml");
    assert_eq!(testhandler.start_document_called, 1);
    assert_eq!(testhandler.start_element_called, 4);
    assert!(!testhandler.elements.is_empty());
    assert_eq!(testhandler.elements[0], r#"<bookstore>"#);
    assert_eq!(
        testhandler.elements[1],
        r#"<http://example.com/books:book xmlns="http://example.com/books" id="1" category="fiction">"#
    );
    assert_eq!(
        testhandler.elements[2],
        r#"<http://example.com/books:page>"#
    );
    assert_eq!(testhandler.elements[3], r#"<publisher>"#);
    assert_eq!(testhandler.end_element_called, 4);
    assert_eq!(testhandler.end_document_called, 1);
}

#[test]
fn test_namespace_prefixes() {
    let test_xml = include_str!("resources/namespaces-prefix.xml");
    let mut testhandler = TestHandler::new();
    parse_string(test_xml.to_string(), Box::new(&mut testhandler))
        .expect("Failed to parse test xml");
    assert_eq!(testhandler.start_document_called, 1);
    assert_eq!(testhandler.start_element_called, 5);
    assert!(!testhandler.elements.is_empty());
    assert_eq!(testhandler.elements[0], r#"<bookstore>"#);
    assert_eq!(
        testhandler.elements[1],
        r#"<book xmlns:books="http://example.com/books" xmlns:covers="http://example.com/covers" id="1" category="fiction">"#
    );
    assert_eq!(
        testhandler.elements[2],
        r#"<http://example.com/books:page>"#
    );
    assert_eq!(testhandler.elements[3], r#"<http://example.com/covers:cover>"#);
    assert_eq!(testhandler.elements[4], r#"<publisher>"#);
    assert_eq!(testhandler.end_element_called, 5);
    assert_eq!(testhandler.end_document_called, 1);
}

#[derive(Debug)]
struct TestHandler {
    start_document_called: usize,
    end_document_called: usize,
    start_element_called: usize,
    end_element_called: usize,
    elements: Vec<String>,
}

impl TestHandler {
    pub fn new() -> Self {
        Self {
            start_document_called: 0,
            end_document_called: 0,
            start_element_called: 0,
            end_element_called: 0,
            elements: vec![],
        }
    }
}

impl SaxHandler for TestHandler {
    fn start_document(&mut self) {
        self.start_document_called += 1;
    }

    fn end_document(&mut self) {
        self.end_document_called += 1;
    }

    fn start_prefix_mapping(&mut self, _prefix: &str, _uri: &str) {
        todo!()
    }

    fn end_prefix_mapping(&mut self, _prefix: &str, _uri: &str) {
        todo!()
    }

    fn start_element(
        &mut self,
        uri: Option<String>,
        _local_name: &str,
        qualified_name: &str,
        attributes: Vec<Attribute>,
    ) {
        self.start_element_called += 1;
        let atts = attributes
            .iter()
            .map(|att| format!(r#"{}="{}""#, att.name, att.value))
            .collect::<Vec<String>>()
            .join(" ");

        let divider = if atts.is_empty() { "" } else { " " };
        self.elements
            .push(format!("<{}{}{}>", qualified_name, divider, atts));
    }

    fn end_element(&mut self, _uri: Option<String>, _local_name: &str, _qualified_name: &str) {
        self.end_element_called += 1;
    }

    fn characters(&mut self, _chars: &[char]) {
        todo!()
    }

    fn error(&mut self, _error: &str) {
        todo!()
    }
}
