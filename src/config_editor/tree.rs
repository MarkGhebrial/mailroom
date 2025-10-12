use std::{
    cell::RefCell,
    fmt::Display,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use ratatui::{
    style::{Style, Stylize},
    widgets::{Block, List, Widget},
};

#[derive(Debug)]
struct Node<T> {
    pub data: T,
    pub children: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data: data,
            children: vec![],
        }
    }

    fn add_child(&mut self, child: Node<T>) {
        self.children.push(Rc::new(RefCell::new(child)));
    }

    fn child(mut self, child: Node<T>) -> Self {
        self.children.push(Rc::new(RefCell::new(child)));
        self
    }
}

struct TreeWalker<'a, T> {
    parent_node: &'a Node<T>,
    index_of_child: usize,
}

#[test]
fn test_tree() {
    let root: Node<String> = Node::new("Root node.".to_string()).child(
        Node::new("Child 1 of root".to_string())
            .child(Node::new("Child 1 of child 1".to_string()))
            .child(
                Node::new("Child 2 of child 1".to_string())
                    .child(Node::new("Modify me, please".to_string())),
            ),
    ).child(Node::new("fdsfsafds".to_string()));

    // let mut branch1 = Node::new("Child of root 1".to_string());
    // branch1.add_child(Node::new("Leaf 1".to_string()));
    // root.add_child(branch1);

    // root.add_child(Node::new("Leaf 2".to_string()));

    print_tree(&root);
}

fn print_tree<T>(node: &Node<T>)
where
    T: Display,
{
    static INDENT_LEVEL: AtomicUsize = AtomicUsize::new(0);

    for _ in 0..INDENT_LEVEL.load(Ordering::Relaxed) {
        print!("  ");
    }
    println!("{}", node.data);

    for node in node.children.iter() {
        INDENT_LEVEL.fetch_add(1, Ordering::Relaxed);
        print_tree(&(*node).borrow());
        INDENT_LEVEL.fetch_sub(1, Ordering::Relaxed);
    }
}
