#[derive(Debug)]
struct Node<T>
where
    T: Copy,
{
    name: char,
    childs: [Option<Box<Node<T>>>; 26],
    val: Option<T>,
}

impl<T> Node<T>
where
    T: Copy,
{
    fn new(val: char) -> Self {
        Self {
            name: val,
            childs: Default::default(),
            val: None,
        }
    }

    fn append_child(&mut self, buffer: &[u8], val: T) {
        if buffer.is_empty() {
            self.val = Some(val);
            return;
        }

        let idx = (buffer[0] - b'a') as usize;

        if self.childs[idx].is_none() {
            self.childs[idx] = Some(Box::new(Node::new(buffer[0] as char)));
        }

        let child = &mut self.childs[idx];

        match child {
            Some(child) => child.append_child(&buffer[1..], val),
            None => todo!(),
        }
    }

    fn get(&self, buffer: &[u8]) -> Option<T> {
        if buffer.is_empty() {
            return self.val;
        }
        let idx = (buffer[0] - b'a') as usize;

        if self.childs[idx].is_none() {
            return None;
        }

        let child = &self.childs[idx];

        match child {
            Some(child) => child.get(&buffer[1..]),
            None => todo!(),
        }
    }
}

fn main() {
    let mut root = Node::new(Default::default());

    root.append_child("hello".as_bytes(), 10);
    root.append_child("helicoptero".as_bytes(), 15);

    println!("{:?}", root.get("helicoptero".as_bytes()));
}
