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

pub fn dmux(inc: Bit, sel: Bit) -> [Bit; 2] {
    [
        and(inc, not(sel)),
        and(inc, sel)
    ]
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

pub fn mux16(a: Word, b: Word, sel: Bit) -> Word {
    Word::new([
        mux(a[0], b[0], sel),
        mux(a[1], b[1], sel),
        mux(a[2], b[2], sel),
        mux(a[3], b[3], sel),
        mux(a[4], b[4], sel),
        mux(a[5], b[5], sel),
        mux(a[6], b[6], sel),
        mux(a[7], b[7], sel),
        mux(a[8], b[8], sel),
        mux(a[9], b[9], sel),
        mux(a[10], b[10], sel),
        mux(a[11], b[11], sel),
        mux(a[12], b[12], sel),
        mux(a[13], b[13], sel),
        mux(a[14], b[14], sel),
        mux(a[15], b[15], sel),
    ])
}

pub fn or8way(a: [Bit; 8]) -> Bit {
    or(
        or(
            or(a[0], a[1]),
            or(a[2], a[3]),
        ),
        or(
            or(a[4], a[5]),
            or(a[6], a[7]),
        ),
    )
}

pub fn mux4way16(a: Word, b: Word, c: Word, d: Word, sel: [Bit; 2]) -> Word {
    let mux2 = |a: Bit, b: Bit, c: Bit, d: Bit, s0: Bit, s1: Bit| -> Bit {
        mux(
            mux(a, b, s1),
            mux(c, d, s1),
            s0,
        )
    };
    Word::new([
        mux2(a[0], b[0], c[0], d[0], sel[0], sel[1]),
        mux2(a[1], b[1], c[1], d[1], sel[0], sel[1]),
        mux2(a[2], b[2], c[2], d[2], sel[0], sel[1]),
        mux2(a[3], b[3], c[3], d[3], sel[0], sel[1]),
        mux2(a[4], b[4], c[4], d[4], sel[0], sel[1]),
        mux2(a[5], b[5], c[5], d[5], sel[0], sel[1]),
        mux2(a[6], b[6], c[6], d[6], sel[0], sel[1]),
        mux2(a[7], b[7], c[7], d[7], sel[0], sel[1]),
        mux2(a[8], b[8], c[8], d[8], sel[0], sel[1]),
        mux2(a[9], b[9], c[9], d[9], sel[0], sel[1]),
        mux2(a[10], b[10], c[10], d[10], sel[0], sel[1]),
        mux2(a[11], b[11], c[11], d[11], sel[0], sel[1]),
        mux2(a[12], b[12], c[12], d[12], sel[0], sel[1]),
        mux2(a[13], b[13], c[13], d[13], sel[0], sel[1]),
        mux2(a[14], b[14], c[14], d[14], sel[0], sel[1]),
        mux2(a[15], b[15], c[15], d[15], sel[0], sel[1]),
    ])
}

pub fn mux8way16(a: Word, b: Word, c: Word, d: Word, e: Word, f: Word, g: Word, h: Word, s: [Bit; 3]) -> Word {
    let mux3 = |a: Bit, b: Bit, c: Bit, d: Bit, e: Bit, f: Bit, g: Bit, h: Bit, s0: Bit, s1: Bit, s2: Bit| -> Bit {
        mux(
            mux(
                mux(a, b, s2),
                mux(c, d, s2),
                s1,
            ),
            mux(
                mux(e, f, s2),
                mux(g, h, s2),
                s1,
            ),
            s0,
        )
    };

    Word::new([
        mux3(a[0], b[0], c[0], d[0], e[0], f[0], g[0], h[0], s[0], s[1], s[2]),
        mux3(a[1], b[1], c[1], d[1], e[1], f[1], g[1], h[1], s[0], s[1], s[2]),
        mux3(a[2], b[2], c[2], d[2], e[2], f[2], g[2], h[2], s[0], s[1], s[2]),
        mux3(a[3], b[3], c[3], d[3], e[3], f[3], g[3], h[3], s[0], s[1], s[2]),
        mux3(a[4], b[4], c[4], d[4], e[4], f[4], g[4], h[4], s[0], s[1], s[2]),
        mux3(a[5], b[5], c[5], d[5], e[5], f[5], g[5], h[5], s[0], s[1], s[2]),
        mux3(a[6], b[6], c[6], d[6], e[6], f[6], g[6], h[6], s[0], s[1], s[2]),
        mux3(a[7], b[7], c[7], d[7], e[7], f[7], g[7], h[7], s[0], s[1], s[2]),
        mux3(a[8], b[8], c[8], d[8], e[8], f[8], g[8], h[8], s[0], s[1], s[2]),
        mux3(a[9], b[9], c[9], d[9], e[9], f[9], g[9], h[9], s[0], s[1], s[2]),
        mux3(a[10], b[10], c[10], d[10], e[10], f[10], g[10], h[10], s[0], s[1], s[2]),
        mux3(a[11], b[11], c[11], d[11], e[11], f[11], g[11], h[11], s[0], s[1], s[2]),
        mux3(a[12], b[12], c[12], d[12], e[12], f[12], g[12], h[12], s[0], s[1], s[2]),
        mux3(a[13], b[13], c[13], d[13], e[13], f[13], g[13], h[13], s[0], s[1], s[2]),
        mux3(a[14], b[14], c[14], d[14], e[14], f[14], g[14], h[14], s[0], s[1], s[2]),
        mux3(a[15], b[15], c[15], d[15], e[15], f[15], g[15], h[15], s[0], s[1], s[2]),
    ])
}

pub fn dmux4way(inc: Bit, sel: [Bit; 2]) -> [Bit; 4] {
    [
        and(not(sel[0]), and(not(sel[1]), inc)),
        and(not(sel[0]), and(sel[1], inc)),
        and(sel[0], and(not(sel[1]), inc)),
        and(sel[0], and(sel[1], inc))
    ]
}

pub fn dmux8way(inc: Bit, sel: [Bit; 3]) -> [Bit; 8] {
    [
        and(and(not(sel[0]), not(sel[1])), and(not(sel[2]), inc)),
        and(and(not(sel[0]), not(sel[1])), and(sel[2], inc)),
        and(and(not(sel[0]), sel[1]), and(not(sel[2]), inc)),
        and(and(not(sel[0]), sel[1]), and(sel[2], inc)),
        and(and(sel[0], not(sel[1])), and(not(sel[2]), inc)),
        and(and(sel[0], not(sel[1])), and(sel[2], inc)),
        and(and(sel[0], sel[1]), and(not(sel[2]), inc)),
        and(and(sel[0], sel[1]), and(sel[2], inc)),
    ]
}

#[cfg(test)]
mod tests {
    use super::{nand, not, and};
    use crate::logic::Bit::{O, I};
    use crate::logic::{Word, and16, or16, mux16, or8way, mux4way16, mux8way16, dmux4way, dmux8way};
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
        assert_eq!(dmux(O, O), [O, O]);
        assert_eq!(dmux(O, I), [O, O]);
        assert_eq!(dmux(I, O), [I, O]);
        assert_eq!(dmux(I, I), [O, I]);
    }

    #[test]
    fn for_dislay() {
        assert_eq!(format!("{}", I), "I".to_string());
        assert_eq!(format!("{}", O), "O".to_string());
    }

    #[test]
    fn for_not16() {
        assert_eq!(
            not16(Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            not16(Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            not16(Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I])),
            Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O])
        );
        assert_eq!(
            not16(Word([O, O, O, O, O, O, O, O, I, I, I, I, I, I, I, I])),
            Word([I, I, I, I, I, I, I, I, O, O, O, O, O, O, O, O])
        );
    }

    #[test]
    fn for_and16() {
        assert_eq!(
            and16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            and16(
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            and16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            and16(
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
            ),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            and16(
                Word([O, O, I, I, O, I, I, O, I, O, O, I, I, O, I, I]),
                Word([O, I, O, I, I, O, I, O, I, I, I, I, O, I, O, I]),
            ),
            Word([O, O, O, I, O, O, I, O, I, O, O, I, O, O, O, I])
        );
    }

    #[test]
    fn for_or16() {
        assert_eq!(
            or16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            or16(
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
            ),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            or16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
            ),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            or16(
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
            ),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            or16(
                Word([O, O, I, I, O, I, I, O, I, O, O, I, I, O, I, I]),
                Word([O, I, O, I, I, O, I, O, I, I, I, I, O, I, O, I]),
            ),
            Word([O, I, I, I, I, I, I, O, I, I, I, I, I, I, I, I])
        );
    }

    #[test]
    fn for_mux16() {
        assert_eq!(
            mux16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                O,
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            mux16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                I,
            ),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            mux16(
                Word([O, I, O, I, O, I, O, I, I, O, I, O, I, O, I, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                O,
            ),
            Word([O, I, O, I, O, I, O, I, I, O, I, O, I, O, I, O])
        );
        assert_eq!(
            mux16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, I, O, I, O, I, O, I, I, O, I, O, I, O, I, O]),
                I,
            ),
            Word([O, I, O, I, O, I, O, I, I, O, I, O, I, O, I, O])
        );
    }

    #[test]
    fn for_or8way() {
        assert_eq!(or8way([O, O, O, O, O, O, O, O]), O);
        assert_eq!(or8way([I, O, O, O, O, O, O, O]), I);
        assert_eq!(or8way([O, I, I, O, O, O, O, O]), I);
        assert_eq!(or8way([O, O, O, I, I, I, O, O]), I);
        assert_eq!(or8way([I, O, I, O, O, O, I, I]), I);
        assert_eq!(or8way([I, O, I, O, I, O, I, I]), I);
        assert_eq!(or8way([I, I, I, I, O, I, I, O]), I);
        assert_eq!(or8way([I, I, O, I, I, I, I, I]), I);
        assert_eq!(or8way([I, I, I, I, I, I, I, I]), I);
    }

    #[test]
    fn for_mux4way16() {
        assert_eq!(
            mux4way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]),
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                [O, O],
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            mux4way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]),
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                [O, I],
            ),
            Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I])
        );
        assert_eq!(
            mux4way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]),
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                [I, O],
            ),
            Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O])
        );
        assert_eq!(
            mux4way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]),
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]),
                Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                [I, I],
            ),
            Word([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
    }

    #[test]
    fn for_mux8way16() {
        assert_eq!(
            mux8way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [O, O, O],
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            mux8way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [O, O, I],
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I])
        );
        assert_eq!(
            mux8way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [O, I, O],
            ),
            Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O])
        );
        assert_eq!(
            mux8way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [O, I, I],
            ),
            Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O])
        );
        assert_eq!(
            mux8way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [I, O, O],
            ),
            Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O])
        );
        assert_eq!(
            mux8way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [I, O, I],
            ),
            Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            mux8way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [I, I, O],
            ),
            Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O])
        );
        assert_eq!(
            mux8way16(
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                Word([O, O, O, O, O, O, O, O, O, O, O, O, I, I, O, O]),
                Word([O, O, O, O, O, O, O, O, O, O, I, I, O, O, O, O]),
                Word([O, O, O, O, O, O, O, O, I, I, O, O, O, O, O, O]),
                Word([O, O, O, O, O, O, I, I, O, O, O, O, O, O, O, O]),
                Word([O, O, O, O, I, I, O, O, O, O, O, O, O, O, O, O]),
                Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O]),
                [I, I, I],
            ),
            Word([O, O, I, I, O, O, O, O, O, O, O, O, O, O, O, O])
        );
    }

    #[test]
    fn for_dmux4way() {
        assert_eq!(dmux4way(O, [O, O]), [O, O, O, O]);
        assert_eq!(dmux4way(O, [O, I]), [O, O, O, O]);
        assert_eq!(dmux4way(O, [I, O]), [O, O, O, O]);
        assert_eq!(dmux4way(O, [I, I]), [O, O, O, O]);
        assert_eq!(dmux4way(I, [O, O]), [I, O, O, O]);
        assert_eq!(dmux4way(I, [O, I]), [O, I, O, O]);
        assert_eq!(dmux4way(I, [I, O]), [O, O, I, O]);
        assert_eq!(dmux4way(I, [I, I]), [O, O, O, I]);
    }

    #[test]
    fn for_dmux8way() {
        assert_eq!(dmux8way(O, [O, O, O]), [O, O, O, O, O, O, O, O]);
        assert_eq!(dmux8way(O, [O, O, I]), [O, O, O, O, O, O, O, O]);
        assert_eq!(dmux8way(O, [O, I, O]), [O, O, O, O, O, O, O, O]);
        assert_eq!(dmux8way(O, [O, I, I]), [O, O, O, O, O, O, O, O]);
        assert_eq!(dmux8way(O, [I, O, O]), [O, O, O, O, O, O, O, O]);
        assert_eq!(dmux8way(O, [I, O, I]), [O, O, O, O, O, O, O, O]);
        assert_eq!(dmux8way(O, [I, I, O]), [O, O, O, O, O, O, O, O]);
        assert_eq!(dmux8way(O, [I, I, I]), [O, O, O, O, O, O, O, O]);
        assert_eq!(dmux8way(I, [O, O, O]), [I, O, O, O, O, O, O, O]);
        assert_eq!(dmux8way(I, [O, O, I]), [O, I, O, O, O, O, O, O]);
        assert_eq!(dmux8way(I, [O, I, O]), [O, O, I, O, O, O, O, O]);
        assert_eq!(dmux8way(I, [O, I, I]), [O, O, O, I, O, O, O, O]);
        assert_eq!(dmux8way(I, [I, O, O]), [O, O, O, O, I, O, O, O]);
        assert_eq!(dmux8way(I, [I, O, I]), [O, O, O, O, O, I, O, O]);
        assert_eq!(dmux8way(I, [I, I, O]), [O, O, O, O, O, O, I, O]);
        assert_eq!(dmux8way(I, [I, I, I]), [O, O, O, O, O, O, O, I]);
    }
}