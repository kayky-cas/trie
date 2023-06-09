use std::fmt::{Debug, Formatter};

pub trait Trie {
    fn append(&mut self, word: &str);
    fn search(&self, word: &str) -> Vec<String>;
}

pub struct Node {
    children: [Option<Box<Node>>; 255],
    end: bool,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let children_str = self
            .children
            .iter()
            .enumerate()
            .filter(|(_, child)| child.is_some())
            .map(|(i, _)| i.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        f.debug_struct("ArrayNode")
            .field("children", &children_str.as_str())
            .field("end", &self.end)
            .finish()
    }
}

const DEFAULT_ARRAY_NODE_VALUE: Option<Box<Node>> = None;

impl Default for Node {
    fn default() -> Self {
        Self {
            children: [DEFAULT_ARRAY_NODE_VALUE; 255],
            end: false,
        }
    }
}

impl Trie for Node {
    fn append(&mut self, word: &str) {
        if word.is_empty() {
            self.end = true;
        } else {
            let pos = word.as_bytes()[0] as usize;

            if self.children[pos].is_none() {
                self.children[pos] = Some(Box::new(Node::default()));
            }

            self.children[pos].as_mut().unwrap().append(&word[1..]);
        }
    }

    fn search(&self, word: &str) -> Vec<String> {
        todo!()
    }
}
