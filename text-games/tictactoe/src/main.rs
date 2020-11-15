// tic-tac-toe game
//
use rand::Rng;

#[derive(PartialEq, Eq)]
enum CellState {
    StateEmpty,
    StatePlayer,
    StateComputer,
}

fn main() {
    let mut state = [
        CellState::StateEmpty,
        CellState::StateEmpty,
        CellState::StateEmpty,
        CellState::StateEmpty,
        CellState::StateEmpty,
        CellState::StateEmpty,
        CellState::StateEmpty,
        CellState::StateEmpty,
        CellState::StateEmpty,
    ];

    println!("Let's play Tic-Tac-Toe!");

    loop {
        // draw game board
        draw_board(&state);

        // player takes turn
        let guess = get_player_input(&state);
        if guess == 10 {
            println!("Invalid choice");
            continue;
        }

        state[guess] = CellState::StatePlayer;

        // check for player win
        if check_win(&state, CellState::StatePlayer) {
            println!("\nPLAYER WINS!");
            break;
        }

        // check for draw
        if check_draw(&state) {
            println!("\nDRAW!");
            break;
        }

        // computer takes turn
        get_computer_input(&mut state);

        // check for computer win
        if check_win(&state, CellState::StateComputer) {
            println!("\nCOMPUTER WINS!");
            break;
        }
    }

    // draw final game board
    draw_board(&state);
}

fn check_win(state: &[CellState; 9], who: CellState) -> bool {
    // check rows
    for row in 0..3 {
        let idx: usize = row * 3 + 0;
        if who == state[idx] && who == state[idx + 1] && who == state[idx + 2] {
            return true;
        }
    }

    // check columns
    for col in 0..3 {
        let idx: usize = col;
        if who == state[idx] && who == state[idx + 3] && who == state[idx + 6] {
            return true;
        }
    }

    // check diagonals
    if who == state[4] {
        if who == state[0] && who == state[8] {
            return true;
        } else if who == state[2] && who == state[6] {
            return true;
        }
    }

    // failed
    false
}

fn get_computer_input(state: &mut [CellState; 9]) {
    let mut options: Vec<usize> = Vec::new();

    for i in 0..9 {
        if CellState::StateEmpty == state[i] {
            options.push(i);
        }
    }

    let sel = rand::thread_rng().gen_range(0, options.len());

    match state[options[sel]] {
        CellState::StateEmpty => state[options[sel]] = CellState::StateComputer,
        _ => (),
    }
}

fn check_draw(state: &[CellState; 9]) -> bool {
    for i in 0..9 {
        if CellState::StateEmpty == state[i] {
            return false;
        }
    }

    true
}

fn draw_board(state: &[CellState; 9]) {
    let mut board = Vec::new();

    for i in 0..9 {
        match state[i] {
            CellState::StatePlayer => board.push(String::from("X")),
            CellState::StateComputer => board.push(String::from("O")),
            CellState::StateEmpty => board.push((i + 1).to_string()),
        }
    }

    println!("/-----------\\");
    println!("| {} | {} | {} |", board[0], board[1], board[2]);
    println!("|---|---|---|");
    println!("| {} | {} | {} |", board[3], board[4], board[5]);
    println!("|---|---|---|");
    println!("| {} | {} | {} |", board[6], board[7], board[8]);
    println!("\\-----------/");
}

fn get_player_input(state: &[CellState; 9]) -> usize {
    println!("Enter the # for your choice (X)");

    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Expected line");

    let guess = guess.as_bytes()[0];
    if guess < 49 || guess > 57 {
        return 10; // invalid choice
    }

    let guess: usize = (guess - 49).into();

    if state[guess] == CellState::StateEmpty {
        return guess;
    }

    10 // invalid choice
}
