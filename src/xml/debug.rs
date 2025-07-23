use log::debug;
use crate::xml::SaxHandler;

pub struct DebugHandler {}

impl SaxHandler for DebugHandler {
    fn start_document(&mut self) {
        debug!("start_document");
    }
    fn end_document(&mut self) {
        debug!("end_document");
    }
    fn start_prefix_mapping(&mut self, _prefix: &str, _uri: &str) {
        debug!("start_prefix_mapping");
    }
    fn end_prefix_mapping(&mut self, _prefix: &str, _uri: &str) {
        debug!("end_prefix_mapping");
    }
    fn start_element(
        &mut self,
        _uri: Option<String>,
        local_name: &str,
        _qualified_name: &str,
        attributes: Vec<crate::xml::Attribute>,
    ) {
        debug!("start_element {}, {:?}", local_name, attributes);
    }
    fn end_element(&mut self, _uri: Option<String>, local_name: &str, _qualified_name: &str) {
        debug!("end_element {} ", local_name);
    }
    fn characters(&mut self, chars: &[char]) {
        debug!("characters {:?}", chars.iter().collect::<String>());
    }
    fn error(&mut self, _error: &str) {
        debug!("error");
    }
}