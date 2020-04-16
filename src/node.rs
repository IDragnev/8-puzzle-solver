use crate::{
    state::{
        self,
        State,
    },
};
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Node {
    pub g: u64,
    pub f: u64,
    pub parent: Option<Rc<Node>>,
    pub state: State,
}

impl Node {
    pub fn new(g: u64, h: u64, parent: Option<Rc<Node>>, state: &State) -> Node {
        Node {
            g,
            f: g + h,
            parent,
            state: *state,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        self.f.cmp(&other.f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nodes_are_compared_by_their_f_value() {
        use state::BLANK;
        let s = State::new([
            [1, 2,      3],
            [4, 5,  BLANK],
            [7, 6,      8]
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
