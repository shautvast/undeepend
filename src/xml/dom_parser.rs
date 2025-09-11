use crate::xml::sax_parser::parse_string;
use crate::xml::{Attribute, SaxError, SaxHandler};

/// get a generic XML object (Document) from the xml contents. This is called DOM parsing
pub fn get_document(xml: impl Into<String>) -> Result<Document, SaxError> {
    let mut dom_hax_handler = DomSaxHandler::new();
    parse_string(&xml.into(), Box::new(&mut dom_hax_handler))?;

    Ok(dom_hax_handler.into_doc())
}

#[derive(Debug)]
pub struct Document {
    pub root: Node,
}

// used internally to holds usize references to children.
// needed to ward off the borrow checker
// don't ask about the name.
#[derive(Debug, Clone, PartialEq)]
struct BNode {
    name: String,
    namespace: Option<String>,
    children: Vec<usize>,
    attributes: Vec<Attribute>,
    text: Option<String>,
}

// in the end the usize references are translated to other Nodes
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub name: String,
    pub namespace: Option<String>,
    pub children: Vec<Node>,
    pub attributes: Vec<Attribute>,
    pub text: Option<String>,
}

impl From<&BNode> for Node {
    fn from(b_node: &BNode) -> Self {
        Self {
            name: b_node.name.clone(),
            namespace: b_node.namespace.clone(),
            children: vec![],
            attributes: b_node.attributes.to_vec(),
            text: b_node.text.clone(),
        }
    }
}

impl BNode {
    fn new(name: &str, namespace: Option<String>, attributes: Vec<Attribute>) -> Self {
        Self {
            name: name.to_string(),
            namespace,
            attributes,
            children: vec![],
            text: None,
        }
    }
}

struct DomSaxHandler {
    node_stack: Vec<usize>,
    nodes: Vec<BNode>,
}

impl DomSaxHandler {
    fn new() -> Self {
        Self {
            node_stack: vec![],
            nodes: vec![],
        }
    }

    fn into_doc(self) -> Document {
        let b_node = &self.nodes[self.node_stack[0]];
        let node = self.to_node(b_node);
        Document { root: node }
    }

    fn to_node(&self, b_node: &BNode) -> Node {
        let mut node: Node = b_node.into();
        for child_index in &b_node.children {
            let child = self.nodes.get(*child_index).unwrap();
            node.children.push(self.to_node(child));
        }
        node
    }
}

impl SaxHandler for DomSaxHandler {
    fn start_document(&mut self) {}

    fn end_document(&mut self) {}

    fn start_prefix_mapping(&mut self, _prefix: &str, _uri: &str) {}

    fn start_element(
        &mut self,
        uri: Option<String>,
        local_name: &str,
        _qualified_name: &str,
        attributes: Vec<Attribute>,
    ) {
        let id = self.nodes.iter().len();
        let node = BNode::new(local_name, uri, attributes);
        self.nodes.push(node);

        if !self.node_stack.is_empty() {
            let parent_index = *self.node_stack.last().unwrap();
            self.nodes.get_mut(parent_index).unwrap().children.push(id);
        }
        self.node_stack.push(id);
    }

    fn end_element(&mut self, _uri: Option<String>, _local_name: &str, _qualified_name: &str) {
        if self.node_stack.len() > 1 {
            self.node_stack.pop();
        }
    }

    fn characters(&mut self, chars: &[char]) {
        if !self.node_stack.is_empty() {
            let top = *self.node_stack.last().unwrap();
            let parent = self.nodes.get_mut(top).unwrap();
            parent.text = Some(chars.iter().collect::<String>());
        }
    }

    fn error(&mut self, error: &str) {
        panic!("{}", error)
    }
}
