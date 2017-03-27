#[allow(dead_code)]

use std::fmt::*;

enum Player {
    P1,
    P2,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Player::*;
        write!(f, "{}", match *self {
            P1 => "Player One",
            P2 => "Player Two",
        })
    }
}

#[derive(Copy, Clone, Debug)]
enum Cell {
    X,
    O,
    Empty,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use Cell::*;
        write!(f, "{}", match *self {
            X => "X",
            O => "O",
            Empty => " ",
        })
    }
}


fn print_board(board: &[[Cell; 3];3 ]) {
    for row in board  {
        for cell in row {
            print!("{} ", cell);
        }
        println!("");
    }
}

fn main() {
    use Cell::*;
    use Player::*;
    use std::io;

    let mut board = [[Empty; 3]; 3];

    let mut player = P1;

    loop {
        print_board(&board);
        player = match player {P1 => P2, P2 => P1,};

        let mut x;
        let mut y;

        let mut done = false;
        let mut choice;

        while !done {
            println!("{}: Choose a cell coordinate!", player);

            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .ok()
                .expect("readline: error");

            choice = input.split(",")
                .take(2)
                .map(|x| x.parse.ok())
                .filter(|&x| x >= 0 && x < 3)
                .collect::<Vec<usize>>();

            done = true;
        }
        break;
    }
}
