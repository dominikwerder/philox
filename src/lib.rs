/*
MIT License

Copyright (c) 2019 Dominik Werder <dominik.werder@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice (including the next paragraph) shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use std::ops::{Shl, Shr};
use generic_array::typenum::{Unsigned, U2, U4, U10, U16, U32};
pub use generic_array::{ArrayLength, GenericArray};

#[cfg(test)]
use generic_array::typenum::{U8, U64};


pub struct Counter<N: Unsigned + ArrayLength<u32>>(pub GenericArray<u32, N>);

impl<N: Unsigned + ArrayLength<u32>> Counter<N> {
  fn inc(&mut self) {
    // TODO
    assert_eq!(N::to_usize(), 4);
    let ctr = &mut self.0;
    let x = ctr[0].overflowing_add(1);
    ctr[0] = x.0;
    if x.1 {
      let x = ctr[1].overflowing_add(1);
      ctr[1] = x.0;
      if x.1 {
        let x = ctr[2].overflowing_add(1);
        ctr[2] = x.0;
        if x.1 {
          let x = ctr[3].overflowing_add(1);
          ctr[3] = x.0;
        }
      }
    }
  }
}

type Ph<N, W, R> = Philox<N, W, R, <N as std::ops::Div<U2>>::Output>;
pub type Philox4x32_10 = Ph<U4, U32, U10>;

pub struct Philox<N: Unsigned + ArrayLength<u32>, W: Unsigned, R: Unsigned, KN: ArrayLength<u32>> {
  key: GenericArray<u32, KN>,
  ctr: Counter<N>,
  _m2: std::marker::PhantomData<W>,
  _m3: std::marker::PhantomData<R>,
}

struct HiLo(u32, u32);

fn mulhilo(a: u32, b: u32) -> HiLo {
  let p = (a as u64).wrapping_mul(b as u64);
  HiLo((p >> 32) as u32, p as u32)
}

impl<N: Unsigned + ArrayLength<u32> + Shr, W: Unsigned, R: Unsigned, KN: ArrayLength<u32>> Philox<N, W, R, KN> {
  pub fn key_default() -> GenericArray<u32, KN> {
    Default::default()
  }
  pub fn from_key(key: GenericArray<u32, KN>) -> Self {
    assert_eq!(W::to_usize(), 32);
    Self {
      key,
      ctr: Counter(Default::default()),
      _m2: Default::default(),
      _m3: Default::default(),
    }
  }
  pub fn set_ctr(mut self, ctr: GenericArray<u32, N>) -> Self {
    self.ctr = Counter(ctr);
    self
  }
  pub fn next(&mut self) -> GenericArray<u32, N> {
    let mut key = self.key.clone();
    let mut ctr = self.ctr.0.clone();
    for _ in 0..R::USIZE {
      self.round(&mut key, &mut ctr);
      self.update_key(&mut key);
    }
    self.ctr.inc();
    ctr
  }
  pub fn next_bytes(&mut self) -> GenericArray<u8, U16> {
    // TODO make generic, but somehow can not divide typenum in return type
    type _AB1 = <U4 as Shl<U2>>::Output;
    type _AB2 = <U4 as Shr<U2>>::Output;
    assert_eq!(N::to_usize(), 4);
    assert_eq!(W::to_usize(), 32);
    let x = self.next();
    unsafe { *(x.as_ptr() as *const GenericArray<u8, U16>) }
  }
  fn round(&mut self, key: &mut GenericArray<u32, KN>, ctr: &mut GenericArray<u32, N>) {
    // These constants were chosen just because the random numbers look statistically best
    #[allow(non_upper_case_globals)] const PHILOX_M4x32_0: u32 = 0xD2511F53;
    #[allow(non_upper_case_globals)] const PHILOX_M4x32_1: u32 = 0xCD9E8D57;
    let HiLo(hi0, lo0) = mulhilo(PHILOX_M4x32_0, ctr[0]);
    let HiLo(hi1, lo1) = mulhilo(PHILOX_M4x32_1, ctr[2]);
    ctr[0] = hi1 ^ ctr[1] ^ key[0];
    ctr[2] = hi0 ^ ctr[3] ^ key[1];
    ctr[1] = lo1;
    ctr[3] = lo0;
  }
  fn update_key(&mut self, key: &mut GenericArray<u32, KN>) {
    const C0: u32 = 0x9E3779B9;
    const C1: u32 = 0xBB67AE85;
    key[0] = key[0].wrapping_add(C0); // golden ratio
    key[1] = key[1].wrapping_add(C1); // sqrt(3) - 1
  }
  pub fn ctr(&self) -> &Counter<N> { &self.ctr }
}


#[test] fn typenum() {
  assert_eq!(U64::to_u32(), 64);
  assert_eq!(<U64 as std::ops::Div<U8>>::Output::to_u32(), 8);
  assert_eq!(<U64 as std::ops::Mul<U2>>::Output::to_u32(), 128);
  assert_eq!(U64::U32, 64);
}

#[test] fn arithmetic() {
  assert_eq!((0xffffffffffffffff_u64 >> 32) as u32, 0xffffffff_u32);
  assert_eq!(0x12345678cafe1337_u64 as u32, 0xcafe1337_u32);
}

#[cfg(test)]
fn parse_test_vector(s: &str) -> (GenericArray<u32, U2>, GenericArray<u32, U4>, GenericArray<u32, U4>) {
  let a: Vec<_> = s.split(" ").map(|x| x.trim()).collect();
  type T = u32;
  fn p(s: &str) -> T {
    T::from_str_radix(s, 16).unwrap()
  }
  (
    [p(a[6]), p(a[7])].into(),
    [p(a[2]), p(a[3]), p(a[4]), p(a[5])].into(),
    [p(a[8]), p(a[9]), p(a[10]), p(a[11])].into(),
  )
}

#[test] fn test_parse_test_vector() {
  /*
  Test vectors contain first the counter, then the key, then the result.
  */
  let s = "philox4x32 10 243f6a88 85a308d3 13198a2e 03707344 a4093822 299f31d0 d16cfe09 94fdcceb 5001e420 24126ea1";
  let v = parse_test_vector(s);
  assert_eq!(v.0.as_slice(), &[0xa4093822, 0x299f31d0]);
}

