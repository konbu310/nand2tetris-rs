#![allow(dead_code, non_snake_case)]

use crate::logic::bit::{I, O};
use crate::logic::*;

pub fn HalfAdder(a: bit, b: bit) -> [bit; 2] {
    [And(a, b), Xor(a, b)]
}

pub fn FullAdder(a: bit, b: bit, c: bit) -> [bit; 2] {
    let res1 = HalfAdder(a, b);
    let res2 = HalfAdder(c, res1[1]);
    [Or(res1[0], res2[0]), res2[1]]
}

pub fn Add16(a: Word, b: Word) -> Word {
    let res15 = HalfAdder(a[15], b[15]);
    let res14 = FullAdder(a[14], b[14], res15[0]);
    let res13 = FullAdder(a[13], b[13], res14[0]);
    let res12 = FullAdder(a[12], b[12], res13[0]);
    let res11 = FullAdder(a[11], b[11], res12[0]);
    let res10 = FullAdder(a[10], b[10], res11[0]);
    let res9 = FullAdder(a[9], b[9], res10[0]);
    let res8 = FullAdder(a[8], b[8], res9[0]);
    let res7 = FullAdder(a[7], b[7], res8[0]);
    let res6 = FullAdder(a[6], b[6], res7[0]);
    let res5 = FullAdder(a[5], b[5], res6[0]);
    let res4 = FullAdder(a[4], b[4], res5[0]);
    let res3 = FullAdder(a[3], b[3], res4[0]);
    let res2 = FullAdder(a[2], b[2], res3[0]);
    let res1 = FullAdder(a[1], b[1], res2[0]);
    let res0 = FullAdder(a[0], b[0], res1[0]);
    Word::new([
        res0[1], res1[1], res2[1], res3[1], res4[1], res5[1], res6[1], res7[1], res8[1], res9[1],
        res10[1], res11[1], res12[1], res13[1], res14[1], res15[1],
    ])
}

pub fn Inc16(a: Word) -> Word {
    Add16(
        a,
        Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I]),
    )
}

/*
  入力:
    x:  Word
    y:  Word
    zx: 入力 x を O にする
    nx: 入力 x を 反転 する
    zy: 入力 y を O にする
    ny: 入力 y を 反転 する
    f:  I は 加算、O は AND演算
    no: 出力を 反転 する
  出力:
    out Word
    zr: out = 0 で I
    ng: out < 0 で I
*/
pub fn ALU(
    x: Word,
    y: Word,
    zx: bit,
    nx: bit,
    zy: bit,
    ny: bit,
    f: bit,
    no: bit,
) -> (Word, bit, bit) {
    let x1 = Mux16(x, Word::new([O; 16]), zx);
    let x2 = Mux16(x1, Not16(x1), nx);
    let y1 = Mux16(y, Word::new([O; 16]), zy);
    let y2 = Mux16(y1, Not16(y1), ny);
    let f_res = Mux16(And16(x2, y2), Add16(x2, y2), f);
    let out = Mux16(f_res, Not16(f_res), no);
    let zr = Not(Or(
        Or8Way([
            out[0], out[1], out[2], out[3], out[4], out[5], out[6], out[7],
        ]),
        Or8Way([
            out[8], out[9], out[10], out[11], out[12], out[13], out[14], out[15],
        ]),
    ));
    let ng = out[0];
    (out, zr, ng)
}

#[cfg(test)]
mod tests {
    use super::{Add16, FullAdder, HalfAdder, Inc16, ALU};
    use crate::logic::bit::{I, O};
    use crate::logic::Word;
    #[test]
    fn for_halfadder() {
        assert_eq!(HalfAdder(O, O), [O, O]);
        assert_eq!(HalfAdder(O, I), [O, I]);
        assert_eq!(HalfAdder(I, O), [O, I]);
        assert_eq!(HalfAdder(I, I), [I, O]);
    }

    #[test]
    fn for_fulladder() {
        assert_eq!(FullAdder(O, O, O), [O, O]);
        assert_eq!(FullAdder(O, O, I), [O, I]);
        assert_eq!(FullAdder(O, I, O), [O, I]);
        assert_eq!(FullAdder(O, I, I), [I, O]);
        assert_eq!(FullAdder(I, O, O), [O, I]);
        assert_eq!(FullAdder(I, O, I), [I, O]);
        assert_eq!(FullAdder(I, I, O), [I, O]);
        assert_eq!(FullAdder(I, I, I), [I, I]);
    }

    #[test]
    fn for_add16() {
        assert_eq!(
            Add16(Word::new([I; 16]), Word::new([O; 16])),
            Word::new([I; 16])
        );
        assert_eq!(
            Add16(
                Word::new([O, O, O, O, I, I, I, I, O, I, O, I, O, O, I, I]),
                Word::new([O, I, I, O, O, I, O, O, I, O, O, I, I, O, I, O])
            ),
            Word::new([O, I, I, I, O, O, I, I, I, I, I, O, I, I, O, I])
        );
        assert_eq!(
            Add16(
                Word::new([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                Word::new([O, I, I, O, O, I, O, O, I, O, O, I, I, O, I, O])
            ),
            Word::new([O, I, I, O, O, I, O, O, I, O, O, I, I, O, O, I])
        );
    }

    #[test]
    fn for_inc16() {
        assert_eq!(Inc16(Word::new([I; 16])), Word::new([O; 16]));
        assert_eq!(
            Inc16(Word::new([O, O, I, I, O, I, O, I, I, I, I, I, I, O, I, I])),
            Word::new([O, O, I, I, O, I, O, I, I, I, I, I, I, I, O, O])
        );
        assert_eq!(
            Inc16(Word::new([O, O, I, I, O, I, O, I, I, I, I, I, I, I, I, I])),
            Word::new([O, O, I, I, O, I, I, O, O, O, O, O, O, O, O, O])
        );
    }

    #[test]
    fn for_alu() {
        assert_eq!(
            // line 1
            ALU(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word::new([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                I,
                O,
                I,
                O,
                I,
                O
            ),
            (
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                I,
                O
            )
        );
        assert_eq!(
            // line 2
            ALU(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word::new([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                I,
                I,
                I,
                I,
                I,
                I
            ),
            (
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I]),
                O,
                O
            )
        );
        assert_eq!(
            // line 16
            ALU(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                Word::new([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                O,
                I,
                O,
                O,
                I,
                I
            ),
            (
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I]),
                O,
                O
            )
        );
        assert_eq!(
            // line 20
            ALU(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                I,
                O,
                I,
                O,
                I,
                O
            ),
            (
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]),
                I,
                O
            )
        );
        assert_eq!(
            // line 23
            ALU(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                O,
                O,
                I,
                I,
                O,
                O
            ),
            (
                Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
                O,
                O
            )
        );
        assert_eq!(
            // line 27
            ALU(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                O,
                O,
                I,
                I,
                I,
                I
            ),
            (
                Word::new([I, I, I, I, I, I, I, I, I, I, I, O, I, I, I, I]),
                O,
                I
            )
        );
        assert_eq!(
            // line 30
            ALU(
                Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
                Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I]),
                O,
                O,
                I,
                I,
                I,
                O
            ),
            (
                Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, O]),
                O,
                O
            )
        );
    }
}
