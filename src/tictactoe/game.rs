const BOARD_W: usize = 3;
const BOARD_H: usize = 3;
const BOARD_SZ: usize = BOARD_H * BOARD_W;

#[derive(Clone, Copy)]
pub enum Tile {
    None = 0,
    X = 1,
    O = 2,
}

pub struct Board {
    tiles: [Tile; BOARD_SZ],
}

pub struct Game {
    board: Board,
    player: Tile,
}

impl Board {
    pub fn new() -> Board {
        Board {
            tiles: [Tile::None; 9],
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            player: Tile::X,
        }
    }
}
