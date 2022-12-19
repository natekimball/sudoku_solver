# Sudoku Solver
Sudoku solver written in Rust.

Run with -i to enter sudoku puzzle as input, or -f followed by the filename to read puzzle from a file.
Run with --no-timeout to disable the timeout for computing harder puzzles.
0's represent empty spaces.

## Examples
```
$ cargo run . -i
SUDOKU SOLVER
enter each row as numbers separated by spaces, followed by enter
blank spaces should be represented by 0
 ex. '0 0 1 9 2 3 0 0 4'
1 2 3 4 5 6 7 8 9
4 5 6 7 8 0 1 2 3
7 0 9 1 2 3 4 0 6
2 1 4 3 6 5 8 9 7
3 6 5 8 0 7 2 1 4
8 9 7 2 1 4 3 6 5
5 3 1 6 4 2 9 7 8
6 0 2 9 7 8 5 3 1
9 7 0 5 3 1 6 4 0

$ cargo run . -f resources/medium.txt

$ cargo run . -f resources/hard.txt --no-timeout

resources/medium.txt:
5 3 0 0 7 0 0 0 0
6 0 0 1 9 5 0 0 0
0 9 8 0 0 0 0 6 0
8 0 0 0 6 0 0 0 3
4 0 0 8 0 3 0 0 1
7 0 0 0 2 0 0 0 6
0 6 0 0 0 0 2 8 0
0 0 0 4 1 9 0 0 5
0 0 0 0 8 0 0 7 9
'''