const BOARD_W: usize = 3;
const BOARD_H: usize = 3;
const BOARD_SZ: usize = BOARD_H * BOARD_W;

#[derive(Clone, Copy)]
pub enum Tile {
    None = 0,
    X = 1,
    _O = 2,
}

pub struct Board {
    _tiles: [Tile; BOARD_SZ],
}

pub struct Game {
    _board: Board,
    _player: Tile,
}

impl Board {
    pub fn new() -> Board {
        Board {
            _tiles: [Tile::None; 9],
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            _board: Board::new(),
            _player: Tile::X,
        }
    }
}
