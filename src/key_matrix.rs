use std::{thread, time};

pub type Keystroke = String;

pub struct KeyMatrix<'a> {
  listener: Box<dyn FnMut(Keystroke) + 'a>,
}

impl<'a> KeyMatrix<'a> {
  pub fn new(listener: impl FnMut(Keystroke) + 'a) -> Self {
    KeyMatrix {
      listener: Box::new(listener),
    }
  }

  fn emit_keystroke(&mut self, keystroke: String) {
    (self.listener)(keystroke);
  }

  pub fn listen(&mut self) {
    loop {
      self.emit_keystroke(String::from("Test"));

      // TODO: Probably need something else when running bare metal
      thread::sleep(time::Duration::from_secs(2));
    }
  }
}
