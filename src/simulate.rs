use crate::can_end::check;
use crate::ended::ended;
use crate::moves::win;
use crate::rotations::{add_move, to_key};
use crate::structs::Coord;
use crate::structs::MovesChoice;

use std::collections::HashMap;

pub fn simulate(
    mat: [[i32; 3]; 3],
    player: i32,
    opponent: i32,
    winning_moves: &mut HashMap<String, MovesChoice>,
) -> ([f64; 2], bool) {
    let ended = ended(mat, player, opponent);
    match ended {
        2 => ([0.0, 1.0], false),
        -1 => {
            let key = to_key(mat, player, 0);
            let status = check(mat, player, opponent);
            match status {
                0 => {
                    let wins = win(mat, player);
                    add_move(mat, wins[0], player, winning_moves, 1.0, 1.0);
                    ([1.0, 1.0], true)
                }
                1 => {
                    let mut max_w = -2.0;
                    let mut ff = false;
                    for l in win(mat, opponent) {
                        let mut n_mat = mat;
                        n_mat[l.y][l.x] = player;
                        let ([res, mvs], f) = simulate(n_mat, opponent, player, winning_moves);
                        if -res / mvs > max_w {
                            max_w = -res / mvs;
                            add_move(mat, l, player, winning_moves, -res, mvs);
                            ff = f;
                        }
                    }
                    for i in 0..3 {
                        for j in 0..3 {
                            if mat[i][j] == 0
                                && win(mat, opponent).iter().all(|x| x.y != i && x.x != j)
                            {
                                add_move(mat, Coord{y: i, x: j}, opponent, winning_moves, 1.0, 1.0);
                            }
                        }
                    }
                    (choice_to_result(winning_moves[&key]), ff)
                }
                2 => {
                    let mut max_w = -2.0;
                    let mut winning = Coord { x: 3, y: 3 };
                    let mut b_res = 0.0;
                    let mut b_mvs = 0.0;
                    let mut results = 0.0;
                    let mut moves = 0.0;
                    let mut ff = false;
                    for i in 0..3 {
                        for j in 0..3 {
                            if mat[i][j] == 0 {
                                let mut n_mat = mat;
                                n_mat[i][j] = player;
                                let ([mut res, mvs], f) =
                                    simulate(n_mat, opponent, player, winning_moves);
                                res = -res;
                                results += res;
                                moves += mvs;
                                if res / mvs > max_w {
                                    winning = Coord { x: j, y: i };
                                    max_w = res / mvs;
                                    b_res = res;
                                    b_mvs = mvs;
                                    ff = f;
                                }
                            }
                        }
                    }
                    add_move(mat, winning, player, winning_moves, b_res, b_mvs);
                    if ff {
                        ([b_res, b_mvs], true)
                    } else {
                        ([results, moves], false)
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn choice_to_result(mc: MovesChoice) -> [f64; 2] {
    [mc.results, mc.moves]
}
