use crate::{
    state::{
        State,
    },
};

pub fn num_misplaced_tiles(state: &State, goal: &State) -> u64 {
    state
    .iter()
    .zip(goal.iter())
    .fold(0, |acc, (left, right)| {
        if left == right { acc } else { acc + 1 }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::BLANK;

    #[test]
    fn num_misplaced_tiles_is_correct() {
        let goal = State::new(&[
            [1,  2,      3],
            [4,  BLANK,  5],
            [6,  7,      8],
        ]).unwrap();   
        let state =  State::new(&[
            [1,  2,  BLANK],
            [4,  8,      5],
            [3,  6,      7],
        ]).unwrap(); 
        
        assert!(num_misplaced_tiles(&goal, &goal) == 0);
        assert!(num_misplaced_tiles(&state, &goal) == 5);
    }
}