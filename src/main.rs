use std::io;

const PAYER_X: char = 'X';
const PAYER_O: char = 'O';

const BOARD_SIZE: usize = 3;

type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

fn initial_board() -> Board {
    [[' '; BOARD_SIZE]; BOARD_SIZE]
}



fn print_boad(board: &Board){
    for row in board {
        for cell in row{
            print!("{}",cell);

        }
        println!();
    }
}

fn get_player_move(current_player: char, board: &Board) -> (usize, usize) {
    loop {
        println!("Player {}, enter your move (row and column): ", current_player);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("faild to read input");
        println!("You entered: {}", input);  

        let cordinates: Vec<usize> = input
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect(); 

        if cordinates.len() == 2 {
            let (row, col) = (cordinates[0], cordinates[1]);
            if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col] == ' ' {
                return (row, col);
        }

        println!("Invalid move. Try again.");
    }
    }
}

fn check_winner(current_player:char,board:&Board)->bool{
   //row 
   for row in 0..BOARD_SIZE{//0
     if board[row][0]==current_player && board[row][1]==current_player && board[row][2]==current_player{
       return true;
     }
   }
   
    //col
    for col in 0..BOARD_SIZE{//0
        if board[0][col]==current_player && board[1][col]==current_player && board[2][col]==current_player{
          return true;
        }
    }

    //diagonal

    if (board[0][0]==current_player && board[1][1]==current_player && board[2][2]==current_player) 
       || (board[0][2]==current_player && board[1][1]==current_player && board[2][0]==current_player)
    {
        return true;
    }
   
   return false;
}


fn check_draw(board:&Board)->bool{
    for row in board{
        for cell in row{
            if *cell==' '{//there should be no empty slots
                return false;
            }
        }
    }
    return true;
}




fn play_game() {

    let mut board = initial_board();
    let mut current_palyer = PAYER_X;

    loop {
    println!("current board:");
    print_boad(&board);

    let (row, col) = get_player_move(current_palyer, &board);
    board[row][col] = current_palyer;

    if check_winner(current_palyer, &board) {
        println!("Player {} wins!", current_palyer);
        print_boad(&board);
        break;
    }

    if check_draw(&board){
        println!("The game is draw");
        break;
    }


    current_palyer = if current_palyer == PAYER_X {
        PAYER_O
    } else {
        PAYER_X
    };

    
}
}



fn main() {

    println!("Welcome to Tic-Tac-Toe!");

    play_game();
}