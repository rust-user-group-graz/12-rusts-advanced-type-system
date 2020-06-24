use std::fmt;
use std::ops::Deref;

struct Wrapped<T: fmt::Display> {
    value: T,
    depth: usize,
}

impl<T: fmt::Display> fmt::Display for Wrapped<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}{}{}", "(".repeat(self.depth), self.value, ")".repeat(self.depth))
  }
}
impl<T: fmt::Display> Deref for Wrapped<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &Wrapped { value: self.value, depth: self.depth - 1 }
  }
}

fn main() {
    let v = Wrapped{ value: 1, depth: 8 };
    println!("{}", v);
    println!("{}", ****v);
    println!("{}", ********v);
}