use std::hash::{
    Hash, 
    Hasher,
};
use crate::{
    state::{
        State,
    },
};

#[derive(Copy, Clone)]
pub struct Node<'a> {
    pub g: u32,
    pub f: u32,
    pub parent: Option<&'a Node<'a>>,
    pub state: State,
}

impl<'a> Node<'a> {
    pub fn new<'b>(g: u32, h: u32, parent: Option<&'b Node<'b>>, state: &State) -> Node<'b> {
        Node {
            g,
            f: g + h,
            parent,
            state: *state,
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