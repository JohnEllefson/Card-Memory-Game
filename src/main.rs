use std::io::prelude::*; 
use rand::Rng;
use std::io;



// ## COORDINATE ##
#[derive(Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn default() -> Coordinate {
        Coordinate {
            x: 0,
            y: 0,
        }
    }

    fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x: x as usize, y: y as usize }
    }
}


//  ## CARD ##
#[derive(Copy, Clone)]
struct Card {
    is_active: bool,
    is_flipped: bool,
    symbol: char,
}

impl Card { 
    fn default() -> Card {
        Card {
            is_active: false,
            is_flipped: false,
            symbol: 'X',
        }
    }
    
    fn new(symbol_param: char) -> Card {
        Card {
            is_active: false,
            is_flipped: false,
            symbol: symbol_param,
        }
    }
}





// Start of the program
fn main() {
    let mut board: [[Card; 4]; 4] = [[Card::default(); 4]; 4];
    let mut is_game_over: bool = false;
    let mut loop_iterations = 0;
    setup_board(&mut board);

    // Run the game loop until the board is empty
    while !is_game_over {
        display_board(&mut board);
        request_input(&mut board);
        is_game_over = check_if_game_over(&mut board);
        loop_iterations += 1;
    }

    // Updates board and ends the game
    display_board(&mut board);
    println!("{}{}{}", "\nCongratulations, you won in ", loop_iterations / 2, " turns!");
}



// Sets up a new 4x4 board by randomly placing cards onto it 
fn setup_board (board: &mut [[Card; 4]; 4]) {
    let symbols = ['!', '@', '#', '$', '%','&', '*', '+'];
    let mut rng = rand::thread_rng();

    // Fill the board with 2 cards with each symbol
    for _ in 0..2 {
        for symbol in symbols 
        {
            let mut was_card_added: bool = false;

            // Look for coordinates with an inactive card to update
            while !was_card_added 
            {
                // Pick a random empty coordinate
                let rand_row = rng.gen_range(0..4);
                let rand_col = rng.gen_range(0..4);

                // If the current position hasn't been given a new Card, add a card to the board with the current
                if !board[rand_row][rand_col].is_active 
                {
                    board[rand_row][rand_col] = Card {is_active: true, is_flipped: false, symbol: char::from(symbol)};
                    was_card_added = true;
                }
            }
        }
    }
}



// Displays the current board
fn display_board (board: &mut [[Card; 4]; 4]) {
    // Create and initialize an array of strings for the row symbols
    let mut row_symbols: [String; 4] = [String::from(""), String::from(""), String::from(""), String::from("")];

    // Populate row_symbols with a string the symbols to display on each row 
    for i in 0..4 {
        for j in 0..4 {
            // Add spacing inbetween each card's symbol
            row_symbols[i].push_str("  ");

            // Card is active and flipped, add card's symbol
            if board[i][j].is_active && board[i][j].is_flipped {
                row_symbols[i].push(board[i][j].symbol);
            }

            // Card is active and not flipped, add a 'c' to indicate an unflipped card
            else if board[i][j].is_active && !board[i][j].is_flipped {
                row_symbols[i].push('c');
            }

            // Card is not active, add a space
            else {
                row_symbols[i].push('-');
            }

        }
    }

    // Clear the terminal
    print! ("\x1B[2J\x1B[1;1H");

    // Print the board    
    println!("{}", "   A  B  C  D");
    println!("{}{}", "1", row_symbols[0]);
    println!("{}{}", "2", row_symbols[1]);
    println!("{}{}", "3", row_symbols[2]);
    println!("{}{}", "4", row_symbols[3]);
 }



