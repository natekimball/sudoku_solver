use std::{thread, time::{Duration, self}, env::args};

use crate::puzzle::Puzzle;
mod puzzle;
mod sudoku_solver;

fn main() {
    let mut puzzle = Puzzle::new();

    println!("puzzle:");
    println!("{puzzle}");

    let solution = thread::spawn(move || {
        puzzle.solve();
        puzzle
    });

    let time = timer(&solution);
    println!("solved in {} seconds", time.as_secs_f64());
    
    let puzzle = solution.join().unwrap();
    puzzle.show_solution();
}

fn timer(solution: &thread::JoinHandle<Puzzle>) -> Duration {
    let no_timeout = args().collect::<Vec<String>>().contains(&"--no-timeout".to_string());
    let start = time::Instant::now();
    let mut time = start;
    while !solution.is_finished() {
        let now = time::Instant::now();
        if now - time > Duration::from_secs(6) {
            println!("solving...");
            time = now;
        }
        if !no_timeout && now - start > Duration::from_secs(360) {
            panic!("Timed out while solving puzzle");
        }
    }
    time::Instant::now() - start
}