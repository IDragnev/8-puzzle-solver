use crate::{
    node::{
        Node,
    },
    state::{
        State,
    },
};

#[derive(Debug)]
pub struct Path {
    node: Node,    
}

impl Path {
    pub fn from<'b>(node: &Node) -> Path {
        Path {
            node: node.clone(),
        }
    }
    
    pub fn to_vec_nodes(&self) -> Vec<&Node> {
        self
        .rev_iter()
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .map(|&n| n)
        .collect()
    }

    pub fn to_vec_states(&self) -> Vec<&State> {
        self
        .to_vec_nodes()
        .iter()
        .map(|n| &n.state)
        .collect()
    }

    fn rev_iter(&self) -> NodeIterator {
        NodeIterator {
            node: Some(&self.node)
        }
    }
}

struct NodeIterator<'a> {
    node: Option<&'a Node>,
}

impl<'a> std::iter::Iterator for NodeIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.node;

        self.node = self.node.and_then(|n| {
            n.parent
            .as_ref()
            .map(|rc_node| {
                rc_node.as_ref()
            })
        });

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use crate::state::BLANK;

    #[test]
    fn iterating_paths() {
        let state = State::new(&[
            [1,  2,      8],
            [4,  BLANK,  5],
            [3,  6,      7],
        ]).unwrap();
        let n0 = Node::new(0, 0, None, &state);
        let n1 = Node::new(1, 0, Some(Rc::new(n0)), &state);
        let n2 = Node::new(2, 0, Some(Rc::new(n1)), &state);
        let p = Path::from(&n2);

        let nodes = p.to_vec_nodes();
        
        assert!(nodes.len() == 3);
        assert!(nodes[0].g == 0);
        assert!(nodes[1].g == 1);
        assert!(nodes[2].g == 2);
    }
}