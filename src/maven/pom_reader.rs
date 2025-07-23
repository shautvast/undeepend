use crate::maven::pom::Pom;
use crate::xml::{Attribute, SaxHandler};

fn read(xml: &str){

}

struct PomReader{

}

impl SaxHandler for PomReader{
    fn start_document(&mut self) {
        todo!()
    }

    fn end_document(&mut self) {
        todo!()
    }

    fn start_prefix_mapping(&mut self, prefix: &str, uri: &str) {
        todo!()
    }

    fn end_prefix_mapping(&mut self, prefix: &str, uri: &str) {
        todo!()
    }

    fn start_element(&mut self, uri: Option<String>, local_name: &str, qualified_name: &str, attributes: Vec<Attribute>) {
        todo!()
    }

    fn end_element(&mut self, uri: Option<String>, local_name: &str, qualified_name: &str) {
        todo!()
    }

    fn characters(&mut self, chars: &[char]) {
        todo!()
    }

    fn error(&mut self, error: &str) {
        todo!()
    }
}