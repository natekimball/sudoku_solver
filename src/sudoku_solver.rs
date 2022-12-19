// backtracking depth-first algorithm
fn solve(puzzle: &mut Vec<Vec<u8>>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if puzzle[i][j] == 0 {
                // println!("puzzle[{i}][{j}]");
                for k in 1..10 {
                    if is_valid(&puzzle, i, j, k) {
                        // println!("puzzle[{i}][{j}] = {k}");
                        puzzle[i][j] = k;
                        if solve(puzzle) {
                            return true;
                        } else {
                            // println!("to puzzle[{i}][{j}]");
                            puzzle[i][j] = 0;
                        }
                    }
                }
                return false;
            }
        }
    }
    is_complete(puzzle)
}

fn is_valid(puzzle: &Vec<Vec<u8>>, i: usize, j: usize, k: u8) -> bool {
    for l in 0..9 {
        if puzzle[i][l] == k {
            return false;
        }
        if puzzle[l][j] == k {
            return false;
        }
    }
    
    let i_start = i - i % 3;
    let j_start = j - j % 3;
    for m in 0..3 {
        for n in 0..3 {
            if puzzle[m+i_start][n+j_start] == k {
                return false;
            }
        }
    }
    true
}

fn is_complete(puzzle: &Vec<Vec<u8>>) -> bool {
    for i in 0..9 {
        let mut row = vec![0;9];
        let mut col = vec![0;9];
        for j in 0..9 {
            if puzzle[i][j] == 0 || puzzle[j][i] == 0 {
                return false;
            }
            row[puzzle[i][j] as usize - 1] += 1;
            col[puzzle[j][i] as usize - 1] += 1;
        }
        if row.iter().any(|&x| x!=1) || col.iter().any(|&x| x!=1) {
            return false;
        }
    }
    
    for i in 0..3 {
        for j in 0..3 {
            let mut boxes = vec![0;9];
            for m in 0..3 {
                for n in 0..3 {
                    if puzzle[i*3+m][j*3+n] == 0 {
                        return false;
                    }
                    boxes[puzzle[i*3+m][j*3+n] as usize - 1] += 1;
                }
            }
            if boxes.iter().any(|&x| x!=1) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_empty_sudoku() {
        let mut puzzle = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        assert!(solve(&mut puzzle));
        assert!(is_complete(&puzzle));
    }
        
    #[test]
    fn solve_trivial_sudoku() {
        let mut puzzle = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![4, 5, 6, 7, 8, 0, 1, 2, 3],
            vec![7, 0, 9, 1, 2, 3, 4, 0, 6],
            vec![2, 1, 4, 3, 6, 5, 8, 9, 7],
            vec![3, 6, 5, 8, 0, 7, 2, 1, 4],
            vec![8, 9, 7, 2, 1, 4, 3, 6, 5],
            vec![5, 3, 1, 6, 4, 2, 9, 7, 8],
            vec![6, 0, 2, 9, 7, 8, 5, 3, 1],
            vec![9, 7, 0, 5, 3, 1, 6, 4, 0]
        ];
        assert!(solve(&mut puzzle));
        assert!(is_complete(&puzzle));
    }
    #[test]
    #[ignore = "takes a very long time"]
    fn solve_difficult_sudoku() {
        let mut puzzle = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 3, 0, 8, 5],
            vec![0, 0, 1, 0, 2, 0, 0, 0, 0],
            vec![0, 0, 0, 5, 0, 7, 0, 0, 0],
            vec![0, 0, 4, 0, 0, 0, 1, 0, 0],
            vec![0, 9, 0, 0, 0, 0, 0, 0, 0],
            vec![5, 0, 0, 0, 0, 0, 0, 7, 3],
            vec![0, 0, 2, 0, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 4, 0, 0, 0, 9]
        ];
        assert!(solve(&mut puzzle));
        assert!(is_complete(&puzzle));
    }

    #[test]
    fn impossible_puzzle() {
        let mut puzzle = vec![
            vec![0, 3, 5, 0, 6, 0, 7, 0, 0],
            vec![4, 6, 0, 1, 7, 3, 2, 8, 5],
            vec![8, 7, 1, 0, 0, 5, 4, 3, 6],
            vec![6, 1, 8, 0, 3, 7, 9, 4, 2],
            vec![0, 2, 4, 0, 9, 6, 1, 5, 0],
            vec![3, 0, 0, 0, 0, 1, 0, 0, 7],
            vec![5, 0, 6, 0, 8, 2, 0, 0, 0],
            vec![0, 0, 2, 3, 0, 0, 5, 6, 4],
            vec![1, 0, 3, 6, 4, 0, 8, 2, 9]
        ];
        assert!(!solve(&mut puzzle));
        assert!(!is_complete(&puzzle));
    }
    
    #[test]
    fn impossible_puzzle2() {
        let mut puzzle = vec![
            vec![2, 0, 0, 0, 0, 0, 0, 1, 0],
            vec![0, 0, 6, 0, 1, 3, 0, 8, 5],
            vec![0, 0, 1, 0, 2, 0, 0, 0, 0],
            vec![0, 8, 0, 5, 0, 7, 0, 0, 0],
            vec![0, 6, 4, 0, 0, 0, 1, 0, 0],
            vec![0, 9, 0, 8, 0, 0, 0, 0, 0],
            vec![5, 0, 0, 0, 0, 0, 0, 7, 3],
            vec![0, 0, 2, 0, 3, 0, 0, 0, 1],
            vec![6, 0, 0, 0, 4, 0, 0, 2, 9]
            ];
            assert!(!solve(&mut puzzle));
            assert!(!is_complete(&puzzle));
        }
}
    
// fn solve(mut puzzle: Vec<Vec<u8>>) -> Solution {
//     // println!("solving...");
//     // puzzle.iter().for_each(|row| println!("{:?}",row));
//     for i in 0..9 {
//         for j in 0..9 {
//             if puzzle[i][j] == 0 {
//                 // println!("puzzle[{i}][{j}]");
//                 for k in 1..10 {
//                     if is_valid(&puzzle, i, j, k) {
//                         // println!("puzzle[{i}][{j}] = {k}");
//                         puzzle[i][j] = k;
//                         let solution = solve(puzzle);
//                         // println!("puzzle[{i}][{j}]");
//                         if solution.solved {
//                             return solution;
//                         } else {
//                             puzzle = solution.puzzle;
//                             puzzle[i][j] = 0;
//                         }
//                     }
//                 }
//                 // println!("backtracking");
//                 return Solution {puzzle: puzzle, solved: false};
//             }
//         }
//     }
//     let solved = is_complete(&puzzle);
//     Solution { puzzle: puzzle, solved: solved }
// }

// pub struct Solution {
//     puzzle: Vec<Vec<u8>>,
//     solved: bool
// }