#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

const MIN_RANGE: i32 = 0;
const MAX_RANGE: i32 = 7;

impl ChessPosition {
    fn in_range(n: i32) -> bool {
        (MIN_RANGE..=MAX_RANGE).contains(&n)
    }

    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(Self::in_range(rank) && Self::in_range(file)) {
            return None;
        }

        Some(Self { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.rank == other.position.rank
            || self.position.file == other.position.file
            || ((self.position.rank - other.position.rank).abs()
                == (self.position.file - other.position.file).abs())
    }
}
