use std::{array, cell::RefCell, rc::Rc, str::FromStr};

const K: usize = 5; //order of the tree

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Node<T> {
    // sized
    data: Vec<T>,
    links: Vec<Box<Node<T>>>, //Box for mutation
}

#[derive(Debug)]
pub struct BTree<T: Copy> {
    root: Option<Box<Node<T>>>,
    degree: usize
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        let data: Vec<T> = Vec::with_capacity(K - 1);
        let links: Vec<Node<T>> = Vec::with_capacity(K);
        Node { data, links }
    }
}


impl<T: Ord> BTree<T> {
    pub fn new(degree: usize) -> Self {
        BTree { root: None, degree}
    }

    pub fn insert(&mut self, key: T) {
        match self.root {
            Some(ref mut root) => {
                if root.data.len() == (2 * self.degree) - 1 {
                    let new_root = Box::new(Node {
                        links: vec![],
                        data:
                    })
                }
            }
        }
    }
}
