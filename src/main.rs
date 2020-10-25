use std::io;

const NUM_FIELDS: usize = 9;

struct Board {
    tokens: [char; NUM_FIELDS],
}

fn print_board(board: &Board) {
    // Prrint empty  newline to split current board from old
    println!();

    for (i, token) in board.tokens.iter().enumerate() {
        // Print the tokens, add newline at specific indexes.
        if i == 2 || i == 5 || i == 8 {
            println!("{}", token);
        } else {
            print!("{}|", token);
        }
    }

    println!();
}

fn set_field(board: &mut Board, field: usize, token: char) {
    board.tokens[field] = token;
}

fn check_win(board: &Board) -> bool {
    false
}

fn main() -> io::Result<()> {
    let won = false;

    println!(
        "Welcome to Tic Tac Toe in Rust!
Player1 will now begin as 'X', please place your first cross."
    );

    let mut board = Board {
        tokens: ['_'; NUM_FIELDS],
    };

    // Game loop
    let mut player_counter = 0;

    while !won {
        player_counter += 1;

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if player_counter % 2 == 0 {
            set_field(&mut board, index, 'O');
        } else {
            set_field(&mut board, index, 'X');
        }

        print_board(&board);
    }

    Ok(())
}
