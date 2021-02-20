use crate::logic::Bit::{I, O};
use crate::logic::*;

pub fn half_adder(a: Bit, b: Bit) -> [Bit; 2] {
  [and(a, b), xor(a, b)]
}

pub fn full_adder(a: Bit, b: Bit, c: Bit) -> [Bit; 2] {
  let res1 = half_adder(a, b);
  let res2 = half_adder(c, res1[1]);
  [or(res1[0], res2[0]), res2[1]]
}

pub fn add16(a: Word, b: Word) -> Word {
  let res15 = half_adder(a[15], b[15]);
  let res14 = full_adder(a[14], b[14], res15[0]);
  let res13 = full_adder(a[13], b[13], res14[0]);
  let res12 = full_adder(a[12], b[12], res13[0]);
  let res11 = full_adder(a[11], b[11], res12[0]);
  let res10 = full_adder(a[10], b[10], res11[0]);
  let res9 = full_adder(a[9], b[9], res10[0]);
  let res8 = full_adder(a[8], b[8], res9[0]);
  let res7 = full_adder(a[7], b[7], res8[0]);
  let res6 = full_adder(a[6], b[6], res7[0]);
  let res5 = full_adder(a[5], b[5], res6[0]);
  let res4 = full_adder(a[4], b[4], res5[0]);
  let res3 = full_adder(a[3], b[3], res4[0]);
  let res2 = full_adder(a[2], b[2], res3[0]);
  let res1 = full_adder(a[1], b[1], res2[0]);
  let res0 = full_adder(a[0], b[0], res1[0]);
  Word::new([
    res0[1], res1[1], res2[1], res3[1], res4[1], res5[1], res6[1], res7[1], res8[1], res9[1],
    res10[1], res11[1], res12[1], res13[1], res14[1], res15[1],
  ])
}

pub fn inc16(a: Word) -> Word {
  add16(
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
pub fn alu(
  x: Word,
  y: Word,
  zx: Bit,
  nx: Bit,
  zy: Bit,
  ny: Bit,
  f: Bit,
  no: Bit,
) -> (Word, Bit, Bit) {
  let x1 = mux16(x, Word::new([O; 16]), zx);
  let x2 = mux16(x1, not16(x1), nx);
  let y1 = mux16(y, Word::new([O; 16]), zy);
  let y2 = mux16(y1, not16(y1), ny);
  let f_res = mux16(and16(x2, y2), add16(x2, y2), f);
  let out = mux16(f_res, not16(f_res), no);
  let zr = not(or(
    or8way([
      out[0], out[1], out[2], out[3], out[4], out[5], out[6], out[7],
    ]),
    or8way([
      out[8], out[9], out[10], out[11], out[12], out[13], out[14], out[15],
    ]),
  ));
  let ng = out[0];
  (out, zr, ng)
}

#[cfg(test)]
mod tests {
  use super::{add16, alu, full_adder, half_adder, inc16};
  use crate::logic::Bit::{I, O};
  use crate::logic::Word;
  #[test]
  fn for_half_adder() {
    assert_eq!(half_adder(O, O), [O, O]);
    assert_eq!(half_adder(O, I), [O, I]);
    assert_eq!(half_adder(I, O), [O, I]);
    assert_eq!(half_adder(I, I), [I, O]);
  }

  #[test]
  fn for_full_adder() {
    assert_eq!(full_adder(O, O, O), [O, O]);
    assert_eq!(full_adder(O, O, I), [O, I]);
    assert_eq!(full_adder(O, I, O), [O, I]);
    assert_eq!(full_adder(O, I, I), [I, O]);
    assert_eq!(full_adder(I, O, O), [O, I]);
    assert_eq!(full_adder(I, O, I), [I, O]);
    assert_eq!(full_adder(I, I, O), [I, O]);
    assert_eq!(full_adder(I, I, I), [I, I]);
  }

  #[test]
  fn for_add16() {
    assert_eq!(
      add16(Word::new([I; 16]), Word::new([O; 16])),
      Word::new([I; 16])
    );
    assert_eq!(
      add16(
        Word::new([O, O, O, O, I, I, I, I, O, I, O, I, O, O, I, I]),
        Word::new([O, I, I, O, O, I, O, O, I, O, O, I, I, O, I, O])
      ),
      Word::new([O, I, I, I, O, O, I, I, I, I, I, O, I, I, O, I])
    );
    assert_eq!(
      add16(
        Word::new([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
        Word::new([O, I, I, O, O, I, O, O, I, O, O, I, I, O, I, O])
      ),
      Word::new([O, I, I, O, O, I, O, O, I, O, O, I, I, O, O, I])
    );
  }

  #[test]
  fn for_inc16() {
    assert_eq!(inc16(Word::new([I; 16])), Word::new([O; 16]));
    assert_eq!(
      inc16(Word::new([O, O, I, I, O, I, O, I, I, I, I, I, I, O, I, I])),
      Word::new([O, O, I, I, O, I, O, I, I, I, I, I, I, I, O, O])
    );
    assert_eq!(
      inc16(Word::new([O, O, I, I, O, I, O, I, I, I, I, I, I, I, I, I])),
      Word::new([O, O, I, I, O, I, I, O, O, O, O, O, O, O, O, O])
    );
  }

  #[test]
  fn for_alu() {
    assert_eq!(
      // line 1
      alu(
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
      alu(
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
      alu(
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
      alu(
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
      alu(
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
      alu(
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
      alu(
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
