mod node;
mod state;
mod path;
mod a_star;
mod heuristics;

use state::{
    State,
    BLANK,
};
use path::Path;

fn print_solution(p: &Path) {
    println!("Solution:");
    for s in p.to_vec_states().iter().map(|s| { format!("{}", s) }) {
        println!("{}", s);
    }
}

fn main() {
    let goal = State::new([
        [1,  2,      3],
        [4,  BLANK,  5],
        [6,  7,      8],
    ]).unwrap();
    
    let state =  State::new([
        [8,  5,  BLANK],
        [6,  2,      4],
        [3,  7,      1],
    ]).unwrap();
    
    if let Some(path) = a_star::search(&state, &goal, &heuristics::num_misplaced_tiles) {
        print_solution(&path);
    }
    else {
        println!("No solution found!");
    }
}
