#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    hash::Hash,
    vec,
};

use rand::seq::IteratorRandom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn parse(c: &str) -> Option<Self> {
        match c {
            "c" => Some(Self::Up),
            "b" => Some(Self::Down),
            "e" => Some(Self::Left),
            "d" => Some(Self::Right),
            _ => None,
        }
    }

    const fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    const fn position_tuple(self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Left => (0, -1),
            Self::Down => (1, 0),
            Self::Right => (0, 1),
        }
    }

    fn iter() -> impl Iterator<Item = Self> {
        [Self::Up, Self::Down, Self::Left, Self::Right]
            .iter()
            .copied()
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Up => "c",
                Self::Down => "b",
                Self::Left => "e",
                Self::Right => "d",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub board: Vec<usize>,
    pub rows: usize,
    pub cols: usize,
    pub slot: usize,
    pub moves: Vec<Direction>,
    pub cost: usize,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.board.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.board.eq(&other.board)
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.moves.len() / 3 + other.cost).cmp(&(self.moves.len() / 3 + self.cost))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    #[must_use]
    pub fn new(board: Vec<usize>, rows: usize, cols: usize) -> Self {
        let slot = board
            .iter()
            .position(|&x| x == 0)
            .expect("There should be one `0` in the array");

        let mut a = Self {
            board,
            rows,
            cols,
            slot,
            cost: 0,
            moves: vec![],
        };

        a.cost = a.manhattan_distance();
        a
    }

    #[must_use]
    pub fn generate_random(rows: usize, cols: usize, amount: usize) -> Self {
        let mut a = Self {
            board: (0..rows * cols).collect(),
            rows,
            cols,
            slot: 0,
            cost: 0,
            moves: vec![],
        };

        for _ in 0..amount {
            let possible_moves = a.possible_moves();
            a = possible_moves
                .filter(|x| x.moves.last().map(|f| f.opposite()) != a.moves.last().copied())
                .choose(&mut rand::thread_rng())
                .expect("Expected possible move here.")
                .clone();
        }

        a
    }

    pub fn check_solution(&self, move_list: impl Iterator<Item = String>) -> Option<bool> {
        let mut final_state = self.clone();

        for m in move_list {
            let direction = Direction::parse(m.as_str())?;
            final_state = final_state.make_move(direction)?;
        }

        Some(final_state.is_final())
    }

    #[must_use]
    pub fn solve(&self) -> Option<Self> {
        let mut heap = BinaryHeap::with_capacity(100_000);
        heap.push(self.clone());

        let mut visited = HashSet::with_capacity(100_000);
        visited.insert(self.clone());

        loop {
            let current = heap.pop()?;

            if current.is_final() {
                return Some(current);
            }

            let possible_moves = current.possible_moves();

            for m in possible_moves {
                if visited.insert(m.clone()) {
                    heap.push(m);
                }
            }
        }
    }

    const fn index_to_position(&self, index: usize) -> (usize, usize) {
        (index / self.rows, index % self.cols)
    }

    const fn position_to_index(&self, position: (usize, usize)) -> usize {
        self.rows * position.0 + position.1
    }

    fn can_move(&self, direction: Direction) -> bool {
        let (i, j) = self.index_to_position(self.slot);

        match direction {
            Direction::Up => i > 0,
            Direction::Left => j > 0,
            Direction::Down => i < self.rows - 1,
            Direction::Right => j < self.cols - 1,
        }
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn make_move(&self, direction: Direction) -> Option<Self> {
        if self.can_move(direction) {
            let mut result = self.clone();
            let (i, j) = self.index_to_position(self.slot);
            let (ii, jj) = direction.position_tuple();

            let new_i = (i as isize) + ii;
            let new_j = (j as isize) + jj;
            let new_slot_index = self.position_to_index((new_i as usize, new_j as usize));

            result
                .board
                .swap(self.slot, new_slot_index);
            result.moves.push(direction);
            result.slot = new_slot_index;
            result.cost = result.manhattan_distance();

            Some(result)
        } else {
            None
        }
    }

    fn possible_moves(&self) -> impl Iterator<Item = Self> + '_ {
        Direction::iter().filter_map(|direction| self.make_move(direction))
    }

    fn manhattan_distance(&self) -> usize {
        (0..self.rows * self.cols)
            .map(|i| {
                if i == self.slot {
                    0
                } else {
                    let pos = self.index_to_position(self.board[i]);
                    let cor = self.index_to_position(i);
                    pos.0.abs_diff(cor.0) + pos.1.abs_diff(cor.1)
                }
            })
            .sum()
    }

    fn is_final(&self) -> bool {
        for i in 0..self.rows * self.cols {
            if self.board[i] != i {
                return false;
            }
        }

        true
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board_representation = self
            .board
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, "{} {}\n{board_representation}", self.rows, self.cols)
    }
}

impl TryFrom<String> for State {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut els = s
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok());

        let rows = els
            .next()
            .ok_or("Couldn't parse rows")?;
        let cols = els
            .next()
            .ok_or("Couldn't parse cols")?;

        let board = els.collect();

        Ok(Self::new(board, rows, cols))
    }
}
