use crate::structs::Coord;
use itertools::Itertools;

pub fn win(mat: [[i32; 3]; 3], player: i32) -> Vec<Coord> {
    let mut wins = Vec::<Coord>::new();
    winning_rows(mat, player, &mut wins);
    wins
}

fn winning_rows(mat: [[i32; 3]; 3], player: i32, wins: &mut Vec<Coord>) {
    for (ind, row) in mat.iter().enumerate() {
        for vc in ([0, 1, 2] as [usize; 3]).into_iter().combinations(2) {
            if row[vc[0]] == player && row[vc[1]] == player && row[3 - (vc[1] + vc[0])] == 0 {
                wins.push(Coord {
                    x: 3 - (vc[1] + vc[0]),
                    y: ind,
                });
            }
        }
    }
    winning_columns(mat, player, wins);
}

fn winning_columns(mat: [[i32; 3]; 3], player: i32, wins: &mut Vec<Coord>) {
    for i in 0..3 {
        for vc in (0..3).combinations(2) {
            if mat[vc[0]][i] == player
                && mat[vc[1]][i] == player
                && mat[3 - (vc[0] + vc[1])][i] == 0
            {
                wins.push(Coord {
                    x: i,
                    y: 3 - (vc[0] + vc[1]),
                });
            }
        }
    }
    winning_diagonals(mat, player, wins);
}

fn winning_diagonals(mat: [[i32; 3]; 3], player: i32, wins: &mut Vec<Coord>) {
    for vc in (0..3).combinations(2) {
        if mat[vc[0]][vc[0]] == player
            && mat[vc[1]][vc[1]] == player
            && mat[3 - (vc[0] + vc[1])][3 - (vc[0] + vc[1])] == 0
        {
            wins.push(Coord {
                x: 3 - (vc[0] + vc[1]),
                y: 3 - (vc[0] + vc[1]),
            });
        }

        if mat[vc[0]][2 - vc[0]] == player
            && mat[vc[1]][2 - vc[1]] == player
            && mat[3 - (vc[0] + vc[1])][(vc[0] + vc[1]) - 1] == 0
        {
            wins.push(Coord {
                x: (vc[0] + vc[1]) - 1,
                y: 3 - (vc[0] + vc[1]),
            });
        }
    }
}
