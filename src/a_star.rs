use crate::{
    path::Path,
    state::{
        State,
        immediate_neighbours,
    },
    node::Node,
};
use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::cmp::Reverse;
use std::rc::Rc;

pub fn search<Heuristic>(start: &State, goal: &State, h: &Heuristic) -> Option<Path>
where Heuristic : Fn(&State, &State) -> u64 {
    let start_node = start_node(start, h(start, goal));
    let start_f = start_node.f;
    let mut frontier = [(start_node, Reverse(start_f))].iter().cloned().collect::<PriorityQueue<_, _>>();
    let mut visited_states = HashSet::new();
    
    while let Some((node, _)) = frontier.pop() {
        if node.state == *goal {
            return Some(Path::from(&node));
        }
        
        visited_states.insert(node.state); 

        for successor in generate_successors(&node, goal, h).into_iter()
                         .filter(|s| !visited_states.contains(&s.state)) {
            if let None = frontier.get(&successor) {
                let f = Reverse(successor.f);
                frontier.push(successor, f);
            }
            else {
                let current_f = frontier.get_priority(&successor).unwrap();
                let successor_f = Reverse(successor.f);
                if successor_f > *current_f {
                    frontier.change_priority(&successor, successor_f);
                }
            }
        }
    }

    None
}

fn start_node(s: &State, h: u64) -> Node {
    Node::new(0, h, None, s)
}

fn generate_successors<H>(node: &Node, goal: &State, h: &H) -> Vec<Node> 
where H : Fn(&State, &State) -> u64 {
    let parent = Rc::new(node.clone());
    immediate_neighbours(&node.state)
    .into_iter()
    .map(move |state| {
        Node::new(
            node.g + 1,
            h(&state, goal),
            Some(Rc::clone(&parent)),
            &state
        )
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
        let s = State::new(&[
            [1,  2,  BLANK],
            [4,  8,      5],
            [3,  6,      7],
        ]).unwrap();
        assert!(search(&s, &s, &h).is_some());
    }

    #[test]
    fn one_move_away_paths_are_found() {
        use crate::heuristics::num_misplaced_tiles;

        let goal = State::new(&[
            [1,  2,      3],
            [4,  BLANK,  5],
            [6,  7,      8],
        ]).unwrap();   
        let state =  State::new(&[
            [1,  2,      3],
            [4,  5,  BLANK],
            [6,  7,      8],
        ]).unwrap(); 

        assert!(search(&state, &goal, &num_misplaced_tiles).is_some());
    }
    
    #[test]
    fn two_moves_away_paths_are_found() {
        use crate::heuristics::num_misplaced_tiles;

        let goal = State::new(&[
            [1,  2,      3],
            [4,  BLANK,  5],
            [6,  7,      8],
        ]).unwrap();   
        let state =  State::new(&[
            [1,  2,  BLANK],
            [4,  5,      3],
            [6,  7,      8],
        ]).unwrap(); 

        assert!(search(&state, &goal, &num_misplaced_tiles).is_some());
    }

    #[test]
    fn long_paths_are_found() {
        use crate::heuristics::num_misplaced_tiles;

        let goal = State::new(&[
            [1,  2,      3],
            [4,  BLANK,  5],
            [6,  7,      8],
        ]).unwrap();
        let state =  State::new(&[
            [8,  5,  BLANK],
            [6,  2,      4],
            [3,  7,      1],
        ]).unwrap();

        assert!(search(&state, &goal, &num_misplaced_tiles).is_some());
    }

    #[test]
    fn search_on_a_no_solution_case_returns_none() {
        use crate::heuristics::num_misplaced_tiles;

        let goal = State::new(&[
            [1,  2,      3],
            [4,  5,      6],
            [7,  8,  BLANK],
        ]).unwrap();
        
        let state =  State::new(&[
            [8,      1,  2],
            [BLANK,  4,  3],
            [7,      6,  5],
        ]).unwrap();

        assert!(search(&state, &goal, &num_misplaced_tiles).is_none());
    }
}