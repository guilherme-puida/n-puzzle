use itertools::Itertools;
use puzzle::State;

pub fn main() {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .unwrap();

    let (rows, cols) = line
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .next_tuple()
        .unwrap();

    std::io::stdin()
        .read_line(&mut line)
        .unwrap();

    let board: Vec<usize> = line
        .split_whitespace()
        .dropping(2)
        .filter_map(|x| x.parse().ok())
        .collect();

    let moves = State::new(board, rows, cols)
        .solve()
        .unwrap()
        .moves;

    println!("{}", moves.iter().join(" "));
}
