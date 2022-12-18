use std::{thread, time::{Duration, self}, env::args};
mod puzzle;
mod sudoku_solver;

fn main() {
    println!("SUDOKU SOLVER");

    let mut puzzle = puzzle::create_puzzle().unwrap();
    
    puzzle.iter().for_each(|row| println!("{:?}", row));

    let solution = thread::spawn(move || (sudoku_solver::solve(&mut puzzle), puzzle));
    
    let no_timeout = args().collect::<Vec<String>>().contains(&"--no-timeout".to_string());

    let start = time::Instant::now();
    let mut time = start;
    while !solution.is_finished() {
        let now = time::Instant::now();
        if now - time > Duration::from_secs(5) {
            println!("solving...");
            time = now;
        }
        if !no_timeout && now - start > Duration::from_secs(360) {
            panic!("Timed out while solving puzzle");
        }
    }

    let (solved, puzzle) = solution.join().unwrap();

    if solved {
        println!("solved!");
        puzzle.iter().for_each(|row| println!("{:?}", row));
    } else {
        println!("unsolvable puzzle");
    }
}