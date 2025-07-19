mod sax_parser;
mod sax_parser_test;
mod debug;

#[derive(Debug)]
pub struct Attribute {
    name: String,
    _namespace: Option<String>,
    value: String,
}

pub trait SaxHandler {
    fn start_document(&mut self);
    fn end_document(&mut self);
    fn start_prefix_mapping(&mut self, prefix: &str, uri: &str);
    fn end_prefix_mapping(&mut self, prefix: &str, uri: &str);
    fn start_element(
        &mut self,
        uri: &str,
        local_name: &str,
        qualified_name: &str,
        attributes: Vec<Attribute>,
    );
    fn end_element(&mut self, uri: &str, local_name: &str, qualified_name: &str);
    fn characters(&mut self, chars: &[char]);
    fn error(&mut self, error: &str);
}

