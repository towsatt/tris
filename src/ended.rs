//0 => player won
//1 => opponent won
//2 => draw
//-1 => continuing
pub fn status(mat: [[i32; 3]; 3], player: i32, opponent: i32) -> i32 {
    if win(mat, player) {
        return 0;
    }
    if win(mat, opponent) {
        return 1;
    }
    if full_grid(mat) {
        return 2;
    }
    -1
}

fn win(mat: [[i32; 3]; 3], player: i32) -> bool {
    winning_rows(mat, player) || winning_columns(mat, player) || winning_diagonals(mat, player)
}

fn winning_rows(mat: [[i32; 3]; 3], player: i32) -> bool {
    for i in mat {
        if i.iter().all(|x| *x == player) {
            return true;
        }
    }
    false
}

fn winning_columns(mat: [[i32; 3]; 3], player: i32) -> bool {
    for i in 0..3 {
        if (0..3).all(|x| mat[x][i] == player) {
            return true;
        }
    }
    false
}

fn winning_diagonals(mat: [[i32; 3]; 3], player: i32) -> bool {
    if (0..3).all(|x| mat[x][x] == player) {
        return true;
    }
    if (0..3).all(|x| mat[x][2 - x] == player) {
        return true;
    }
    false
}

fn full_grid(mat: [[i32; 3]; 3]) -> bool {
    for i in mat {
        for j in i {
            if j == 0 {
                return false;
            }
        }
    }
    true
}
