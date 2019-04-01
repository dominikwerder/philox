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

fn speed() {
  let t1 = std::time::Instant::now();
  let mut ph = Philox4x32_10::default();
  let key = GenericArray::from_slice(&[2, 6]);
  let mut ctr = Counter(GenericArray::default());
  let nn = 10000000;
  for _ in 0..nn {
    ph.next(key.clone(), ctr.0.clone());
    ctr.inc();
  }
  let t2 = std::time::Instant::now();
  let dt = t2 - t1;
  let secs = dt.as_secs() as f32 + 1e-3 * dt.subsec_millis() as f32;
  println!("Duration: {:?}  MB/s: {:.0}", secs, (nn as f32 * 16.) / secs / 1024.0 / 1024.0);
}

fn main() {
  if std::env::args().len() == 1 {
    speed();
  }
}
