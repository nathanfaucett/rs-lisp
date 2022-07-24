use core::fmt::{self, Write};

use hashbrown::HashMap;

#[derive(Default)]
pub struct LispMap<K, V>(pub HashMap<K, V>);

impl<K, V> fmt::Debug for LispMap<K, V>
where
  K: fmt::Debug,
  V: fmt::Debug,
{
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.0.is_empty() {
      f.write_str("{}")
    } else {
      f.write_char('{')?;
      let mut index = self.0.len();
      for (k, v) in self.0.iter() {
        write!(f, "{:?} {:?}", k, v)?;
        index -= 1;
        if index != 0 {
          write!(f, ", ")?;
        }
      }
      f.write_char('}')
    }
  }
}
