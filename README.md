Sudoku Solver in Rust
=========
  - We are exploring different algorithm to speed up sudoku solving. See also: https://github.com/mozilla/rust/blob/master/src/test/bench/sudoku.rs
  - We are implementing "Constraint Propagation" using the same method described by Peter Norvig: http://norvig.com/sudoku.html.
  - We also use different data structure to solve the puzzle, by using a binary bit to store information in each grid, instead of hashmap and vector. Ex.000100 = 3, 010100 = 53, 000111 = 321 (a placement of each '1' bit corresponds to the actual number).
  - The solving algorithm is "back tracking," 1). store each move, 2). if we reach the deadend, we backup and go to the different route, and then repeat 1 again.
  - We are using heuristic, lowest number of possible value, to select where we first start to solve the puzzle.

Version
----

0.1
How to compile
--------------

```sh
rustc name_of_the_file.rs
```
How to run
--------------
```sh
./name_of_the_file -
```
Input
--------------
####Then input a puzzle as a string, 81 characters with 0 indicates the empty space.
For example:
```sh
003020600900305001001806400008102900700000008006708200002609500800203009005010300
```
corresponds to:
```sh
| .  .  3 | .  2  . | 6  .  . |
| 9  .  . | 3  .  5 | .  .  1 |
| .  .  1 | 8  .  6 | 4  .  . |
|- - - - -|- - - - -|- - - - -|
| .  .  8 | 1  .  2 | 9  .  . |
| 7  .  . | .  .  . | .  .  8 |
| .  .  6 | 7  .  8 | 2  .  . |
|- - - - -|- - - - -|- - - - -|
| .  .  2 | 6  .  9 | 5  .  . |
| 8  .  . | 2  .  3 | .  .  9 |
| .  .  5 | .  1  . | 3  .  . |
```
Potential Improvement
--------------
- We might take advantage of Rust's task to help speed up the process. For example, if one cell returns 89 after the first elimination. We know that either the answer should be 8 or 9 in this cell. We can then use task to help solve 2 paths: path 8 and path 9. 
- Because we are using bit and our data structure of bit is different, the process of changing it to int or str takes a lot of time. To clarify, we are using u16, when a digit in u16 bit turns to 1. It means that that digit is the number corresponding to the placement of that digit. For example: 000100 = 3, 010100 = 53. If we can find a way to speed up this conversion process. The solver will be much much faster.

---
####Note:
- File: Sudoku_propagate.rs will show you the result the first elimination through constraint propagation.

####Problem encountered
- We still have a problem with elimination algorithm in sudoku_improved.rs
Constraint propagation somehow doesn't check the neighbors or peers when it eliminates, and thus in some puzzles, it generates an incorrect result.

License
----

MIT

    
