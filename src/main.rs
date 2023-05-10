struct Node {
    val: char,
    child: [Option<Box<Node>>; 255],
}

const DEFAULT_NODE_VALUE: Option<Box<Node>> = None;

impl Default for Node {
    fn default() -> Self {
        Self {
            val: '\0',
            child: [DEFAULT_NODE_VALUE; 255],
        }
    }
}

fn main() {
    print!("Hello")
}
