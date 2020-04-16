use std::hash::{
    Hash, 
    Hasher,
};
use std::cmp::Ordering;
use std::mem;
use std::collections::HashSet;

pub const BLANK: u8 = 9;

#[derive(Copy, Clone, Debug)]
pub struct State {
    grid: [[u8; 3]; 3],
    blank_position: (usize, usize), 
}

pub struct StateIterator<'a> {
    grid: &'a[[u8; 3]; 3],
    row: usize,
    col: usize,
}

impl State {
    pub fn new(grid: [[u8; 3]; 3]) -> Option<State> {
        let mut used = HashSet::new();
        let mut blank_position = (0, 0);
        
        for i in 0..3 {
            for j in 0..3 {
                let x = grid[i][j];

                if !State::is_valid_cell_value(x) || used.contains(&x) {
                    return None;
                }
                else if x == BLANK {
                    blank_position = (i, j);
                }

                used.insert(x);
            }
        }
        
        Some(State{
            grid,
            blank_position,
        })
    }

    fn is_valid_cell_value(x: u8) -> bool {
        x == BLANK || x < 9 
    }

    pub fn iter(&self) -> StateIterator {
        StateIterator {
            grid: &self.grid,
            row: 0,
            col: 0,
        }
    } 

    pub fn move_up(&self) -> Option<State> {
        let (i, j) = self.blank_position;
        if i > 0 {
            let mut result = *self;
            let temp = result[i - 1][j];
            result[i - 1][j] = result[i][j];
            result[i][j] = temp;
            Some(result)
        } 
        else {
            None
        }
    }

    pub fn move_down(&self) -> Option<State> {
        let (i, j) = self.blank_position;
        if i < 2 {
            let mut result = *self;
            let temp = result[i + 1][j];
            result[i + 1][j] = result[i][j];
            result[i][j] = temp;
            Some(result)
        } 
        else {
            None
        }
    }

    pub fn move_left(&self) -> Option<State> {
        let (i, j) = self.blank_position;
        if j > 0 {
            let mut result = *self;
            swap(&mut result[i], j - 1, j);
            Some(result)
        } 
        else {
            None
        }
    }

    pub fn move_right(&self) -> Option<State> {
        let (i, j) = self.blank_position;
        if j < 2 {
            let mut result = *self;
            swap(&mut result[i], j, j + 1);
            Some(result)
        } 
        else {
            None
        }
    }
}

pub fn immediate_neighbours(s: &State) -> Vec<State> {
    [s.move_up(), s.move_down(), s.move_left(), s.move_right()]
    .into_iter()
    .filter_map(|opt| *opt)
    .collect()
}

impl std::ops::Index<usize> for State {
    type Output = [u8; 3];

    fn index(&self, i: usize) -> &Self::Output {
        &self.grid[i]
    }
}

impl std::ops::IndexMut<usize> for State {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.grid[i]
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self.grid[i][j] != other.grid[i][j] {
                    return false;
                }
            }
        }
        
        true
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        for i in 0..3 {
            let s: u8 = self.grid[i].iter().sum();
            s.hash(hasher); 
        }
    }
}

impl<'a> std::iter::Iterator for StateIterator<'a> {
    type Item = u8;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.row > 2 {
            return None;
        }
        
        let result = Some(self.grid[self.row][self.col]);

        let (i, j) = {
            let col = self.col + 1;
            if col > 2 { (self.row + 1, 0) } else { (self.row, col) }
        };
        self.row = i;
        self.col = j;

        result
    }
}

fn swap<T>(x: &mut [T], i: usize, j: usize) {
    let (lo, hi) = match i.cmp(&j) {
        Ordering::Equal => return,
        Ordering::Less => (i, j),
        Ordering::Greater => (j, i),
    };
    let (init, tail) = x.split_at_mut(hi);
    mem::swap(&mut init[lo], &mut tail[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn states_allow_no_duplicates() {
        let state = State::new([
                [1,     2,   3],
                [1,     4,   5],
                [BLANK, 6,   7],
        ]);
        assert!(state.is_none());
    }

    #[test]
    fn states_validate_the_values() {
        let state = State::new([
                [1,     2,   10],
                [1,     4,   5],
                [BLANK, 6,   7],
        ]);
        assert!(state.is_none());
    }

    #[test]
    fn valid_state_is_some() {
        let state = State::new([
                [1,     2,   3],
                [4,     8,   5],
                [BLANK, 6,   7],
        ]);
        assert!(state.is_some());
    }

    #[test]
    fn move_up_and_right_valid() {
        let state = State::new([
                [1,     2,   3],
                [4,     8,   5],
                [BLANK, 6,   7],
        ]).unwrap();

        assert_eq!(state.move_up().unwrap(),
        State::new([
                [1,     2,   3],
                [BLANK, 8,   5],
                [4,     6,   7],
        ]).unwrap());

        assert_eq!(state.move_right().unwrap(),
        State::new([
               [1,  2,     3],
               [4,  8,     5],
               [6,  BLANK, 7],
       ]).unwrap()); 
    }

    #[test]
    fn move_up_and_right_invalid() {
        let state = State::new([
                [1,     2,   BLANK],
                [4,     8,       5],
                [3,     6,       7],
        ]).unwrap();

        assert!(state.move_up().is_none());
        assert!(state.move_right().is_none());
    }
    
    #[test]
    fn move_down_and_left_invalid() {
        let state = State::new([
                [1,     2,   3],
                [4,     8,   5],
                [BLANK, 6,   7],
        ]).unwrap();

        assert!(state.move_down().is_none());
        assert!(state.move_left().is_none());
    }

    #[test]
    fn move_down_and_left_valid() {
        let state = State::new([
                [1,  2,  BLANK],
                [4,  8,      5],
                [3,  6,      7],
        ]).unwrap();

        assert_eq!(state.move_down().unwrap(),
        State::new([
            [1,  2,      5],
            [4,  8,  BLANK],
            [3,  6,      7],
        ]).unwrap());
        assert_eq!(state.move_left().unwrap(),
        State::new([
            [1,  BLANK,  2],
            [4,  8,      5],
            [3,  6,      7],
        ]).unwrap());
    }

    #[test]
    fn immediate_neighbours_with_blank_at_the_center() {
        let state = State::new([
            [1,  2,      8],
            [4,  BLANK,  5],
            [3,  6,      7],
        ]).unwrap();
        
        let neighbours = immediate_neighbours(&state);

        assert_eq!(neighbours, vec![
            state.move_up().unwrap(),
            state.move_down().unwrap(),
            state.move_left().unwrap(),
            state.move_right().unwrap(),
        ]);
    }

    #[test]
    fn iterating_states() {
        let state = State::new([
            [1,  2,      8],
            [4,  BLANK,  5],
            [3,  6,      7],
        ]).unwrap();
        
        let tiles = state.iter().collect::<Vec<u8>>();
        
        assert_eq!(tiles, vec![1, 2, 8, 4, BLANK, 5, 3, 6, 7]);
    }
}