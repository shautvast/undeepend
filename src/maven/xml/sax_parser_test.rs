use crate::maven::xml::{Attribute, SaxHandler};

#[cfg(test)]
mod tests {
    use crate::maven::xml::sax_parser::parse_string;
    use crate::maven::xml::sax_parser_test::TestHandler;

    #[test]
    fn test_xml_header() {
        let test_xml = include_str!("test/header.xml");
        let mut testhandler = TestHandler::new();
        parse_string(test_xml.to_string(), Box::new(&mut testhandler))
            .expect("Failed to parse test xml");
        println!("{:?}", testhandler);
        assert_eq!(testhandler.start_document_called, 1);
        assert_eq!(testhandler.end_document_called, 1);
    }

    #[test]
    fn test_single_element_short() {
        let test_xml = include_str!("test/header.xml");
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
        let test_xml = include_str!("test/element.xml");
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
        let test_xml = include_str!("test/element_with_attribute.xml");
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
        let test_xml = include_str!("test/comment.xml");
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
    fn test_namespaces() {
        let test_xml = include_str!("test/namespaces.xml");
        let mut testhandler = TestHandler::new();
        parse_string(test_xml.to_string(), Box::new(&mut testhandler))
            .expect("Failed to parse test xml");
        assert_eq!(testhandler.start_document_called, 1);
        assert_eq!(testhandler.start_element_called, 4);
        assert!(!testhandler.elements.is_empty());
        assert_eq!(
            testhandler.elements[0],
            r#"<bookstore>"#
        );
        assert_eq!(
            testhandler.elements[1],
            r#"<http://example.com/books:book xmlns="http://example.com/books" id="1" category="fiction">"#
        );
        assert_eq!(
            testhandler.elements[2],
            r#"<http://example.com/books:page>"#
        );
        assert_eq!(
            testhandler.elements[3],
            r#"<publisher>"#
        );
        assert_eq!(testhandler.end_element_called, 4);
        assert_eq!(testhandler.end_document_called, 1);
    }
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
        local_name: &str,
        _qualified_name: &str,
        attributes: Vec<Attribute>,
    ) {
        self.start_element_called += 1;
        let atts = attributes
            .iter()
            .map(|att| format!(r#"{}="{}""#, att.name, att.value))
            .collect::<Vec<String>>()
            .join(" ");

        let uri = if let Some(uri) = uri {
            format!("{}:", uri)
        } else {
            "".to_string()
        };

        let divider = if atts.is_empty() { "" } else { " " };
        self.elements
            .push(format!("<{}{}{}{}>", uri, local_name, divider, atts));
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
