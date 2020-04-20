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
use rand::Rng;

fn eight_puzzle_solver(start: &State, goal: &State) {
    println!("Start state:\n{}", start);
    println!("Goal state: \n{}", goal);

    if let Some(path) = a_star::search(&start, &goal, &heuristics::num_misplaced_tiles) {
        print_solution(&path);
    }
    else {
        println!("No solution found!");
    }
}

fn print_solution(p: &Path) {
    println!("Solution:");
    for s in p.to_vec_states().iter().map(|s| { format!("{}", s) }) {
        println!("{}", s);
    }
}

fn generate_random_state() -> State {
    let mut tiles = state::possible_tiles().into_iter().collect::<Vec<_>>();
    let mut grid = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            let k = rand::thread_rng().gen_range(0, tiles.len());
            grid[i][j] = tiles[k];
            tiles.remove(k);
        }
    }

    State::new(&grid).unwrap()
}

fn main() {
    let goal = State::new(&[
        [1,  2,      3],
        [4,  5,      6],
        [7,  8,  BLANK],
    ]).unwrap();
    
    let start = generate_random_state();
    
    eight_puzzle_solver(&start, &goal);
}
