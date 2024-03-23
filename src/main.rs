mod key_matrix;

use key_matrix::{KeyMatrix, Keystroke};

fn listener(keystroke: Keystroke) {
    println!("listener: {}", keystroke);
}

fn main() {
    let mut key_matrix = KeyMatrix::new(listener);
    key_matrix.listen();
}
