use std::collections::HashMap;

use crate::structs::{Coord, MovesChoice};

pub fn add_move(
    mat: [[i32; 3]; 3],
    mut action: Coord,
    player: i32,
    winning_moves: &mut HashMap<String, MovesChoice>,
    results: f64,
    moves: f64,
) {
    for i in 0..4 {
        let key = to_key(mat, player, i);
        let _ = winning_moves.insert(
            key,
            MovesChoice {
                action,
                results,
                moves,
            },
        );
        action = rotate_coord(action);
    }
}

fn rotate_coord(action: Coord) -> Coord {
    let y = action.y;
    let x = action.x;
    Coord { y: x, x: 2 - y }
}

pub fn to_key(mat: [[i32; 3]; 3], player: i32, dir: i32) -> String {
    let mut s = "".to_string();
    match dir {
        0 => {
            for i in mat {
                for j in i {
                    s += if j == 0 {
                        "0"
                    } else if j == player {
                        "p"
                    } else {
                        "o"
                    };
                }
            }
        }
        1 => {
            for i in 0..3 {
                for j in (0..3).rev() {
                    s += if mat[j][i] == 0 {
                        "0"
                    } else if mat[j][i] == player {
                        "p"
                    } else {
                        "o"
                    };
                }
            }
        }
        2 => {
            for i in (0..3).rev() {
                for j in (0..3).rev() {
                    s += if mat[i][j] == 0 {
                        "0"
                    } else if mat[i][j] == player {
                        "p"
                    } else {
                        "o"
                    };
                }
            }
        }
        3 => {
            for i in (0..3).rev() {
                for j in mat {
                    s += if j[i] == 0 {
                        "0"
                    } else if j[i] == player {
                        "p"
                    } else {
                        "o"
                    };
                }
            }
        }
        _ => unreachable!(),
    }
    s
}
