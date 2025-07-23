use crate::xml::{Attribute, SaxError, SaxHandler};
use std::collections::HashMap;

pub fn parse_string(xml: String, handler: Box<&mut dyn SaxHandler>) -> Result<(), SaxError> {
    SAXParser::new(xml, handler).parse()
}

pub struct SAXParser<'a> {
    xml: Vec<char>,
    handler: Box<&'a mut dyn SaxHandler>,
    position: usize,
    current: char,
    namespace_stack: Vec<(String, isize)>,
    prefix_mapping: HashMap<String, String>,
}

impl<'a> SAXParser<'a> {
    pub fn new(xml: String, handler: Box<&'a mut dyn SaxHandler>) -> Self {
        Self {
            xml: xml.chars().collect(),
            handler,
            position: 0,
            current: '\0',
            namespace_stack: Vec::new(),
            prefix_mapping: HashMap::new(),
        }
    }

    fn parse(&mut self) -> Result<(), SaxError> {
        self.advance()?;
        self.expect(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>",
            "Content is not allowed in prolog.",
        )?;
        self.skip_whitespace()?;
        self.handler.start_document();
        self.parse_elements()
    }

    fn parse_elements(&mut self) -> Result<(), SaxError> {
        while self.position < self.xml.len() {
            if self.current == '<' {
                self.advance()?;
                if self.current == '!' {
                    self.skip_comment()?;
                } else if self.current != '/' {
                    self.parse_start_element()?;
                } else {
                    self.parse_end_element()?;
                }
            }
        }
        self.handler.end_document();
        Ok(())
    }

    fn skip_comment(&mut self) -> Result<(), SaxError> {
        self.expect("!--", "Expect comment start")?;
        let mut c = self.current;
        let mut end_in_sight = 0;
        while end_in_sight < 3 && self.position < self.xml.len() {
            match c {
                '-' if end_in_sight < 2 => {
                    end_in_sight += 1;
                }
                '>' if end_in_sight == 2 => {
                    end_in_sight += 1;
                }
                _ if end_in_sight == 2 => {
                    return Err(SaxError::BadCharacter);
                }
                _ if end_in_sight > 0 => {
                    end_in_sight = 0;
                }
                _ => {}
            }
            c = self.advance()?;
        }
        self.skip_whitespace()?;
        Ok(())
    }

    fn parse_start_element(&mut self) -> Result<(), SaxError> {
        let qname = self.read_until(" \t\n/>")?;

        let mut atts = vec![];
        let mut c = self.current;

        while c.is_whitespace() {
            self.skip_whitespace()?;
            atts.push(self.parse_attribute()?);
            c = self.advance()?;
        }

        let (namespace, lname) = if qname.contains(":") {
            let tokens = qname.splitn(2, ":").collect::<Vec<&str>>();
            let prefix = tokens[0].to_string();
            let name = tokens[1].to_string();
            let namespace = self.prefix_mapping.get(&prefix);
            if let Some(namespace) = namespace {
                (Some(namespace.to_string()), name)
            } else {
                return Err(SaxError::UndeclaredNamespacePrefix(prefix));
            }
        } else if !self.namespace_stack.is_empty() {
            let (name, count) = self.namespace_stack.pop().unwrap();
            self.namespace_stack.push((name.clone(), count + 1));
            (Some(name.clone()), qname)
        } else {
            (None, qname)
        };

        let qualified_name = if let Some(namespace) = &namespace{
            &format!("{}:{}", namespace.clone(), &lname)
        } else {
            &lname
        };

        self.handler
            .start_element(namespace.clone(), lname.as_str(), qualified_name, atts);
        self.skip_whitespace()?;

        if self.current == '/' {
            self.advance()?;
            let namespace = self.pop_namespace();
            self.handler.end_element(namespace, lname.as_str(), qualified_name);
        }
        self.expect_char('>')?;
        self.skip_whitespace()?;
        Ok(())
    }

    fn parse_attribute(&mut self) -> Result<Attribute, SaxError> {
        let att_name = self.read_until("=")?;
        self.skip_whitespace()?;
        self.expect("=", "Expected =")?;
        self.expect("\"", "Expected start of attribute value")?;
        let att_value = self.read_until("\"")?;

        if att_name.starts_with("xmlns:") {
            let prefix = att_name[6..].to_string();
            self.prefix_mapping.insert(prefix, att_value.to_string());
        }

        let namespace = if att_name == "xmlns" {
            self.namespace_stack.push((att_value.clone(), -1));
            Some(att_value.clone())
        } else {
            None
        };

        Ok(Attribute {
            name: att_name.trim().to_string(),
            namespace,
            value: att_value,
        })
    }

    fn parse_end_element(&mut self) -> Result<(), SaxError> {
        self.advance()?;
        let name = self.read_until(">")?;

        let namespace = self.pop_namespace();

        self.handler.end_element(namespace, name.as_str(), "");

        self.expect(">", "Expect end of element")?;
        self.skip_whitespace()?;
        Ok(())
    }

    fn pop_namespace(&mut self) -> Option<String> {
        let namespace = if !self.namespace_stack.is_empty() {
            let (name, count) = self.namespace_stack.pop().unwrap();

            if count > 0 {
                self.namespace_stack.push((name.to_string(), count - 1));
                Some(name)
            } else {
                None
            }
        } else {
            None
        };
        namespace
    }

    fn read_until(&mut self, until: &str) -> Result<String, SaxError> {
        let start = self.position;
        let mut c = self.current;
        let until = until.chars().collect::<Vec<char>>();
        while !until.contains(&c) {
            if self.position > self.xml.len() {
                return Err(SaxError::UnexpectedEof);
            }
            c = self.advance()?;
        }
        Ok(self.xml[start - 1..self.position - 1]
            .iter()
            .collect::<String>())
    }

    fn skip_whitespace(&mut self) -> Result<(), SaxError> {
        let mut c = self.current;
        while (c.is_whitespace()) && self.position < self.xml.len() {
            c = self.advance()?;
        }
        Ok(())
    }

    fn advance(&mut self) -> Result<char, SaxError> {
        if self.position > self.xml.len() {
            return Err(SaxError::UnexpectedEof);
        }
        self.position += 1;
        self.current = if self.position <= self.xml.len() {
            self.xml[self.position - 1]
        } else {
            '\0'
        };
        Ok(self.current)
    }

    fn expect(&mut self, expected: &str, message: &str) -> Result<(), SaxError> {
        for c in expected.chars() {
            if !self.expect_char(c)? {
                return Err(SaxError::UnexpectedCharacter(message.to_string()));
            }
        }
        Ok(())
    }

    fn expect_char(&mut self, expected: char) -> Result<bool, SaxError> {
        if self.position > self.xml.len() {
            return Ok(false);
        }
        let same = self.current == expected;
        if same {
            self.advance()?;
        }
        Ok(same)
    }
}
