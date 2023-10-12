use crate::simulate::Coord;
use itertools::Itertools;

pub fn win(mat: [[i32; 3]; 3], player: i32) -> Coord {
    winning_rows(mat, player)
}

fn winning_rows(mat: [[i32; 3]; 3], player: i32) -> Coord {
    for (ind, row) in mat.iter().enumerate() {
        for vc in ([0, 1, 2] as [usize; 3]).into_iter().combinations(2) {
            if row[vc[0]] == player && row[vc[1]] == player && row[3 - (vc[1] + vc[0])] == 0 {
                return Coord {
                    x: 3 - (vc[1] + vc[0]),
                    y: ind,
                };
            }
        }
    }
    winning_columns(mat, player)
}

fn winning_columns(mat: [[i32; 3]; 3], player: i32) -> Coord {
    for i in 0..3 {
        for vc in (0..3).combinations(2) {
            if mat[vc[0]][i] == player
                && mat[vc[1]][i] == player
                && mat[3 - (vc[0] + vc[1])][i] == 0
            {
                return Coord {
                    x: i,
                    y: 3 - (vc[0] + vc[1]),
                };
            }
        }
    }
    winning_diagonals(mat, player)
}

fn winning_diagonals(mat: [[i32; 3]; 3], player: i32) -> Coord {
    for vc in (0..3).combinations(2) {
        if mat[vc[0]][vc[0]] == player
            && mat[vc[1]][vc[1]] == player
            && mat[3 - (vc[0] + vc[1])][3 - (vc[0] + vc[1])] == 0
        {
            return Coord {
                x: 3 - (vc[0] + vc[1]),
                y: 3 - (vc[0] + vc[1]),
            };
        }

        if mat[vc[0]][2 - vc[0]] == player
            && mat[vc[1]][2 - vc[1]] == player
            && mat[3 - (vc[0] + vc[1])][(vc[0] + vc[1]) - 1] == 0
        {
            return Coord {
                x: (vc[0] + vc[1]) - 1,
                y: 3 - (vc[0] + vc[1]),
            };
        }
    }
    Coord { x: 3, y: 3 }
}
