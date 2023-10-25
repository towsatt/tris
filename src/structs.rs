#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}
#[derive(Clone, Copy, Debug)]
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
