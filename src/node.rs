use crate::{
    state::{
        State,
    },
};
use std::rc::Rc;
use std::hash::{
    Hash, 
    Hasher,
};

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

impl Hash for Node {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.state.hash(hasher); 
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl Eq for Node {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nodes_are_compared_by_their_states() {
        use crate::state::BLANK;
        let s = State::new([
            [1, 2,      3],
            [4, 5,  BLANK],
            [7, 6,      8]
        ]).unwrap();

        let n0 = Node::new(0, 1, None, &s);
        let n1 = Node::new(1, 1, None, &s.move_up().unwrap());

        assert!(n0 == n0);
        assert!(n0 != n1);
    }
}
