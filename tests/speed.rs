use typenum::{U4};
use philox::{GenericArray, Philox4x32_10};

struct Counter(GenericArray<u32, U4>);

impl Counter {
  fn inc(&mut self) {
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

#[test] fn speed() {
  use std::io::Write;
  let mut f1 = std::fs::File::create("2-6-0xffth").unwrap();
  let mut ph = Philox4x32_10::default();
  let key = GenericArray::from_slice(&[2, 6]);
  let mut ctr = Counter(GenericArray::default());
  let now = std::time::Instant::now;
  let t1 = now();
  loop {
    let r = ph.next(key.clone(), ctr.0.clone());
    ctr.inc();
    if ctr.0[0] & 0x7fff == 0 {
      if now() - t1 > std::time::Duration::from_millis(1500) {
        //break;
      }
    }
    if ctr.0[0] & 0xff == 0 {
      write!(f1, "{:08x} {:08x} {:08x} {:08x}\n", r[3], r[2], r[1], r[0]).unwrap();
      if ctr.0[0] >= 0x100000 {
        break;
      }
    }
  }
  let dt = now() - t1;
  let secs = dt.as_secs() as f32 + 1e-3 * dt.subsec_millis() as f32;
  let nn = ((ctr.0[1] as u64) << 32) + ctr.0[0] as u64;
  eprintln!("Duration: {:?}  MB/s: {:.0}", secs, (nn as f32 * 16.) / secs / 1024.0 / 1024.0);
}

fn main() {
  if std::env::args().len() == 1 {
    speed();
  }
}
