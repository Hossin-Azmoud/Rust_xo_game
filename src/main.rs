


use std::io;

fn get_i8number(n: &mut i8, prompt: &str) {
    let mut temp = String::new();
    println!("{}", prompt);
    io::stdin()
    .read_line(&mut temp)
    .expect("failed to read.");
    *n = temp
    .trim()
    .parse()
    .expect("Input not an integer");
}

fn getname(c: &mut String, prompt: String) {
    let mut temp = String::new();
    println!("{}", prompt);
    io::stdin()
    .read_line(&mut temp)
    .expect("failed to read.");
    *c = temp
    .trim().to_string();

}


fn check_board(board: [[&str; 3]; 3]) -> &str {
    /*
        0 1 2
        3 4 5
        6 7 8
    */
    
    let mut sig = "";

    for i in board {
        if i[0] == i[1] && i[1] == i[2] {
            if i[0] != "_" {
                sig = i[0];
            }
        }
    }

    return sig;
}

fn display_board(board: [[&str; 3]; 3]) {
    println!("");

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if j == board[i].len() - 1 {
                println!("{}", board[i][j]);
            } else {
                print!("{} | ", board[i][j]);
            }
        }
    }

    println!("");
}

fn run(_flag: &mut bool, board: &mut [[&str; 3]; 3], players: &[String; 2], idx: usize) -> i8 {
    

    let sig: &str = check_board(*board);
    let mut flag: i8 = 1;

    if sig.len() == 1  {
        println!("{}", SIG);
        flag = 2;
        return flag;
    }

    display_board(*board);
    
    let mut index: i8 = 0;
  
    let sym: [&str; 2] = ["O", "X"];
    println!("{}'s turn", players[idx]);
    
    get_i8number(&mut index, "Index: ");
    fn validation(n: i8) -> bool {
        return 0 <= n && n <= 9;
    }

    match validation(index) {
        true => {
            // Valid index
            let mut counter: i8 = 1;
            for i in 0..board.len() {
                for j in 0..board[i].len() {
                    
                    if counter == index {
                        match board[i][j] == sym[0] || board[i][j] == sym[1] {
                            true =>  {
                                flag = 0;
                            },
                            false => {
                                board[i][j] = sym[idx];
                                flag = 1;
                            }
                        };
                    }

                    counter += 1;
                }
            }
        },

        false => {
            // Invalid index
            println!("{} is an invalid index, should be 0 <= index < 9", index);
            flag = 0;
        }

    }

    return flag;
}

fn main() {
    // TODO implement points.
    let mut run_: bool = true;
    let sym: [&str; 2] = ["O", "X"];

    let mut board: [[&str; 3]; 3] = [
        ["_"; 3], 
        ["_"; 3], 
        ["_"; 3]
    ];
    
   
    let mut players: [String; 2] = [
        "".to_string(),
        "".to_string()
    ];
    
    for i in 0..2 {
        getname(&mut players[i], format!("{}:", sym[i]));
        println!("");
    }
    
    let mut i: usize = 0;

    while run_ {
        let flag = run(&mut run_, &mut board, &players, i);
        
        match flag {
            0 => {

            },
            1 => {
                match i {
                    0 => {
                        i = 1;
                    },
                    1 => {
                        i = 0;
                    },
            
                    _  => {
                        println!("invalid, and inexpected index!");
                    }
                }
            },
            
            2 => {
                run_ = false;
            },
            _ => {}
        }

        
    }

}
