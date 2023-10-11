use std::{collections::HashMap, io, time::Instant};

mod can_end;
mod ended;
mod moves;
mod simulate;

use ended::status;
use simulate::{simulate, to_key, MovesChoice};

fn main() {
    let mut winning_moves = HashMap::<String, MovesChoice>::new();
    let start = Instant::now();
    simulate([[0; 3]; 3], 1, 2, &mut winning_moves);
    let duration = start.elapsed();
    println!("Time elapsed in simulate() is: {:?}", duration);
    println!("insert your symbol (X or O)");
    let mut p1 = String::new();
    let mut err = false;
    while !(["X", "O"].iter().any(|x| *x == p1)) {
        if err {
            p1.clear();
            println!("wrong symbol");
        }
        io::stdin().read_line(&mut p1).unwrap();
        p1 = p1.trim().to_string().to_uppercase();
        if !err {
            err = !err
        };
    }
    let cpu = if p1 == "O" { "X" } else { "O" };
    let mut board = Board {
        mat: [[0; 3]; 3],
        player: 1,
        opponent: 2,
        player_symbol: cpu.to_string(),
        opponent_symbol: p1,
    };
    let mut turn: bool = rand::random();
    while status(board.mat, board.player, board.opponent) == -1 {
        if turn {
            let key = to_key(board.mat, board.player);
            let tomove = winning_moves[&key].action;
            board.mat[tomove.y][tomove.x] = board.player;
        } else {
            loop{
                let corr_range = |i: usize| {(0..3).contains(&i)};  
                let mut input = String::new();
                println!("insert coords (y, x):");
                io::stdin().read_line(&mut input).unwrap();
                //formato della stringa: "y:i32 x:i32"
                let mut iter = input.trim().split(' ');
                let y: usize = iter.next().unwrap().parse().unwrap();
                let x: usize = iter.next().unwrap().parse().unwrap();
                if corr_range(y) && corr_range(x) && board.mat[y][x] == 0 {
                    board.mat[y][x] = board.opponent;
                    break;
                }
            }
        }
        println!("{board}");
        turn = !turn;
    }
    match status(board.mat, board.player, board.opponent) {
        0 => println!("you lost"),
        1 => println!("you won"),
        2 => println!("draw"),
        _ => unreachable!(),
    }
}

struct Board {
    mat: [[i32; 3]; 3],
    player: i32,
    opponent: i32,
    player_symbol: String,
    opponent_symbol: String,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.mat {
            let helper = |i: usize| {
                if row[i] == self.player {
                    &self.player_symbol
                } else if row[i] == self.opponent {
                    &self.opponent_symbol
                } else {
                    " "
                }
            };
            writeln!(f, "{} | {} | {}", helper(0), helper(1), helper(2),)?;
        }
        Ok(())
    }
}
