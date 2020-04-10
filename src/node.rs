use std::hash::{
    Hash, 
    Hasher,
};
use crate::{
    configuration::{
        Configuration,
    },
};

#[derive(Copy, Clone)]
pub struct Node<'a> {
    pub g: u32,
    pub f: u32,
    pub parent: Option<&'a Node<'a>>,
    pub state: Configuration,
}

impl<'a> Node<'a> {
    pub fn new<'b>(g: u32, h: u32, parent: Option<&'b Node<'b>>, config: &Configuration) -> Node<'b> {
        Node {
            g,
            f: g + h,
            parent,
            state: *config,
        }
    }
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl<'a> Eq for Node<'a> {}

impl<'a> Hash for Node<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.g.hash(state);
        self.f.hash(state);
    }
}