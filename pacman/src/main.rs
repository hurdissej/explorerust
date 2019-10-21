extern crate termion;

fn main() {
    let board_size = 16;
    let mut board = vec![vec![BoardTile::Empty; board_size]; board_size];
    initialise_player(&mut board);
    print_board(&board, board_size);
}

//Function that borrows the boards current state and prints it 
fn print_board(board: &Vec<Vec<BoardTile>>, board_size: usize){
    for i in 0..board_size {
        print!("|");
        for j in 0..board_size{
            match board[i][j] {
                BoardTile::Empty => print!("."),
                BoardTile::Food => print!("o"),
                BoardTile::Enemy => print!("E"),
                BoardTile::Jewel => print!("J"),
                BoardTile::Pacman => print!("P")
            }
        }
        print!("|");
        println!("");
    }
}

fn initialise_player(board: &mut Vec<Vec<BoardTile>>){
    board[0][0] = BoardTile::Pacman;
}

#[derive(Copy, Clone)]
enum BoardTile {
    Food,
    Empty,
    Pacman,
    Jewel,
    Enemy
}