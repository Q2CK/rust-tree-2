use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(PartialEq)]
struct Content {
    text: String
}

type Container = Rc<RefCell<Node>>;

#[derive(PartialEq)]
struct Node {
    parent: Option<Container>,
    children: Vec<Container>,
    depth: usize,
    content: Content
}

impl Node {
    fn new(text: String) -> Container {
        Rc::new(RefCell::new(Node {
            parent: None,
            children: vec![],
            depth: 0,
            content: Content { text }
        }))
    }
    fn add(parent: &Container, new_node: Container) {
        new_node.borrow_mut().parent = Some(parent.clone());
        parent.borrow_mut().children.push(new_node);
    }
    fn all<F>(&mut self, f: &mut F) where F: FnMut(&mut Node) {
        f(self);
        for child in &self.children {
            child.borrow_mut().all(f);
        }
    }
}

fn main() {
    let root = Node::new("this is the root node".to_string());
    let sub_node = Node::new("this is a sub node".to_string());
    let sub_sub_node = Node::new("even deeper".to_string());
    let sub_sub_node_ref = sub_sub_node.clone();
    Node::add(&sub_node, sub_sub_node);
    Node::add(&root, sub_node);
    Node::add(&root, Node::new("test".to_string()));

    let mut level = 0;
    let mut last_split_level = 0;

    root.borrow_mut().all(&mut (|x: &mut Node| {
        x.depth = level;
        if x.children.len() > 1 {
            last_split_level = x.depth.clone();
            level += 1;
        } else if x.children.len() == 1 {
            level += 1;
        } else {
            level = last_split_level;
        }
    }));

    root.borrow_mut().all(&mut (|x: &mut Node| {
        println!("{} {}", x.content.text, x.depth);
    }));
}
