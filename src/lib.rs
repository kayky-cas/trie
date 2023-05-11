use std::fmt::{Debug, Formatter};

pub trait Trie {
    fn append(&mut self, word: &str);
    fn search(&self, word: &str) -> Option<String>;
}

pub struct ArrayNode {
    children: [Option<Box<ArrayNode>>; 255],
    end: bool,
}

impl Debug for ArrayNode {
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
        if word.is_empty() {
            if self.end {
                Some(String::from(""))
            } else {
                None
            }
        } else {
            let pos = word.as_bytes()[0] as usize;

            if self.children[pos].is_none() {
                None
            } else {
                let result = self.children[pos].as_ref().unwrap().search(&word[1..]);

                match result {
                    Some(mut result) => {
                        result.insert(0, word.chars().nth(0).unwrap());
                        Some(result)
                    }
                    None => None,
                }
            }
        }
    }
}