// Requests a coordinate from the user and acts accordingly
 fn request_input(board: &mut [[Card; 4]; 4]) {
    let mut chosen_coord = Coordinate::default();
    let mut found_coord = false;

    // Request a coordinate from the user until a valid one is entered
    while !found_coord {
        // Read in user input
        print!("\nEnter coordinate to select a card: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("failed to read line");
        let input = input.trim().to_string();

        // Ensure the user entered coordinate is 2 characters with 1 being alphabetical and
        // the other numerical. If so, generate an actual board coordinate and select the card there.
        if input.len() == 2 {
            if input.chars().nth(0).unwrap().is_alphabetic() && input.chars().nth(1).unwrap().is_numeric() {
                chosen_coord = generate_board_coord(input.chars().nth(0).unwrap(), input.chars().nth(1).unwrap());
                found_coord = select_board_coord(board, chosen_coord);
            }
            else if input.chars().nth(0).unwrap().is_numeric() && input.chars().nth(1).unwrap().is_alphabetic() {
                chosen_coord = generate_board_coord(input.chars().nth(1).unwrap(), input.chars().nth(0).unwrap());
                found_coord = select_board_coord(board, chosen_coord);
            }
        }

        // Error message if an invalid coordinate was entered
        if !found_coord { println!("Invalid coordinate entered. Try again."); }
    }
 }



// Creates a Coordinate from the given alpha and numerical chars passed in
 fn generate_board_coord(alpha_coord: char, num_coord: char) -> Coordinate {
    let mut coordinate = Coordinate::new(-1, -1);

    // Set the row (x) for the specified coordinate
    if num_coord == '1' { coordinate.x = 0; }
    else if num_coord == '2' { coordinate.x = 1; }
    else if num_coord == '3' { coordinate.x = 2; }
    else if num_coord == '4' { coordinate.x = 3; }

    // Set the col (y) for the specified coordinate
    if alpha_coord.to_ascii_uppercase() == 'A' { coordinate.y = 0; }
    else if alpha_coord.to_ascii_uppercase() == 'B' { coordinate.y = 1; }
    else if alpha_coord.to_ascii_uppercase() == 'C' { coordinate.y = 2; }
    else if alpha_coord.to_ascii_uppercase() == 'D' { coordinate.y = 3; }

    // Return the generated coordinate
    return coordinate;
 }



// Selects the card in the given coordinate on the board
fn select_board_coord(board: &mut [[Card; 4]; 4], coord: Coordinate) -> bool {
    let mut selected_coords: Vec<Coordinate> = Vec::new();

    // Immediently return false if the coordinate is outside the board
    if coord.x < 0 || coord.x > 3 || coord.y < 0 || coord.y > 3 { return false; }

    // Flip the currently selected card
    board[coord.x as usize][coord.y as usize].is_flipped = true;

    // Find the coordinate for each selected card on the board
    for i in 0..4 {
        for j in 0..4 {
            if board[i][j].is_flipped {
                selected_coords.push(Coordinate::new(i as i32, j as i32)); 
            }
        }
    }

    // If there are 2 selected cards, compare their symbols
    if  selected_coords.len() >= 2 {
        // Display the updated board and sleep for 2 seconds
        // before reflipping or deactivitating the selected cards
        display_board(board);
        std::thread::sleep(std::time::Duration::from_secs(2));

        // If symbols match on the two cards, deactivate them
        if board[selected_coords[0].x][selected_coords[0].y].symbol == board[selected_coords[1].x][selected_coords[1].y].symbol {
            board[selected_coords[0].x][selected_coords[0].y].is_active = false;
            board[selected_coords[1].x][selected_coords[1].y].is_active = false;
        }

        // Unflip the selected cards
        board[selected_coords[0].x][selected_coords[0].y].is_flipped = false;
        board[selected_coords[1].x][selected_coords[1].y].is_flipped = false;
        selected_coords.clear();
    }

    // The card was selected successfully
    return true;
}



// Determines if the game is over or not
fn check_if_game_over(board: &mut [[Card; 4]; 4]) -> bool {
    let mut is_game_over = true;

    // If any card is still active, the game is not over
    for i in 0..4 {
        for j in 0..4 {
            if board[i][j].is_active { is_game_over = false; }
        }
    }

    return is_game_over;
}
