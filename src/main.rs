use trie::{ArrayNode, Trie};

fn main() {
    let mut root = ArrayNode::default();
    root.append("kayky");
    root.append("kayke");

    println!("{:?}", root.search("kayk"));
}
