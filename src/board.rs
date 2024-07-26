/// https://github.com/denkspuren/BitboardC4/blob/master/BitboardDesign.md
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board {
    board: [u64; 2],
    height: [usize; 7],
    count: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Side {
    #[default]
    A,
    B,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            a: 0,
            b: 0,
            height: [0, 7, 15, 24, 30, 35, 42],
            side_to_move: Side::default(),
        }
    }
}

impl Board {
    pub fn make_move(&mut self, col: usize) {
    }
}
