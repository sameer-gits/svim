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
    fn new(text: &str) -> Text {
        let text = Text {
            buffer: String::new(), // Initialize an empty String
            start: 0,
            end: text.len() - 1,
        };
        text
    }

}
