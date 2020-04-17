mod node;
mod state;
mod path;
mod a_star;
mod heuristics;

use state::{
    State,
    BLANK,
};

fn main() {
    let goal = State::new([
        [1,  2,      3],
        [4,  BLANK,  5],
        [6,  7,      8],
    ]).unwrap(); 
    let _ = a_star::search(&goal, &goal, &heuristics::num_misplaced_tiles);
}
