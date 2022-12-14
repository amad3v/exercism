// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        use Direction::*;

        let d = match self.d {
            North => East,
            East => South,
            South => West,
            West => North,
        };

        Self { d, ..self }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        use Direction::*;

        let d = match self.d {
            North => West,
            East => North,
            South => East,
            West => South,
        };

        Self { d, ..self }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        use Direction::*;

        match self.d {
            North => Self {
                y: self.y + 1,
                ..self
            },
            East => Self {
                x: self.x + 1,
                ..self
            },
            South => Self {
                y: self.y - 1,
                ..self
            },
            West => Self {
                x: self.x - 1,
                ..self
            },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |a, i| match i {
            'A' => a.advance(),
            'L' => a.turn_left(),
            'R' => a.turn_right(),
            _ => a,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
