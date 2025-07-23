use crate::maven::pom::Pom;
use crate::xml::{Attribute, SaxHandler};

fn read(xml: &str){

}

struct PomReader{
    pom: Pom,
}

impl SaxHandler for PomReader{
    fn start_document(&mut self) {
    }

    fn end_document(&mut self) {
    }

    fn start_prefix_mapping(&mut self, prefix: &str, uri: &str) {
    }

    fn start_element(&mut self, uri: Option<String>, local_name: &str, qualified_name: &str, attributes: Vec<Attribute>) {
        // match local_name{
        //     "modelVersion" => {self.pom=Pom{}}
        // }
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