use std::{env::args, io::{self, Error, ErrorKind}, fs, fmt::{Display, Formatter}};

use crate::sudoku_solver;

pub struct Puzzle {
    puzzle: Vec<Vec<u8>>,
    pub solved: bool
}
impl Puzzle {
    pub fn new() -> Self {
        let puzzle = create_puzzle().unwrap();
        Self { puzzle, solved: false }
    }

    pub fn solve(&mut self) {
        self. solved = sudoku_solver::solve(&mut self.puzzle);
    }

    pub(crate) fn show_solution(&self) {
        if self.solved {
            println!("solved!");
            println!("{self}");
        } else {
            println!("unsolvable puzzle");
        }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.puzzle.iter().enumerate() {
            if i%3==0 && i!=0 {
                writeln!(f, "------+-------+------")?;
            }
            for (j, num) in row.iter().enumerate() {
                if j%3==0 && j!=0 {
                    write!(f, "| ")?;
                }
                write!(f, "{} ", num)?;
            }
            if i!=self.puzzle.len()-1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn create_puzzle() -> Result<Vec<Vec<u8>>, io::Error> {
    let args = args().collect::<Vec<String>>();
    let puzzle;
    if args.contains(&"-i".to_string()) {
        puzzle = puzzle_from_input();
    } else if args.contains(&"-f".to_string()) {
        let index = args.iter().position(|x| x == "-f").unwrap();
        puzzle = puzzle_from_file(args.get(index+1).expect("No file specified after -f"));
    } else {
        puzzle = puzzle_from_file("resources/medium.txt");
    }
    if puzzle.len()!=9 || puzzle[0].len()!=9 || !solvable(&puzzle) {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid sudoku puzzle."));
    }
    Ok(puzzle)
}

fn puzzle_from_input() -> Vec<Vec<u8>> {
    let mut puzzle = Vec::with_capacity(9);
    println!("enter each row as numbers separated by spaces, followed by enter");
    println!("blank spaces should be represented by 0");
    println!(" ex. '0 0 1 9 2 3 0 0 4'");
    let mut i = 0;
    while i<9 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().split_whitespace().map(|num| num.parse::<u8>().unwrap_or(10)).collect::<Vec<u8>>();
        if input.iter().any(|&x| x > 9) || input.len() != 9 {
            println!("Invalid input. Please try again enter 9 digits between 0 and 9:");
        } else {
            puzzle.push(input);
            i+=1;
        }
    }
    puzzle
}

fn puzzle_from_file(file: &str) -> Vec<Vec<u8>> {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    contents.lines().map(|line| line.split_whitespace().map(|num| num.parse::<u8>().unwrap()).collect()).collect()
}


fn solvable(puzzle: &Vec<Vec<u8>>) -> bool {
    for i in 0..9 {
        let mut row = vec![0;9];
        let mut col = vec![0;9];
        for j in 0..9 {
            if puzzle[i][j] == 0 || puzzle[j][i] == 0 {
                continue;
            } if puzzle[i][j] > 9 || puzzle[j][i] > 9 {
                return false;
            }
            row[puzzle[i][j] as usize - 1] += 1;
            col[puzzle[j][i] as usize - 1] += 1;
        }
        if row.iter().any(|&x| x>1) || col.iter().any(|&x| x>1) {
            return false;
        }
    }

    for i in 0..3 {
        for j in 0..3 {
            let mut boxes = vec![0;9];
            for m in 0..3 {
                for n in 0..3 {
                    if puzzle[i*3+m][j*3+n] == 0 {
                        continue;
                    } if puzzle[i*3+m][j*3+n] > 9 {
                        return false;
                    }
                    boxes[puzzle[i*3+m][j*3+n] as usize - 1] += 1;
                }
            }
            if boxes.iter().any(|&x| x>1) {
                return false;
            }
        }
    }
    true
}