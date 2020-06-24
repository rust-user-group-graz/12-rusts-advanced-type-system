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
        &self.value
    }
}

fn main() {
    let v1 = Wrapped{ value: 42, depth: 0 };
    let v2 = Wrapped{ value: &v1, depth: 1 };
    let v3 = Wrapped{ value: &v2, depth: 2 };
    let v4 = Wrapped{ value: &v3, depth: 3 };
    println!("{}", v1);
    println!("{}", v2);
    println!("{}", *v2);
}
