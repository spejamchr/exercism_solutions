use Error::*;
use Frame::*;
use State::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Clone, Copy)]
enum State {
    FirstRoll,
    SecondRoll(u16),
    SpareFillBall(u16),
    FirstStrikeFill,
    SecondStrikeFill(u16),
    Complete,
}

enum Frame {
    Open(u16, u16),
    Spare(u16),
    Strike,
    LastSpare(u16, u16),
    LastStrike(u16, u16),
}

pub struct BowlingGame {
    state: State,
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            state: FirstRoll,
            frames: Vec::new(),
        }
    }

    fn add_frame(&mut self, frame: Frame) -> Result<(), Error> {
        self.frames.push(frame);
        Ok(())
    }

    fn set_state(&mut self, state: State) -> Result<(), Error> {
        self.state = state;
        Ok(())
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match (self.state, pins, self.frames.len()) {
            (Complete, _, _) => Err(GameComplete),
            (_, (11..=u16::MAX), _) => Err(NotEnoughPinsLeft),
            (FirstRoll, 10, 9) => self.set_state(FirstStrikeFill),
            (FirstRoll, 10, _) => self.add_frame(Strike),
            (FirstRoll, _, _) => self.set_state(SecondRoll(pins)),
            (SecondRoll(a), b, _) if a + b > 10 => Err(NotEnoughPinsLeft),
            (SecondRoll(a), b, 9) if a + b == 10 => self.set_state(SpareFillBall(a)),
            (SecondRoll(a), b, _) if a + b == 10 => self
                .add_frame(Spare(a))
                .and_then(|_| self.set_state(FirstRoll)),
            (SecondRoll(a), b, 9) => self
                .add_frame(Open(a, b))
                .and_then(|_| self.set_state(Complete)),
            (SecondRoll(a), b, _) => self
                .add_frame(Open(a, b))
                .and_then(|_| self.set_state(FirstRoll)),
            (SpareFillBall(a), b, _) => self
                .add_frame(LastSpare(a, b))
                .and_then(|_| self.set_state(Complete)),
            (FirstStrikeFill, a, _) => self.set_state(SecondStrikeFill(a)),
            (SecondStrikeFill(a), b, _) if a < 10 && a + b > 10 => Err(NotEnoughPinsLeft),
            (SecondStrikeFill(a), b, _) => self
                .add_frame(LastStrike(a, b))
                .and_then(|_| self.set_state(Complete)),
        }
    }

    fn first_at(&self, i: usize) -> u16 {
        match self.frames[i] {
            Open(a, _) | Spare(a) | LastSpare(a, _) => a,
            Strike | LastStrike(_, _) => 10,
        }
    }

    fn second_at(&self, i: usize) -> u16 {
        match self.frames[i] {
            Open(_, b) => b,
            Spare(a) | LastSpare(a, _) => 10 - a,
            Strike => self.first_at(i + 1),
            LastStrike(a, _) => a,
        }
    }

    fn frame_score(&self, frame: &Frame, i: usize) -> u16 {
        match frame {
            Open(a, b) => a + b,
            Spare(_) => 10 + self.first_at(i + 1),
            Strike => 10 + self.first_at(i + 1) + self.second_at(i + 1),
            LastSpare(_, b) => 10 + b,
            LastStrike(a, b) => 10 + a + b,
        }
    }

    pub fn score(&self) -> Option<u16> {
        match self.state {
            Complete => Some(
                self.frames
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (i, f)| acc + self.frame_score(f, i)),
            ),
            _ => None,
        }
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}
