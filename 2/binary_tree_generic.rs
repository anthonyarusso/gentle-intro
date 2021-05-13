type NodeBox<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    payload: T,
    left: NodeBox<T>,
    right: NodeBox<T>
}

impl <T: PartialOrd + std::fmt::Display> Node<T> {
    fn new(p: T) -> Self {
        Self {
            payload: p,
            left: None,
            right: None
        }
    }
    fn boxer(node: Self) -> NodeBox<T> {
        Some(Box::new(node))
    }
    fn set_left(&mut self, node: Self) {
        self.left = Self::boxer(node);
    }
    fn set_right(&mut self, node: Self) {
        self.right = Self::boxer(node);
    }
    fn insert(&mut self, data: T) {
        if data < self.payload {
            match self.left {
                Some(ref mut n) => n.insert(data),
                None => self.set_left(Self::new(data))
            }
        } else {
            match self.right {
                Some(ref mut n) => n.insert(data),
                None => self.set_right(Self::new(data))
            }
        }
    }
    fn visit(&self) {
        if let Some(ref left) = self.left {
            left.visit();
        }
        println!("'{}'", self.payload);
        if let Some(ref right) = self.right {
            right.visit();
        }
    }
}

fn main() {
    let mut root = Node::new("mn".to_string());
    root.insert("a".to_string());
    root.insert("g".to_string());
    root.insert("r".to_string());
    root.insert("z".to_string());

    println!("tree {:#?}", root);
    root.visit();
}