#[cfg(test)]
fn split_test_vectors() -> Vec<String> {
  "
  philox4x32 10 00000000 00000000 00000000 00000000 00000000 00000000 6627e8d5 e169c58d bc57ac4c 9b00dbd8
  philox4x32 10 ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff 408f276d 41c83b0e a20bc7c6 6d5451fd
  philox4x32 10 243f6a88 85a308d3 13198a2e 03707344 a4093822 299f31d0 d16cfe09 94fdcceb 5001e420 24126ea1
  ".trim().split("\n").map(|x|x.trim().to_string()).collect()
}

#[test] fn check_test_vectors_array_u32() {
  for v in &split_test_vectors() {
    let v = parse_test_vector(v);
    let mut ph = Ph::<U4, U32, U10>::from_key(v.0).set_ctr(v.1);
    let r = ph.next();
    assert_eq!(r.as_slice(), v.2.as_slice());
  }
}

#[test] fn check_test_vectors_bytes() {
  for v in &split_test_vectors() {
    let v = parse_test_vector(v);
    let mut ph = Ph::<U4, U32, U10>::from_key(v.0).set_ctr(v.1);
    let r = ph.next_bytes();
    use std::mem::transmute as tr;
    let r = unsafe { [
      tr::<_, u32>(*(&r[0] as *const _ as *const [u8;4])),
      tr::<_, u32>(*(&r[4] as *const _ as *const [u8;4])),
      tr::<_, u32>(*(&r[8] as *const _ as *const [u8;4])),
      tr::<_, u32>(*(&r[12] as *const _ as *const [u8;4])),
    ] };
    assert_eq!(&r, v.2.as_slice());
  }
}
