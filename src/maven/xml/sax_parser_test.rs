use crate::maven::xml::{Attribute, SaxHandler};

#[cfg(test)]
mod tests {
    use crate::maven::xml::sax_parser::parse_string;
    use crate::maven::xml::sax_parser_test::TestHandler;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn initialize() {
        INIT.call_once(|| {
            env_logger::init();
        });
    }

    #[test]
    fn test_xml_header() {
        let test_xml = include_str!("test/header.xml");
        let mut testhandler = TestHandler::new();
        parse_string(test_xml.to_string(), Box::new(&mut testhandler))
            .expect("Failed to parse test xml");
        println!("{:?}", testhandler);
        assert!(testhandler.start_document_called);
    }

    #[test]
    fn test_single_element_short() {
        let test_xml = include_str!("test/header.xml");
        let mut testhandler = TestHandler::new();
        parse_string(test_xml.to_string(), Box::new(&mut testhandler))
            .expect("Failed to parse test xml");
        assert!(testhandler.start_document_called);
        assert!(testhandler.start_element_called);
        assert!(!testhandler.elements.is_empty());
        assert_eq!(testhandler.elements[0], "<xml>");
    }

    #[test]
    fn test_single_element() {
        let test_xml = include_str!("test/element.xml");
        let mut testhandler = TestHandler::new();
        parse_string(test_xml.to_string(), Box::new(&mut testhandler))
            .expect("Failed to parse test xml");
        assert!(testhandler.start_document_called);
        assert!(testhandler.start_element_called);
        assert!(!testhandler.elements.is_empty());
        assert_eq!(testhandler.elements[0], "<element>");
    }

    #[test]
    fn test_single_element_single_attribute() {
        let test_xml = include_str!("test/element_with_attribute.xml");
        let mut testhandler = TestHandler::new();
        parse_string(test_xml.to_string(), Box::new(&mut testhandler))
            .expect("Failed to parse test xml");
        assert!(testhandler.start_document_called);
        assert!(testhandler.start_element_called);
        assert!(!testhandler.elements.is_empty());
        assert_eq!(testhandler.elements[0], r#"<element a="1">"#);
        assert!(testhandler.end_element_called);
        assert!(testhandler.end_document_called);
    }
}

#[derive(Debug)]
struct TestHandler {
    start_document_called: bool,
    end_document_called: bool,
    start_element_called: bool,
    end_element_called: bool,
    elements: Vec<String>,
}

impl TestHandler {
    pub fn new() -> Self {
        Self {
            start_document_called: false,
            end_document_called: false,
            start_element_called: false,
            end_element_called: false,
            elements: vec![],
        }
    }
}

impl SaxHandler for TestHandler {
    fn start_document(&mut self) {
        self.start_document_called = true;
    }

    fn end_document(&mut self) {
        self.end_document_called = true;
    }

    fn start_prefix_mapping(&mut self, _prefix: &str, _uri: &str) {
        todo!()
    }

    fn end_prefix_mapping(&mut self, _prefix: &str, _uri: &str) {
        todo!()
    }

    fn start_element(
        &mut self,
        _uri: &str,
        local_name: &str,
        _qualified_name: &str,
        attributes: Vec<Attribute>,
    ) {
        self.start_element_called = true;
        let atts = attributes
            .iter()
            .map(|att| format!(r#"{}="{}""#, att.name, att.value))
            .collect::<Vec<String>>()
            .join(" ");

        let divider = if atts.is_empty() { "" } else { " " };
        self.elements
            .push(format!("<{}{}{}>", local_name, divider, atts));
    }

    fn end_element(&mut self, _uri: &str, _local_name: &str, _qualified_name: &str) {
        self.end_element_called = true;
    }

    fn characters(&mut self, _chars: &[char]) {
        todo!()
    }

    fn error(&mut self, _error: &str) {
        todo!()
    }
}
