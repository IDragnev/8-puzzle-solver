use crate::{
    path::Path,
    state::{
        State,
        immediate_neighbours,
    },
    node::Node,
};

use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::rc::Rc;

pub fn search<Heuristic>(start: &State, goal: &State, h: &Heuristic) -> Option<Path>
where Heuristic : Fn(&State, &State) -> u64 {
    let start_node = start_node(start, h(start, goal));
    let mut frontier = BinaryHeap::from(vec![start_node]);
    let mut visited = HashSet::new();
    
    while let Some(node) = frontier.pop() {
        visited.insert(node.state); 
        
        if node.state == *goal {
            return Some(Path::from(&node));
        }

        for succ in generate_successors(&node, goal, h) {
            if !visited.contains(&succ.state) {
                frontier.push(succ);
            }
        }
    }

    None
}

fn start_node(s: &State, h: u64) -> Node {
    Node {
        g: 0,
        f: h,
        parent: None,
        state: *s
    }
}

fn generate_successors<H>(node: &Node, goal: &State, h: &H) -> Vec<Node> 
where H : Fn(&State, &State) -> u64 {
    immediate_neighbours(&node.state)
    .into_iter()
    .map(move |state| {
        let g = node.g + 1;
        Node {
            g,
            f: g + h(&state, goal),
            parent: Some(Rc::new(node.clone())),
            state,
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::BLANK;

    #[test]
    fn trivial_paths_are_found() {
        let h = |_: &State, _:&State| 1;
        let s = State::new([
            [1,  2,  BLANK],
            [4,  8,      5],
            [3,  6,      7],
        ]).unwrap();
        assert!(search(&s, &s, &h).is_some());
    }

}