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
    let mut forced = false;
    match ended {
        0 => ([1.0, 1.0], false),
        1 => ([-1.0, 1.0], false),
        2 => ([0.0, 1.0], false),
        -1 => {
            let key = to_key(mat, player, 0);
            let status = check(mat, player, opponent);
            match status {
                0 => {
                    let winning = win(mat, player);
                    for winn in winning {
                        add_move(mat, winn, player, winning_moves, 1.0, 1.0);
                    }
                    forced = true;
                }
                1 => {
                    let not_losing = win(mat, opponent);
                    let mut not_losing_move_weight = -2.0;
                    for not_l in not_losing {
                        let mut n_mat = mat;
                        n_mat[not_l.y][not_l.x] = player;
                        let ([mut results, moves], f) =
                            simulate(n_mat, opponent, player, winning_moves);
                        results = -results;
                        if f && results == 1.0 {
                            add_move(mat, not_l, player, winning_moves, results, moves);
                            forced = true;
                            break;
                        } else if -results / moves > not_losing_move_weight {
                            not_losing_move_weight = results / moves;
                            add_move(mat, not_l, player, winning_moves, results, moves);
                        }
                    }
                }
                2 => {
                    let mut best_move = Coord { y: 3, x: 3 };
                    let mut winning_weight = -2.0;
                    let mut results = 0.0;
                    let mut moves = 0.0;
                    for i in 0..3 {
                        for j in 0..3 {
                            if mat[i][j] == 0 {
                                let mut n_mat = mat;
                                n_mat[i][j] = player;
                                let ([mut res, ms], can_force) =
                                    simulate(n_mat, opponent, player, winning_moves);
                                res = if res == 0.0 { res } else { -res };
                                if can_force && (res == -1.0 || res == 1.0) {
                                    return ([-res, 1.0], true);
                                }
                                results += res;
                                moves += ms;
                                if res / ms > winning_weight {
                                    best_move = Coord { y: i, x: j };
                                    winning_weight = res / ms;
                                }
                            }
                        }
                    }
                    add_move(mat, best_move, player, winning_moves, results, moves);
                }
                _ => unreachable!(),
            }
            (choice_to_result(winning_moves[&key]), forced)
        }
        _ => unreachable!(),
    }
}

fn choice_to_result(mc: MovesChoice) -> [f64; 2] {
    [mc.results, mc.moves]
}
