#[derive(Debug, Clone)]
pub enum Trace {
    BRACKET(usize), // (
    NAKED(usize) // #
}

impl Trace {
    pub fn consume(&self, c: &char) -> bool
    {
        match self {
            Trace::BRACKET(_) => match c {
                ')' =>  true,
                _ => false
            }
            Trace::NAKED(_) => match c {
                ' ' => true,
                _ => false
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct Content {
    pub value: String,
    pub start: usize,
    pub end: Option<usize>
}

#[derive(Debug, Clone)]
pub enum State {
    NORMAL{block_traces: Vec<Trace>},
    MATCHING {current: Content, block_traces: Vec<Trace>, content_traces: Vec<Trace>},
    MATCHED(Content)
}

impl State {
    pub fn normal() -> Self {
        Self::NORMAL {
            block_traces: vec![]
        }
    }

    pub fn matching(index: usize, block_traces: Vec<Trace>, content: Option<Content>) -> Self {
        Self::MATCHING {
            block_traces,
            current: content.unwrap_or(Content::new("".to_owned(), index)),
            content_traces: vec![]
        }
    }
}

impl Content {
    pub fn new(content: String, start: usize) -> Self {
        Self {
            value: content, start, end: None
        }
    }

    pub fn merge_content(&mut self, content: String) {
        self.value.push_str(content.as_ref());
    }
}