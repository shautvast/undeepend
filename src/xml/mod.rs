pub mod sax_parser;
mod debug;

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub namespace: Option<String>,
    pub value: String,
}

pub trait SaxHandler {
    fn start_document(&mut self);
    fn end_document(&mut self);
    fn start_prefix_mapping(&mut self, prefix: &str, uri: &str);
    fn start_element(
        &mut self,
        uri: Option<String>,
        local_name: &str,
        qualified_name: &str,
        attributes: Vec<Attribute>,
    );
    fn end_element(&mut self, uri: Option<String>, local_name: &str, qualified_name: &str);
    fn characters(&mut self, chars: &[char]);
    fn error(&mut self, error: &str);
}

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum SaxError {
    BadCharacter,
    UnexpectedEof,
    UnexpectedCharacter(String),
    UndeclaredNamespacePrefix(String),
}

impl fmt::Display for SaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SaxError::BadCharacter => write!(f, "Bad character"),
            SaxError::UnexpectedEof => write!(f, "Unexpected end of document"),
            SaxError::UnexpectedCharacter(c) => write!(f, "Unexpected character {}",c),
            SaxError::UndeclaredNamespacePrefix(prefix) => write!(f, "Undeclared namespace prefix{}", prefix),
        }
    }
}

impl std::error::Error for SaxError {}