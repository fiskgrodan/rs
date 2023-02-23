use std::time::Instant;

fn main() {
  let start = Instant::now();
  let mut last = Instant::now();

  loop {
    if start.elapsed().as_millis() >= 1000 {
      break;
    }

    if last.elapsed().as_millis() >= 100 {
      println!("{}", last.elapsed().as_millis());
      last = Instant::now();
    }
  }

  println!("{}", start.elapsed().as_millis());
}
