#[derive(Debug)]
struct Node<T>
where
    T: Copy,
{
    id: char,
    childs: [Option<Box<Node<T>>>; 26],
    value: Option<T>,
}

impl<T> Node<T>
where
    T: Copy,
{
    fn new(id: char) -> Self {
        Self {
            id,
            childs: Default::default(),
            value: None,
        }
    }

    fn append_child(&mut self, buffer: &[u8], value: T) {
        if buffer.is_empty() {
            self.value = Some(value);
            return;
        }

        let index = (buffer[0] - b'a') as usize;

        if self.childs[index].is_none() {
            self.childs[index] = Some(Box::new(Node::new(buffer[0] as char)));
        }

        let child = &mut self.childs[index];

        match child {
            Some(child) => child.append_child(&buffer[1..], value),
            None => todo!(),
        }
    }

    fn get(&self, buffer: &[u8]) -> Option<T> {
        if buffer.is_empty() {
            return self.value;
        }
        let index = (buffer[0] - b'a') as usize;

        if self.childs[index].is_none() {
            return None;
        }

        let child = &self.childs[index];

        match child {
            Some(child) => child.get(&buffer[1..]),
            None => todo!(),
        }
    }
}

fn main() {
    let mut root = Node::new(Default::default());

    root.append_child(b"hello", 10);
    root.append_child(b"helicoptero", 15);

    println!("{:?}", root.get(b"helicoptero"));
}
