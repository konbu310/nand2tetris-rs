#![allow(dead_code, non_snake_case)]

use crate::logic::bit::{self, I, O};
use ClockState::{Tick, Tock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockState {
    Tick, // 0
    Tock, // 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock {
    state: ClockState,
}

impl Clock {
    pub fn state(&self) -> ClockState {
        self.state
    }

    pub fn next(&mut self) {
        self.state = match self.state {
            Tick => Tock,
            Tock => Tick,
        }
    }

    pub fn new() -> Self {
        Clock { state: Tick }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DFF {
    state_past: bit,
    state_new: bit,
}

impl DFF {
    pub fn new() -> Self {
        DFF {
            state_past: O,
            state_new: O,
        }
    }

    pub fn input(&mut self, a: bit, clock: &Clock) {
        if clock.state() == Tick {
            self.state_past = self.state_new;
            self.state_new = a
        }
    }

    pub fn outpu(self, clock: &Clock) -> bit {
        match clock.state() {
            Tick => self.state_past,
            Tock => self.state_new,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bit {
    dff: DFF,
}
