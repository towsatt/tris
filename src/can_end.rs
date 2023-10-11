// 0 -> can win
// 1 -> can lose
// 2 -> none

pub fn check(mat: [[i32; 3]; 3], player: i32, opponent: i32) -> i32 {
    if check_rows(mat, player) {
        return 0;
    }
    if check_rows(mat, opponent) {
        return 1;
    }
    2
}

fn check_rows(mat: [[i32; 3]; 3], player: i32) -> bool {
    for i in mat {
        if row(i, player) {
            return true;
        }
    }
    check_columns(mat, player)
}

fn row(mut row: [i32; 3], player: i32) -> bool {
    row.sort();
    row[0] == 0 && row[1] == player && row[2] == player
}

fn check_columns(mat: [[i32; 3]; 3], player: i32) -> bool {
    for i in 0..3 {
        if column(mat, i, player) {
            return true;
        }
    }
    check_diagonals(mat, player)
}

fn column(mat: [[i32; 3]; 3], i: usize, player: i32) -> bool {
    let mut col: [i32; 3] = [mat[0][i], mat[1][i], mat[2][i]];
    col.sort();
    col[0] == 0 && col[1] == player && col[2] == player
}

fn check_diagonals(mat: [[i32; 3]; 3], player: i32) -> bool {
    let mut d1: [i32; 3] = [-1; 3];
    let mut d2: [i32; 3] = [-1; 3];
    for i in 0..3 {
        d1[i] = mat[i][i];
        d2[i] = mat[i][2 - i];
    }
    d1.sort();
    d2.sort();
    (d1[0] == 0 && d1[1] == player && d1[2] == player)
        || (d2[0] == 0 && d2[1] == player && d2[2] == player)
}
