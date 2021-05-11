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

    pub fn output(self, clock: &Clock) -> bit {
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

#[cfg(test)]
mod tests {
    use super::ClockState::{Tick, Tock};
    use super::*;
    use crate::logic::bit::{I, O};

    #[test]
    fn for_clock_new() {
        let mut clock = Clock::new();
        assert_eq!(clock.state(), Tick);
        clock.next();
        assert_eq!(clock.state(), Tock);
        clock.next();
        assert_eq!(clock.state(), Tick);
    }

    #[test]
    fn for_dff() {
        // init: past -> O, new -> O
        let mut dff = DFF::new();
        let mut clock = Clock::new();

        // past -> O, new -> I
        dff.input(I, &clock);
        // clock = Tick, output = past
        assert_eq!(dff.output(&clock), O);
        // clock -> Tock
        clock.next();

        // clock = Tock, nothing happen
        dff.input(O, &clock);
        // clock = Tock, output = new
        assert_eq!(dff.output(&clock), I);
        // clock -> Tick
        clock.next();

        // past -> I, new -> O
        dff.input(O, &clock);
        // clock = Tick, output = past
        assert_eq!(dff.output(&clock), I);
        // clock -> Tock
        clock.next();

        // clock = Tock, nothing happen
        dff.input(O, &clock);
        // clock = Tock, output = new
        assert_eq!(dff.output(&clock), O);
    }
}
