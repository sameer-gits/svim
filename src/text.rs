#[derive(Debug)]
pub enum Rope {
    Node(Node),
    Text(Text),
}

#[derive(Debug)]
pub struct Node {
    weight: usize,
    left: Option<Box<Rope>>,
    right: Option<Box<Rope>>,
}

#[derive(Debug)]
pub struct Text {
    buffer: String,
    start: usize,
    end: usize,
}

impl Text {
    pub fn new(s: &str) -> Text {
        let text = Text {
            buffer: String::new(),
            start: 0,
            end: s.len(),
        };
        text
    }

    pub fn add_text(&mut self, character: char) {
        self.buffer.push(character);
        self.end += 1;
    }
}
