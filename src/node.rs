use crate::{
    state::{
        self,
        State,
    },
};
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
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
        self.f == other.f
    }
}

impl<'a> Eq for Node<'a> {}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Node<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Node<'a>) -> Ordering {
        self.f.cmp(&other.f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nodes_are_compared_by_their_f_value() {
        let s = State::new([
            [1, 2, 3],
            [4, 5, state::BLANK],
            [7, 6, 8]
        ]).unwrap();

        let n0 = Node::new(0, 1, None, &s);
        let n1 = Node::new(1, 1, None, &s);

        assert!(n0 == n0);
        assert!(n1 == n1);
        assert!(n0 != n1);
        assert!(n0 < n1);
        assert!(n1 > n0);
        assert!(n0 <= n1);
        assert!(n1 >= n0);
    }
}
