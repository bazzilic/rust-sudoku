use std::io::Write;

#[derive(Debug)]
struct Cell {
    value: Option<i8>,
    corner_marks: [i8; 9],
    centre_marks: [i8; 9],
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            value: None,
            corner_marks: [0; 9],
            centre_marks: [0; 9],
        }
    }
}

type Board = [[Cell; 9]; 9];

fn main() {
    let mut board: Board = Default::default();
    
    let bdstr = match input_board() {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("{:?}", bdstr);

    read_board(&bdstr, &mut board);

    println!("{:?}", board);
}

fn input_board() -> Result<String, String> {
    for i in 0..3 {
        match input_board_attempt() {
            Ok(b) => {
                return Ok(b);
            }
            Err(e) => {
                println!("{}", e);
                if i == 2 {
                    break;
                }
                println!("Try again!");
            }
        }
    }
    return Err(String::from("Exiting after 3 failed attempts"));
}

fn input_board_attempt() -> Result<String, String> {
    let mut res = String::new();
    println!("Enter the board, one line at a time (e.g., 12--56-89<Enter>)");
    println!("Empty line == '---------'");
    println!("   |         |");
    for i in 0..9 {
        let mut line = String::new();
        
        print!("{}: |", i+1);
        std::io::stdout().flush();

        let err = std::io::stdin().read_line(&mut line); // including '\n'
        if err.is_err() {
            return Err(format!("Error reading line: {}", err.unwrap_err()));
        }
        
        let trimmed_line = line.trim();
        if trimmed_line.len() > 9 {
            return Err(format!("Invalid line length: {}", trimmed_line.len()));
        }

        let padded_line = format!("{:-<9}", trimmed_line).as_str();

        for c in padded_line.chars() {
            if (c != '-' && !c.is_numeric()) || c == '0' {
                return Err(format!("Invalid character, must be [123456789-]: {}", c));
            }
        }

        res.push_str(padded_line);
    }
    return Ok(res);
}

fn read_board(lines: &str, board: &mut Board) {
    if lines.len() != 81 {
        panic!("Invalid board length: {}", lines.len());
    }

    let (mut x, mut y) = (0, 0);

    for c in lines.chars() {
        if x == 9 {
            x = 0;
            y += 1;
        }
        if c == '-' {
            board[y][x].value = None;
        } else {
            board[y][x].value = Some(c.to_digit(10).unwrap() as i8);
        }
        x += 1;
    }
}
