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
        self.advance()?;
        self.expect(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>",
            "Content is not allowed in prolog.",
        )?;
        self.skip_whitespace()?;
        self.handler.start_document();
        self.parse_elements()
    }

    fn parse_elements(&mut self) -> anyhow::Result<()> {
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

    fn skip_comment(&mut self) -> anyhow::Result<()> {
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
                _ if end_in_sight > 0 => {
                    end_in_sight -= 0;
                }
                _ => {}
            }
            c = self.advance()?;
        }
        self.skip_whitespace()?;
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
        self.skip_whitespace()?;
        if self.current == '/' {
            self.advance()?;
        }
        self.expect_char('>')?;
        self.skip_whitespace()?;
        Ok(())
    }

    fn parse_attribute(&mut self) -> anyhow::Result<Attribute> {
        let att_name = self.read_until("=")?;
        self.skip_whitespace()?;
        self.expect("=", "Expected =")?;
        self.expect("\"", "Expected start of attribute value")?;
        let att_value = self.read_until("\"")?;

        Ok(Attribute {
            name: att_name.trim().to_string(),
            _namespace: Some("".to_string()),
            value: att_value,
        })
    }

    fn parse_end_element(&mut self) -> anyhow::Result<()> {
        self.advance()?;
        let name = self.read_until(">")?;
        self.handler.end_element("", name.as_str(), "");
        self.expect(">", "Expect end of element")?;
        Ok(())
    }

    fn read_until(&mut self, until: &str) -> anyhow::Result<String> {
        let start = self.position;
        let mut c = self.current;
        let until = until.chars().collect::<Vec<char>>();
        while !until.contains(&c) {
            if self.position > self.xml.len() {
                return Err(anyhow::anyhow!("End reached while expecting {:?}", until));
            }
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
        if self.position > self.xml.len() {
            return Err(anyhow::anyhow!(
                "End reached while expecting {:?}",
                self.current
            ));
        }
        self.position += 1;
        self.current = if self.position <= self.xml.len() {
            self.xml[self.position - 1]
        } else {
            '\0'
        };
        Ok(self.current)
    }

    fn expect(&mut self, expected: &str, message: &str) -> anyhow::Result<()> {
        for c in expected.chars() {
            if !self.expect_char(c)? {
                return Err(anyhow::anyhow!(message.to_string()));
            }
        }
        Ok(())
    }

    fn expect_char(&mut self, expected: char) -> anyhow::Result<bool> {
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
