use std::{fs, process::ExitCode};

use puzzle::State;
use std::env;

pub fn main() -> ExitCode {
    // argv[1] - saida da solução
    // argv[2] - arquivo de solução (aceito)
    // argv[3] - entrada do problema (arquivo)

    let args: Vec<String> = env::args().collect();

    let saida_solucao = args.get(1).unwrap();
    let entrada_problema = args.get(3).unwrap();

    let saida_string = fs::read_to_string(saida_solucao).unwrap();
    let entrada_string = fs::read_to_string(entrada_problema).unwrap();

    let board = State::try_from(entrada_string).unwrap();

    let correct = board.check_solution(
        saida_string
            .split_whitespace()
            .map(|x| x.to_string()),
    );

    if correct == Some(true) {
        println!("aceito");
        ExitCode::from(4)
    } else {
        println!("errado");
        ExitCode::from(6)
    }
}
