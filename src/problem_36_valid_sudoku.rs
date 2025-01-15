use std::collections::HashMap;

/*
Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

    Each row must contain the digits 1-9 without repetition.
    Each column must contain the digits 1-9 without repetition.
    Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.

Note:

    A Sudoku board (partially filled) could be valid but is not necessarily solvable.
    Only the filled cells need to be validated according to the mentioned rules.

    Input: board = 
[['5','3','.','.','7','.','.','.','.']
, vec!['6','.','.','1','9','5','.','.','.']
, vec!['.','9','8','.','.','.','.','6','.']
, vec!['8','.','.','.','6','.','.','.','3']
, vec!['4','.','.','8','.','3','.','.','1']
, vec!['7','.','.','.','2','.','.','.','6']
, vec!['.','6','.','.','.','.','2','8','.']
, vec!['.','.','.','4','1','9','.','.','5']
, vec!['.','.','.','.','8','.','.','7','9']]
Output: true

Example 2:

Input: board = 
[['8','3','.','.','7','.','.','.','.']
, vec!['6','.','.','1','9','5','.','.','.']
, vec!['.','9','8','.','.','.','.','6','.']
, vec!['8','.','.','.','6','.','.','.','3']
, vec!['4','.','.','8','.','3','.','.','1']
, vec!['7','.','.','.','2','.','.','.','6']
, vec!['.','6','.','.','.','.','2','8','.']
, vec!['.','.','.','4','1','9','.','.','5']
, vec!['.','.','.','.','8','.','.','7','9']]
Output: false
Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
*/
use crate::Solution;

impl Solution {
    pub fn get_box(position: (usize, usize)) -> usize {
        ((position.0 / 3) * 10) + (position.0 / 3)
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut n_line = 1;
        let mut n_colun = 0;
        let mut seen_elements: HashMap<char, (usize, usize)> = HashMap::new();

        for line in board {
            for block in line { 
                n_colun += 1;              
                
                match seen_elements.get(&block) {
                    Some(coord) => {
                    if Solution::get_box(*coord) == Solution::get_box((n_line, n_colun)) {
                            seen_elements.insert(block, (n_line, n_colun));
                            println!("box: {}", Solution::get_box((n_line, n_colun)));
                        }  
                        seen_elements.insert(block, (n_line, n_colun));
                    },
                    None => {
                        seen_elements.insert(block, (n_line, n_colun));
                    }
                }
                

                seen_elements.insert(block, (n_line, n_colun));
                
            }

            n_line += 1;
            n_colun = 0;
        } 

        false
    }
}

#[test]
fn test_is_valid() {
    Solution::is_valid_sudoku(vec![vec!['5','3','.','.','7','.','.','.','.'], vec!['6','.','.','1','9','5','.','.','.'], vec!['.','9','8','.','.','.','.','6','.'], vec!['8','.','.','.','6','.','.','.','3'], vec!['4','.','.','8','.','3','.','.','1'], vec!['7','.','.','.','2','.','.','.','6'], vec!['.','6','.','.','.','.','2','8','.'], vec!['.','.','.','4','1','9','.','.','5'], vec!['.','.','.','.','8','.','.','7','9']]);
}