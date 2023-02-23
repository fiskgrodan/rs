use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

#[derive(Debug)]
struct State {
  foo: u8,
  bar: u8,
}

impl State {
  fn up_foo(&mut self) {
    self.foo = self.foo.checked_add(1).unwrap_or(u8::MIN);
  }
  fn up_bar(&mut self) {
    self.bar = self.bar.checked_add(1).unwrap_or(u8::MIN);
  }
}

fn main() {
  let safe = Arc::new(Mutex::new(State { foo: u8::MIN, bar: u8::MIN }));

  {
    let safe = Arc::clone(&safe);
    thread::spawn(move|| {
      loop {
        let mut state = safe.lock().unwrap();
        state.up_foo();
      }
    });
  }

  {
    let safe = Arc::clone(&safe);
    thread::spawn(move|| {
      loop {
        let mut state = safe.lock().unwrap();
        state.up_bar();
      }
    });
  }

  println!("{:?}", *safe.lock().unwrap());
  thread::sleep(Duration::from_millis(333));
  println!("{:?}", *safe.lock().unwrap());
  thread::sleep(Duration::from_millis(333));
  println!("{:?}", *safe.lock().unwrap());
  thread::sleep(Duration::from_millis(333));
  println!("{:?}", *safe.lock().unwrap());
}
