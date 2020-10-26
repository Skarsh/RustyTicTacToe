use std::io;

const NUM_FIELDS: usize = 9;

#[derive(Copy, Clone, PartialEq)]
/// The state of a field on the board. Can be a cross, circle or blank.
enum State {
    X,
    O,
    Blank,
}

/// The game board is being represented here.
struct Board {
    /// The board must contain fields, 9 to be exact, which are stored in an array of States.
    fields: [State; NUM_FIELDS],
}

/// Prints the state of the board, returns ()
fn print_board(board: &Board) {
    // Print empty  newline to split current board from old
    println!();

    for (i, field) in board.fields.iter().enumerate() {
        // Print the fields, add newline at specific indexes.
        if i == 2 || i == 5 || i == 8 {
            match field {
                State::X => println!("X"),
                State::O => println!("O"),
                State::Blank => println!(" "),
            }
        } else {
            match field {
                State::X => print!("X|"),
                State::O => print!("O|"),
                State::Blank => print!(" |"),
            }
        }
    }

    println!();
}

/// Checks whether the field on the board is free, returns the result as a bool.
fn check_free_field(board: &Board, index: usize) -> bool {
    board.fields[index] == State::Blank
}

/// Sets the value of a specified field on the board, returns ().
fn set_field(board: &mut Board, index: usize, state: State) {
    board.fields[index] = state;
}

/// Returns the boolean value of the win condition and the player who won.
fn check_win(board: &Board) -> (bool, u8) {
    let fields = board.fields;

    // First row
    if fields[0] == State::X && fields[1] == State::X && fields[2] == State::X {
        (true, 1)
    } else if fields[0] == State::O && fields[1] == State::O && fields[2] == State::O {
        (true, 2)
    }
    // Second row
    else if fields[3] == State::X && fields[4] == State::X && fields[5] == State::X {
        (true, 1)
    } else if fields[3] == State::O && fields[4] == State::O && fields[5] == State::O {
        (true, 2)
    }
    // Third row
    else if fields[6] == State::X && fields[7] == State::X && fields[8] == State::X {
        (true, 1)
    } else if fields[6] == State::O && fields[7] == State::O && fields[8] == State::O {
        (true, 2)
    }
    // First Column
    else if fields[0] == State::X && fields[3] == State::X && fields[6] == State::X {
        (true, 1)
    } else if fields[0] == State::O && fields[3] == State::O && fields[6] == State::O {
        (true, 2)
    }
    // Second Column
    else if fields[1] == State::X && fields[4] == State::X && fields[7] == State::X {
        (true, 1)
    } else if fields[1] == State::O && fields[4] == State::O && fields[7] == State::O {
        (true, 2)
    }
    // Third Column
    else if fields[2] == State::X && fields[5] == State::X && fields[8] == State::X {
        (true, 1)
    } else if fields[2] == State::O && fields[5] == State::O && fields[8] == State::O {
        (true, 2)
    }
    // First diagonal
    else if fields[0] == State::X && fields[4] == State::X && fields[8] == State::X {
        (true, 1)
    } else if fields[0] == State::O && fields[4] == State::O && fields[8] == State::O {
        (true, 2)
    }
    // Second diagonal
    else if fields[2] == State::X && fields[4] == State::X && fields[6] == State::X {
        (true, 1)
    } else if fields[2] == State::O && fields[4] == State::O && fields[6] == State::O {
        (true, 2)
    } else {
        (false, 0)
    }
}

fn main() -> io::Result<()> {
    let won = false;

    println!(
        "Welcome to Tic Tac Toe in Rust!
Player1 will now begin as 'X', please place your first cross."
    );

    let mut board = Board {
        fields: [State::Blank; NUM_FIELDS],
    };

    // Game loop
    let mut player_counter = 1;

    print_board(&board);

    while !won {
        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if !check_free_field(&board, index) {
            println!("The field is already occupied! Try again.");
            continue;
        }

        if player_counter % 2 == 0 {
            set_field(&mut board, index, State::O);
        } else {
            set_field(&mut board, index, State::X);
        }

        let res = check_win(&board);
        if res.0 {
            println!("Player {} won!", res.1);
            print_board(&board);
            break;
        }

        print_board(&board);

        // Increment the player count before next player.
        player_counter += 1;
    }

    Ok(())
}
