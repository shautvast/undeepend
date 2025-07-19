use crate::maven::xml::{Attribute, SaxHandler};

pub fn parse_string(xml: String, handler: Box<&mut dyn SaxHandler>) -> anyhow::Result<()> {
    let mut parser = SAXParser::new(xml, handler);
    parser.parse()
}

struct SAXParser<'a> {
    xml: Vec<char>,
    handler: Box<&'a mut dyn SaxHandler>,
    position: usize,
    current: char,
}

impl<'a> SAXParser<'a> {
    pub fn new(xml: String, handler: Box<&'a mut dyn SaxHandler>) -> Self {
        Self {
            xml: xml.chars().collect(),
            handler,
            position: 0,
            current: '\0',
        }
    }

    fn parse(&mut self) -> anyhow::Result<()> {
        self.expect(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>",
            "Content is not allowed in prolog.",
        )?;
        self.skip_whitespace()?;
        self.handler.start_document();
        self.parse_elements()
    }

    fn parse_elements(&mut self) -> anyhow::Result<()> {
        if self.current == '<' {
            self.advance()?;
            if self.next_char()? != '/' {
                self.parse_start_element()?;
            } else {
                self.parse_end_element()?;
            }
        }
        Ok(())
    }

    fn parse_start_element(&mut self) -> anyhow::Result<()> {
        let name = self.read_until(" />")?;
        let mut atts = vec![];
        let mut c = self.current;
        while c == ' ' {
            self.skip_whitespace()?;
            atts.push(self.parse_attribute()?);
            c = self.advance()?;
        }

        self.handler.start_element("", name.as_str(), "", atts);
        Ok(())
    }

    fn parse_attribute(&mut self) -> anyhow::Result<Attribute> {
        let att_name = self.read_until("=")?;
        self.skip_whitespace()?;
        self.expect("\"", "Expected start of attribute value")?;
        let att_value = self.read_until("\"")?;

        Ok(Attribute {
            name: att_name.trim().to_string(),
            namespace: Some("".to_string()),
            value: att_value,
        })
    }

    fn parse_end_element(&mut self) -> anyhow::Result<()> {
        let name = self.read_until(">")?;
        self.handler.end_element("", name.as_str(), "");
        Ok(())
    }

    fn read_until(&mut self, until: &str) -> anyhow::Result<String> {
        let start = self.position;
        let mut c = self.current;
        let until = until.chars().collect::<Vec<char>>();
        while !until.contains(&c) {
            c = self.advance()?;
        }
        Ok(self.xml[start - 1..self.position - 1]
            .iter()
            .collect::<String>())
    }

    fn skip_whitespace(&mut self) -> anyhow::Result<()> {
        let mut c = self.current;
        while (c.is_whitespace()) && self.position < self.xml.len() {
            c = self.advance()?;
        }
        Ok(())
    }

    fn advance(&mut self) -> anyhow::Result<char> {
        self.position += 1;
        self.current = self.xml[self.position - 1];
        Ok(self.current)
    }

    fn next_char(&mut self) -> anyhow::Result<char> {
        if self.position >= self.xml.len() {
            Err(anyhow::anyhow!("End reached"))
        } else {
            Ok(self.xml[self.position + 1])
        }
    }

    fn expect(&mut self, header_line: &str, message: &str) -> anyhow::Result<()> {
        for c in header_line.chars() {
            if !self.expect_char(c)? {
                return Err(anyhow::anyhow!(message.to_string()));
            }
        }
        self.advance()?;
        Ok(())
    }

    fn expect_char(&mut self, expected: char) -> anyhow::Result<bool> {
        if self.position >= self.xml.len() {
            return Ok(false);
        }
        Ok(self.advance()? == expected)
    }
}
