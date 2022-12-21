use std::env;

use puzzle::State;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = args
        .get(1)
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap();
    let cols = args
        .get(2)
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap();
    let move_amount = args
        .get(3)
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap();

    let problem = State::generate_random(lines, cols, move_amount);
    println!("{}", problem);
}
