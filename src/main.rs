use colored::*;
use reader::scan;

fn main() {
    println!("Début",);

    /*
    let mut echequier: [&str; 64] = [
        "♜", "♞", "♝", "♛", "♚", "♝", "♞", "♜", "♟", "♟", "♟", "♟", "♟", "♟", "♟", "♟", " ", " ",
        " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ",
        " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "♙", "♙", "♙", "♙", "♙", "♙",
        "♙", "♙", "♖", "♘", "♗", "♕", "♔", "♗", "♘", "♖",
    ];*/

    let piece_set_white: [&str; 6] = ["♜", "♞", "♝", "♛", "♚", "♟"];
    let piece_set_black: [&str; 6] = ["♖", "♘", "♗", "♕", "♔", "♙"];

    /* Piece set in letters if emojis not supported
    let piece_set_white: [&str; 6] = ["T", "C", "F", "R", "K", "P"];
    let piece_set_black: [&str; 6] = ["T", "C", "F", "R", "K", "P"];
    */

    let piece_set_blank: [&str; 6] = [" ", " ", " ", " ", " ", " "];

    let pieces: [[&str; 6]; 3] = [
        piece_set_white.to_owned(),
        piece_set_black.to_owned(),
        piece_set_blank.to_owned(),
    ];

    let mut echequier: [&str; 64] = generate_board(pieces);
    /*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-GAME LOOP*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/
    let mut sup: String = "".to_owned();
    let up_msg: String = "NoeChec".to_owned(); //not used here, but make us able to display a msg at the top of the screen
    loop {
        clearscreen::clear().expect("failed to clear screen");
        println!("{}", up_msg);
        println!("-----------------------");
        afficher_echequier(&echequier, pieces);
        let mut input: String = scan("Next Move ?", sup.as_str()).trim().to_string();
        if input == "exit" {
            std::process::exit(0);
        } else {
            /*
                xx?   => donne les déplacements possible
                xx>yy => déplace la pièce de XX vers yy
            */
            let numbers: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

            let char_arg_1: &char = &get_char_from_scan(0, &input);
            let char_arg_2: &char = &get_char_from_scan(1, &input);
            let char_arg_3: &char = &get_char_from_scan(2, &input);

            let char_arg_4: &char = &get_char_from_scan(3, &input);
            let char_arg_5: &char = &get_char_from_scan(4, &input);

            if char_arg_3 == &'>' {
                //make_move

                //errors :
                if input.len() == 5
                    && numbers.contains(char_arg_1)
                    && !input.chars().nth(0).is_none()
                    && !input.chars().nth(1).is_none()
                    && !input.chars().nth(3).is_none()
                    && !input.chars().nth(4).is_none()
                    && numbers.contains(char_arg_1)
                    && numbers.contains(char_arg_2)
                    && numbers.contains(char_arg_4)
                    && numbers.contains(char_arg_5)
                {
                    input.push_str(" was the last move\n");
                    sup = input;
                    let i81: i8 = char_arg_1.to_owned() as i8 - 0x30;
                    let i82: i8 = char_arg_2.to_owned() as i8 - 0x30;
                    let i84: i8 = char_arg_4.to_owned() as i8 - 0x30;
                    let i85: i8 = char_arg_5.to_owned() as i8 - 0x30;

                    echequier = make_move(get_cell(i81, i82), get_cell(i84, i85), echequier);
                } else {
                    sup = "ERROR ! incorrect syntax \n".to_owned();
                }
            }
        }

        print!("-----------------------");
    }
}

fn generate_board(pieces: [[&str; 6]; 3]) -> [&str; 64] {
    let mut echequier: [&str; 64] = [""; 64];
    let mut echequier_index: usize = 0;
    for i in 0..5 {
        echequier[echequier_index] = pieces[0][i];
        echequier_index += 1;
    }
    for i in 0..3 {
        println!("{}", i);
        echequier[echequier_index] = pieces[0][2 - i];
        echequier_index += 1;
    }
    for _i in 1..=8 {
        echequier[echequier_index] = pieces[0][5];
        echequier_index += 1;
    }
    //////////////////////////////////////
    for _i in 1..=32 {
        echequier[echequier_index] = pieces[2][0];
        echequier_index += 1;
    }
    //////////////////////////////////////
    for _i in 1..=8 {
        echequier[echequier_index] = pieces[1][5];
        echequier_index += 1;
    }
    for i in 0..5 {
        echequier[echequier_index] = pieces[1][i];
        echequier_index += 1;
    }
    for i in 0..3 {
        println!("{}", i);
        echequier[echequier_index] = pieces[1][2 - i];
        echequier_index += 1;
    }
    return echequier;
}

/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-Display Board*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/
fn afficher_echequier(board: &[&str; 64], pieces: [[&str; 6]; 3]) {
    print!("\t",);
    for i in 0..=7 {
        //quadrillage
        print!("{} ", i);
    }
    print!("\n");
    ////////////////
    let mut compteur: i8 = 0;
    print!("\n{}\t", 0);
    for x in board {
        compteur += 1;

        if pieces[0].contains(x) {
            print!("{} ", x.green());
        } else if pieces[1].contains(x) {
            print!("{} ", x.yellow());
        } else {
            print!("{} ", x.white());
        }

        if compteur % 8 == 0 && compteur <= 64 - 8 {
            print!("\n{}\t", (compteur / 8));
        }
    }
    print!("\n");
}

/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-SCAN INPUT*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/
mod reader {
    use std::io;
    pub fn scan(msg: &str, sup: &str) -> String {
        println!("{}{}", sup, msg);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_ok) => {}
            Err(_err) => {}
        }
        input.trim().to_string()
    }
}

/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-Get Cell*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

fn get_cell(mut x: i8, mut y: i8) -> usize {
    //get cell of the board from the 2 i8 that represent the cell when displayed
    x = 8 * (x);
    y += 0;

    return (x + y) as usize;
}

/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-CMD Execute*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

fn make_move(cell1: usize, cell2: usize, board: [&str; 64]) -> [&str; 64] {
    //switch two cell from the board
    let mut temp_board: [&str; 64] = board;
    let temp_container: &str = temp_board[cell1];

    temp_board[cell1] = temp_board[cell2];
    temp_board[cell2] = temp_container;

    return temp_board;
}

/*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*/

fn get_char_from_scan(i: usize, str: &String) -> char {
    match str.chars().nth(i) {
        Some(x) => return x,
        None => return ' ',
    }
}
