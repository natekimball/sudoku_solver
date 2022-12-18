pub fn solve(puzzle: &mut Vec<Vec<i32>>) -> bool {
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

// fn solve(mut puzzle: Vec<Vec<i32>>) -> Solution {
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
//     puzzle: Vec<Vec<i32>>,
//     solved: bool
// }

fn is_valid(puzzle: &Vec<Vec<i32>>, i: usize, j: usize, k: i32) -> bool {
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

fn is_complete(puzzle: &Vec<Vec<i32>>) -> bool {
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