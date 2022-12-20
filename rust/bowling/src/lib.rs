#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(PartialEq, Eq)]
pub enum FrameKind {
    Open,
    Spare,
    Strike,
}

impl Default for FrameKind {
    fn default() -> Self {
        Self::Open
    }
}

#[derive(Default)]
pub struct Frame {
    kind: FrameKind,
    first: u16,
    second: u16,
    done: bool,
}

#[derive(Default)]
pub struct BowlingGame {
    frames: Vec<Frame>,
}

const TENTH_FRAME: usize = 9;
const MAX_PINS: u16 = 10;
const FRAMES_COUNT: usize = 10;

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    fn is_game_complete(&self) -> bool {
        let last_frame = self.frames.last();
        let tenth_frame = self.frames.get(TENTH_FRAME);

        match self.frames.len() {
            10 => last_frame.unwrap().kind == FrameKind::Open && last_frame.unwrap().done,
            11 => {
                (tenth_frame.unwrap().kind == FrameKind::Strike
                    && last_frame.unwrap().kind != FrameKind::Strike
                    && last_frame.unwrap().done)
                    || tenth_frame.unwrap().kind == FrameKind::Spare
            }
            12 => {
                tenth_frame.unwrap().kind == FrameKind::Strike
                    && self.frames.get(TENTH_FRAME + 1).unwrap().kind != FrameKind::Strike
            }
            _ => false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.is_game_complete() {
            return Err(Error::GameComplete);
        }

        if self.frames.is_empty() || self.frames.last().unwrap().done {
            self.frames.push(match pins == MAX_PINS {
                true => Frame {
                    kind: FrameKind::Strike,
                    first: MAX_PINS,
                    second: 0,
                    done: true,
                },
                false => Frame {
                    first: pins,
                    ..Default::default()
                },
            });
        } else {
            let last_frame = self.frames.last_mut().unwrap();

            if !last_frame.done {
                let total_pins = last_frame.first + pins;

                if total_pins > MAX_PINS {
                    return Err(Error::NotEnoughPinsLeft);
                }

                last_frame.second = pins;
                last_frame.done = true;

                if total_pins == MAX_PINS {
                    last_frame.kind = FrameKind::Spare;
                }
            }
        }

        Ok(())
    }

    fn is_score_ready(&self) -> bool {
        let frames_count = self.frames.len();

        if self.frames.is_empty() || frames_count < FRAMES_COUNT {
            return false;
        }

        if frames_count < FRAMES_COUNT + 1
            && (self.frames.get(TENTH_FRAME).unwrap().kind == FrameKind::Strike
                || self.frames.get(TENTH_FRAME).unwrap().kind == FrameKind::Spare)
        {
            return false;
        }

        if self.frames.get(TENTH_FRAME).unwrap().kind == FrameKind::Strike
            && self.frames.get(TENTH_FRAME + 1).unwrap().kind == FrameKind::Strike
            && frames_count < FRAMES_COUNT + 2
        {
            return false;
        }

        true
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_score_ready() {
            return None;
        }

        let mut score: u16 = 0;

        for (idx, frame) in self.frames.iter().enumerate() {
            score += frame.first + frame.second;
            let next = self.frames.get(idx + 1);

            if frame.kind == FrameKind::Spare {
                score += next.unwrap().first;
            } else if frame.kind == FrameKind::Strike {
                let next = next.unwrap();
                score += next.first;
                if next.kind == FrameKind::Strike {
                    let subsequent = self.frames.get(idx + 2).unwrap();
                    score += subsequent.first;
                } else {
                    score += next.second;
                }
            }

            if idx == 9 {
                break;
            }
        }

        Some(score)
    }
}
