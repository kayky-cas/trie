pub trait Trie {
    fn append(&mut self, word: &str);
    fn search(&self, word: &str) -> Option<String>;
}

pub struct ArrayNode {
    children: [Option<Box<ArrayNode>>; 255],
    end: bool,
}

const DEFAULT_ARRAY_NODE_VALUE: Option<Box<ArrayNode>> = None;

impl Default for ArrayNode {
    fn default() -> Self {
        Self {
            children: [DEFAULT_ARRAY_NODE_VALUE; 255],
            end: false,
        }
    }
}

impl Trie for ArrayNode {
    fn append(&mut self, word: &str) {
        if word.is_empty() {
            self.end = true;
        } else {
            let pos = word.as_bytes()[0] as usize;

            if self.children[pos].is_none() {
                self.children[pos] = Some(Box::new(ArrayNode::default()));
            }

            self.children[pos].as_mut().unwrap().append(&word[1..]);
        }
    }

    fn search(&self, word: &str) -> Option<String> {
        todo!()
    }
}
