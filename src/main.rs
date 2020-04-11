mod node;
mod state;
mod path;

use std::collections::BinaryHeap;
use std::collections::HashSet;

use path::Path;
use state::{
    State,
    immediate_neighbours,
};
use node::Node;

fn initial_node<'a>(s: &State, h: u32) -> Node<'a> {
    Node {
        g: 0,
        f: h,
        parent: None,
        state: *s
    }
}

fn generate_successors<'a, H>(node: &'a Node<'a>, goal: &State, h: &H) -> Vec<Node<'a>> 
where H : Fn(&State, &State) -> u32 {
    immediate_neighbours(&node.state)
    .into_iter()
    .map(move |state| {
        let g = node.g + 1;
        Node {
            g,
            f: g + h(&state, goal),
            parent: Some(node),
            state,
        }
    })
    .collect()
}

fn main() {
    println!("Hello, world!");
}
