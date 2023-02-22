use std::io;

fn display_board(board: &[[char; 3]; 3]) {
    for row in board.iter() {
        for &cell in row.iter() {
            print!("{} ", cell);
        }
        println!("");
    }
}

fn check_win(board: &[[char; 3]; 3]) -> bool {
    for row in 0..3 {
        if board[row][0] != ' ' && board[row][0] == board[row][1] && board[row][1] == board[row][2] {
            return true;
        }
    }

    for col in 0..3 {
        if board[0][col] != ' ' && board[0][col] == board[1][col] && board[1][col] == board[2][col] {
            return true;
        }
    }

    if board[0][0] != ' ' && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return true;
    }

    if board[0][2] != ' ' && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return true;
    }

    return false;
}

fn check_draw(board: &[[char; 3]; 3]) -> bool {
    for row in board.iter() {
        for &cell in row.iter() {
            if cell == ' ' {
                return false;
            }
        }
    }

    return true;
}

fn main() {
    let mut board = [[' ';3];3];
    let mut player = 'X';
    loop {
        display_board(&board);
        println!("Player {}'s turn. Please enter row and column:", player);
        let mut move_input = String::new();
        io::stdin().read_line(&mut move_input).unwrap();
        let move_vec: Vec<usize> = move_input.trim().split(" ").map(|x| x.parse().unwrap()).collect();
        let (row, col) = (move_vec[0], move_vec[1]);
        if board[row][col] == ' ' {
            board[row][col] = player;
            if check_win(&board) {
                println!("Player {} wins!", player);
                break;
            } else if check_draw(&board) {
                println!("It's a draw!");
                break;
            }
            player = if player == 'X' { 'O' } else { 'X' };
        } else {
            println!("Invalid move, please try again");
        }
    }
}
