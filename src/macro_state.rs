#[derive(Debug, Clone)]
pub enum Trace {
    BRACKET, // (
    NAKED // #
}

impl Trace {
    pub fn consume(&self, c: &char) -> bool
    {
        match self {
            Trace::BRACKET => match c {
                ')' =>  true,
                _ => false
            }
            Trace::NAKED => match c {
                ' ' => true,
                _ => false
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct Content {
    pub value: String,
    pub start: u32,
    pub end: Option<u32>
}

#[derive(Debug, Clone)]
pub enum State {
    NORMAL,
    MATCHING {current: Content, traces: Vec<Trace>},
    MATCHED(Content)
}

impl Content {
    pub fn new(content: String, start: u32) -> Self {
        Self {
            value: content, start, end: None
        }
    }

    pub fn merge_content(&mut self, content: String) {
        self.value.push_str(content.as_ref());
    }
}