struct Solution;

impl Solution {
    pub fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize, c: char) -> bool {
        for i in 0..9{
            if board[i][col] == c && i != row{
                return false;
            }
            if board[row][i] == c && i != col{
                return false;
            }
            let x = 3 * (row / 3) + i / 3;
            let y = 3 * (col / 3) + i % 3;
            if board[x][y] == c && (x != row && y != col){
                return false;
            }
        }
        return true;
    }


    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..board.len(){
            for j in 0..board[i].len(){
                if board[i][j] != '.'{
                    if !Solution::is_valid(&board, i, j, board[i][j]){
                    println!("{} {} {}", i, j, board[i][j]);
                        return false;
                    }
                }
            }

        }
     return true;   
    }
}

fn main() {
    let board = 
        vec![vec!['5','3','.','.','7','.','.','.','.']
        ,vec!['6','.','.','1','9','5','.','.','.']
        ,vec!['.','9','8','.','.','.','.','6','.']
        ,vec!['8','.','.','.','6','.','.','.','3']
        ,vec!['4','.','.','8','.','3','.','.','1']
        ,vec!['7','.','.','.','2','.','.','.','6']
        ,vec!['.','6','.','.','.','.','2','8','.']
        ,vec!['.','.','.','4','1','9','.','.','5']
        ,vec!['.','.','.','.','8','.','.','7','9']];

    println!("{}",Solution::is_valid_sudoku(board));
}
