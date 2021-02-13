use crate::logic::Bit::{O, I};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use std::convert::{From, Into};
use num_traits::{PrimInt, FromPrimitive};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Bit {
    O,
    I,
}

impl Display for Bit {
    fn fmt(&self, dest: &mut Formatter) -> fmt::Result {
        let buf = match self {
            I => "I".to_string(),
            O => "O".to_string()
        };
        write!(dest, "{}", buf)
    }
}

impl<T> From<T> for Bit
    where T: PrimInt + FromPrimitive
{
    fn from(value: T) -> Self {
        if value == T::from_i32(0).unwrap() {
            O
        } else if value == T::from_i32(1).unwrap() {
            I
        } else {
            panic!("Bit needs 0 or 1.")
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Word([Bit; 16]);

impl Word {
    pub fn new(a: [Bit; 16]) -> Self {
        Word(a)
    }

    pub fn to_slice(&self) -> [Bit; 16] {
        [
            self[0],
            self[1],
            self[2],
            self[3],
            self[4],
            self[5],
            self[6],
            self[7],
            self[8],
            self[9],
            self[10],
            self[11],
            self[12],
            self[13],
            self[14],
            self[15]
        ]
    }
}

impl Display for Word {
    fn fmt(&self, dest: &mut Formatter) -> fmt::Result {
        let mut buf = "[".to_string();
        for i in 0..16 {
            buf = format!("{} {},", buf, self[i]);
        }
        buf = format!("{} ]", buf);
        write!(dest, "{}", buf)
    }
}

impl Index<usize> for Word {
    type Output = Bit;
    fn index(&self, index: usize) -> &Self::Output {
        if index > 15 {
            panic!(format!("index fail: {} is out of range.", index));
        }
        &self.0[index]
    }
}

impl IndexMut<usize> for Word {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > 15 {
            panic!(format!("index_mut fail: {}, out of range.", index));
        }
        self.0.index_mut(index)
    }
}

impl From<String> for Word {
    fn from(mut s: String) -> Self {
        s = s.split_terminator(' ').collect();
        let mut instruction = Word::new([O; 16]);
        let mut i = 0usize;
        for bytes in s.bytes() {
            instruction[i] = match bytes {
                48 => O,
                49 => I,
                _ => panic!("`WOrd::from_string` fail: cannot find 0 or 1.")
            };
            i = if i == 16 {
                panic!("`Word::from_string` fail: need less than 16.")
            } else {
                i + 1
            }
        }
        if i == 16 {
            instruction
        } else {
            panic!("`Word::from_string` fail: need more than 15")
        }
    }
}

impl From<&str> for Word {
    fn from(s: &str) -> Self {
        let s: String = s.split_terminator(' ').collect();
        let mut instruction = Word::new([O; 16]);
        let mut i = 0usize;
        for bytes in s.bytes() {
            instruction[i] = match bytes {
                48 => O,
                49 => I,
                _ => panic!("`Word::from_string` fail: cannot find 0 or 1.")
            };
            i = if i == 16 {
                panic!("`Word::from_string` fail: need less than 16.")
            } else {
                i + 1
            }
        }
        if i == 16 {
            instruction
        } else {
            panic!("`Word::from_string` fail: need more than 15")
        }
    }
}

pub fn nand(a: Bit, b: Bit) -> Bit {
    match a {
        O => match b {
            O => I,
            I => I
        },
        I => match b {
            O => I,
            I => O
        }
    }
}

pub fn not(a: Bit) -> Bit {
    nand(a, a)
}

pub fn and(a: Bit, b: Bit) -> Bit {
    nand(nand(a, b), nand(a, b))
}

pub fn or(a: Bit, b: Bit) -> Bit {
    nand(nand(a, a), nand(b, b))
}

pub fn xor(a: Bit, b: Bit) -> Bit {
    or(and(a, not(b)), and(b, not(a)))
}

pub fn mux(a: Bit, b: Bit, sel: Bit) -> Bit {
    or(and(a, not(sel)), and(b, sel))
}

pub fn dmux(inc: Bit, sel: Bit) -> (Bit, Bit) {
    (
        and(inc, not(sel)),
        and(inc, sel)
    )
}

pub fn not16(a: Word) -> Word {
    Word::new([
        not(a[0]),
        not(a[1]),
        not(a[2]),
        not(a[3]),
        not(a[4]),
        not(a[5]),
        not(a[6]),
        not(a[7]),
        not(a[8]),
        not(a[9]),
        not(a[10]),
        not(a[11]),
        not(a[12]),
        not(a[13]),
        not(a[14]),
        not(a[15]),
    ])
}

pub fn and16(a: Word, b: Word) -> Word {
    Word::new([
        and(a[0], b[0]),
        and(a[1], b[1]),
        and(a[2], b[2]),
        and(a[3], b[3]),
        and(a[4], b[4]),
        and(a[5], b[5]),
        and(a[6], b[6]),
        and(a[7], b[7]),
        and(a[8], b[8]),
        and(a[9], b[9]),
        and(a[10], b[10]),
        and(a[11], b[11]),
        and(a[12], b[12]),
        and(a[13], b[13]),
        and(a[14], b[14]),
        and(a[15], b[15]),
    ])
}

pub fn or16(a: Word, b: Word) -> Word {
    Word::new([
        or(a[0], b[0]),
        or(a[1], b[1]),
        or(a[2], b[2]),
        or(a[3], b[3]),
        or(a[4], b[4]),
        or(a[5], b[5]),
        or(a[6], b[6]),
        or(a[7], b[7]),
        or(a[8], b[8]),
        or(a[9], b[9]),
        or(a[10], b[10]),
        or(a[11], b[11]),
        or(a[12], b[12]),
        or(a[13], b[13]),
        or(a[14], b[14]),
        or(a[15], b[15]),
    ])
}

mod tests {
    use super::{nand, not, and};
    use crate::logic::Bit::{O, I};
    use crate::logic::{Word};
    use crate::logic::{or, xor, mux, dmux, not16};

    #[test]
    fn for_nand() {
        assert_eq!(nand(O, O), I);
        assert_eq!(nand(O, I), I);
        assert_eq!(nand(I, O), I);
        assert_eq!(nand(I, I), O)
    }

    #[test]
    fn for_note() {
        assert_eq!(not(O), I);
        assert_eq!(not(I), O);
    }

    #[test]
    fn for_and() {
        assert_eq!(and(O, O), O);
        assert_eq!(and(O, I), O);
        assert_eq!(and(I, O), O);
        assert_eq!(and(I, I), I);
    }

    #[test]
    fn for_or() {
        assert_eq!(or(O, O), O);
        assert_eq!(or(O, I), I);
        assert_eq!(or(I, O), I);
        assert_eq!(or(I, I), I);
    }

    #[test]
    fn for_xor() {
        assert_eq!(xor(O, O), O);
        assert_eq!(xor(O, I), I);
        assert_eq!(xor(I, O), I);
        assert_eq!(xor(I, I), O);
    }

    #[test]
    fn for_mux() {
        assert_eq!(mux(O, O, O), O);
        assert_eq!(mux(O, I, O), O);
        assert_eq!(mux(I, O, O), I);
        assert_eq!(mux(I, I, O), I);
        assert_eq!(mux(O, O, I), O);
        assert_eq!(mux(O, I, I), I);
        assert_eq!(mux(I, O, I), O);
        assert_eq!(mux(I, I, I), I);
    }

    #[test]
    fn for_dmux() {
        assert_eq!(dmux(O, O), (O, O));
        assert_eq!(dmux(O, I), (O, O));
        assert_eq!(dmux(I, O), (I, O));
        assert_eq!(dmux(I, I), (O, I));
    }

    #[test]
    fn for_not16() {
        assert_eq!(
            not16(Word::new([O, O, O, O, O, O, O, O, I, I, I, I, I, I, I, I])),
            Word::new([I, I, I, I, I, I, I, I, O, O, O, O, O, O, O, O])
        )
    }
}