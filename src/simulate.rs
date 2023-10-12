use crate::can_end::check;
use crate::ended::status;
use crate::moves::win;

use std::collections::HashMap;

pub fn simulate(
    mat: [[i32; 3]; 3],
    player: i32,
    opponent: i32,
    winning_moves: &mut HashMap<String, MovesChoice>,
) -> [f64; 2] {
    let key = to_key(mat, player, 0);
    if winning_moves.contains_key(&key) {
        return [winning_moves[&key].results, winning_moves[&key].moves];
    }
    let stat = check(mat, player, opponent);
    match stat {
        0 => {
            let tomove = win(mat, player);
            add_move(mat, tomove, player, winning_moves, 1.0, 1.0);
        }
        1 => {
            iterate_mat(mat, player, opponent, winning_moves, key.clone());
            return [winning_moves[&key].results, winning_moves[&key].moves];
        }
        2 => {
            let ended = status(mat, player, opponent);
            match ended {
                0 => return [1.0, 1.0],
                1 => return [-1.0, 1.0],
                2 => return [0.0, 1.0],
                -1 => {
                    iterate_mat(mat, player, opponent, winning_moves, key.clone());
                    return [winning_moves[&key].results, winning_moves[&key].moves];
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
    [winning_moves[&key].results, winning_moves[&key].moves]
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
#[derive(Clone, Copy)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}
pub struct MovesChoice {
    pub action: Coord,
    pub results: f64,
    pub moves: f64,
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({};{})", self.x, self.y)
    }
}

impl std::fmt::Display for MovesChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.action, self.results / self.moves)
    }
}

fn iterate_mat(
    mat: [[i32; 3]; 3],
    player: i32,
    opponent: i32,
    winning_moves: &mut HashMap<String, MovesChoice>,
    key: String,
) -> [f64; 2] {
    let mut results = 0.0;
    let mut moves = 0.0;
    let mut winning: Coord = Coord { x: 3, y: 3 };
    let mut highest_weight = -1.0;
    for i in 0..3 {
        for j in 0..3 {
            if mat[i][j] == 0 {
                let mut n_mat = mat;
                n_mat[i][j] = player;
                let [n_r, n_m] = simulate(n_mat, opponent, player, winning_moves);
                let n_h_w = (-n_r) / n_m;
                results -= n_r;
                moves += n_m;
                if n_h_w > highest_weight {
                    highest_weight = n_h_w;
                    winning = Coord { x: j, y: i };
                }
            }
        }
    }
    add_move(mat, winning, player, winning_moves, results, moves);
    [winning_moves[&key].results, winning_moves[&key].moves]
}

fn add_move(
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
