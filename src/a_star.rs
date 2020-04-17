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
    let mut frontier = [(start_node, Reverse(start_f))].iter().cloned().collect::<PriorityQueue<_, Reverse<u64>>>();
    let mut visited_states = HashSet::new();
    
    while let Some((node, _)) = frontier.pop() {
        println!("popped node = {:?}", node);
        if node.state == *goal {
            return Some(Path::from(&node));
        }
        
        visited_states.insert(node.state); 

        for succ in generate_successors(&node, goal, h).into_iter()
                    .filter(|s| !visited_states.contains(&s.state)) {
            println!("successor i = {:?}", succ);
            if let None = frontier.get(&succ) {
                println!("inserted in frontier...");
                let f = succ.f;
                frontier.push(succ, Reverse(f));
            }
            else {
                println!("not inserted - KEY UPDATE??");
                let current_f = frontier.get_priority(&succ).unwrap();
                println!("succ.f = {:?}; currenf_f = {:?}", Reverse(succ.f), current_f);
                if Reverse(succ.f) > *current_f {
                    println!("got in if");
                    frontier.change_priority(&succ, Reverse(succ.f));
                }
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

    #[test]
    fn one_move_away_paths_are_found() {
        use crate::heuristics::num_misplaced_tiles;

        let goal = State::new([
            [1,  2,      3],
            [4,  BLANK,  5],
            [6,  7,      8],
        ]).unwrap();   
        let state =  State::new([
            [1,  2,      3],
            [4,  5,  BLANK],
            [6,  7,      8],
        ]).unwrap(); 

        assert!(search(&state, &goal, &num_misplaced_tiles).is_some());
    }
    
    #[test]
    fn two_moves_away_paths_are_found() {
        use crate::heuristics::num_misplaced_tiles;

        let goal = State::new([
            [1,  2,      3],
            [4,  BLANK,  5],
            [6,  7,      8],
        ]).unwrap();   
        let state =  State::new([
            [1,  2,  BLANK],
            [4,  5,      3],
            [6,  7,      8],
        ]).unwrap(); 

        assert!(search(&state, &goal, &num_misplaced_tiles).is_some());
    }
}