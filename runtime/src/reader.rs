use alloc::string::String;
use alloc::vec;
use core::num::ParseIntError;
use core::str::FromStr;

use gc::Gc;

use super::{
  new_char, new_escape, new_i16, new_i32, new_i64, new_i8, new_isize, new_keyword,
  new_keyword_with_meta, new_map_from, new_persistent_list_from_with_meta,
  new_persistent_map_from_with_meta, new_persistent_vector_from_with_meta, new_string,
  new_symbol_with_meta, new_u16, new_u32, new_u64, new_u8, new_usize, nil_value, Escape, Keyword,
  Map, Object, PersistentList, PersistentMap, PersistentScope, PersistentVector, Symbol, Value,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Reader {
  filename: Option<String>,
  index: usize,
  line: usize,
  col: usize,
  chars: vec::Vec<char>,
}

impl Reader {
  #[inline]
  pub fn new(filename: Option<String>, chars: vec::Vec<char>) -> Self {
    Reader {
      filename,
      index: 0,
      line: 1,
      col: 0,
      chars: chars,
    }
  }
  #[inline]
  pub fn filename(&self) -> Option<&String> {
    self.filename.as_ref()
  }
  #[inline]
  pub fn line(&self) -> usize {
    self.line
  }
  #[inline]
  pub fn col(&self) -> usize {
    self.col
  }
  #[inline]
  fn consume(&mut self) -> &mut Self {
    if self.peek().map(is_newline).unwrap_or(false) {
      self.line += 1;
      self.col = 0;
    } else {
      self.col += 1;
    }
    self.index += 1;
    self
  }
  #[inline]
  fn next(&mut self) -> Option<char> {
    let index = self.index;
    self.consume();
    self.chars.get(index).map(Clone::clone)
  }
  #[inline]
  fn peek(&self) -> Option<char> {
    self.peek_nth(0)
  }
  #[inline]
  fn peek_nth(&self, index: usize) -> Option<char> {
    self.chars.get(self.index + index).map(Clone::clone)
  }
}

#[inline]
pub fn read_value(scope: &Gc<Object<PersistentScope>>, reader: &mut Reader) -> Gc<dyn Value> {
  while let Some(ch) = reader.peek() {
    if is_whitespace(ch) {
      reader.consume();
    } else {
      match ch {
        '(' => {
          reader.consume();
          return read_list(scope, reader).into_value();
        }
        '[' => {
          reader.consume();
          return read_vec(scope, reader).into_value();
        }
        '{' => {
          reader.consume();
          return read_map(scope, reader).into_value();
        }
        '"' => {
          reader.consume();
          return read_string(scope, reader).into_value();
        }
        '\'' => {
          reader.consume();
          return read_char(scope, reader).into_value();
        }
        ':' => {
          reader.consume();
          return read_keyword(scope, reader).into_value();
        }
        '`' => {
          reader.consume();
          return read_escape(scope, reader).into_value();
        }
        ';' => {
          return read_comment(scope, reader);
        }
        ch => {
          if is_numeric(reader, ch) {
            reader.consume();
            return read_number(scope, reader, ch);
          } else {
            return read_symbol(scope, reader).into_value();
          }
        }
      }
    }
  }

  nil_value(scope).clone().into_value()
}

#[inline]
fn read_list(
  scope: &Gc<Object<PersistentScope>>,
  reader: &mut Reader,
) -> Gc<Object<PersistentList>> {
  let mut persistent_list = PersistentList::new();
  let meta = create_meta(scope, reader);

  while let Some(ch) = reader.peek() {
    if ch == ')' {
      reader.consume();
      break;
    } else if is_whitespace(ch) {
      reader.consume();
    } else {
      persistent_list = persistent_list.push_back(read_value(scope, reader));
    }
  }

  new_persistent_list_from_with_meta(scope, persistent_list, Some(meta))
}

#[inline]
fn read_vec(
  scope: &Gc<Object<PersistentScope>>,
  reader: &mut Reader,
) -> Gc<Object<PersistentVector>> {
  let mut persistent_vector = PersistentVector::new();
  let meta = create_meta(scope, reader);

  while let Some(ch) = reader.peek() {
    if ch == ']' {
      reader.consume();
      break;
    } else if is_whitespace(ch) {
      reader.consume();
    } else {
      persistent_vector = persistent_vector.push(read_value(scope, reader));
    }
  }

  new_persistent_vector_from_with_meta(scope, persistent_vector, Some(meta))
}

#[inline]
fn read_map(scope: &Gc<Object<PersistentScope>>, reader: &mut Reader) -> Gc<Object<PersistentMap>> {
  let mut persistent_map = PersistentMap::new();
  let meta = create_meta(scope, reader);

  while let Some(ch) = reader.peek() {
    if ch == '}' {
      reader.consume();
      break;
    } else if is_whitespace(ch) {
      reader.consume();
    } else {
      let key = read_value(scope, reader);
      let mut value = nil_value(scope).clone().into_value();

      while let Some(ch) = reader.peek() {
        if ch == '}' {
          break;
        } else if is_whitespace(ch) {
          reader.consume();
        } else {
          value = read_value(scope, reader);
        }
      }

      persistent_map = persistent_map.set(key, value);
    }
  }

  new_persistent_map_from_with_meta(scope, persistent_map, Some(meta))
}

#[inline]
fn read_symbol(scope: &Gc<Object<PersistentScope>>, reader: &mut Reader) -> Gc<Object<Symbol>> {
  let mut string = String::new();
  let meta = create_meta(scope, reader);

  while let Some(ch) = reader.peek() {
    if is_closer(ch) || is_whitespace(ch) {
      break;
    } else {
      reader.consume();
      string.push(ch);
    }
  }

  new_symbol_with_meta(scope, string, Some(meta))
}

#[inline]
fn read_keyword(scope: &Gc<Object<PersistentScope>>, reader: &mut Reader) -> Gc<Object<Keyword>> {
  let mut string = String::new();
  let meta = create_meta(scope, reader);

  while let Some(ch) = reader.peek() {
    if is_closer(ch) || is_whitespace(ch) {
      break;
    } else {
      reader.consume();
      string.push(ch);
    }
  }

  new_keyword_with_meta(scope, string, Some(meta))
}

#[inline]
fn read_escape(scope: &Gc<Object<PersistentScope>>, reader: &mut Reader) -> Gc<Object<Escape>> {
  new_escape(scope, read_value(scope, reader))
}

#[inline]
fn read_comment(scope: &Gc<Object<PersistentScope>>, reader: &mut Reader) -> Gc<dyn Value> {
  while let Some(ch) = reader.peek() {
    if is_newline(ch) {
      break;
    } else {
      reader.consume();
    }
  }
  read_value(scope, reader)
}

#[inline]
fn read_string(scope: &Gc<Object<PersistentScope>>, reader: &mut Reader) -> Gc<Object<String>> {
  let mut string = String::new();
  let meta = create_meta(scope, reader);

  while let Some(ch) = reader.next() {
    if ch == '"' {
      break;
    } else {
      string.push(ch);
    }
  }

  let mut string = new_string(scope, string);
  string.set_meta(meta);
  string
}

#[inline]
fn read_char(scope: &Gc<Object<PersistentScope>>, reader: &mut Reader) -> Gc<Object<char>> {
  let mut string = String::new();
  let meta = create_meta(scope, reader);

  while let Some(ch) = reader.next() {
    if ch == '\'' {
      break;
    } else {
      string.push(ch);
    }
  }

  // TODO: actually get a char
  let ch = string.chars().nth(0).expect("failed to get char");
  let mut ch = new_char(scope, ch);

  ch.set_meta(meta);
  ch
}

#[inline]
fn read_number(
  scope: &Gc<Object<PersistentScope>>,
  reader: &mut Reader,
  ch: char,
) -> Gc<dyn Value> {
  let mut string = String::new();
  let meta = create_meta(scope, reader);

  let mut typ_size = String::new();
  let mut typ_char = 'i';
  let mut typ_read = false;

  let mut dot_read = ch == '.';
  let is_neg = ch == '-';

  if dot_read {
    string.push('0');
    string.push(ch);
  } else if is_neg {
    if let Some(ch) = reader.next() {
      string.push('-');
      string.push(ch);
    } else {
      string.push('0');
    }
  } else {
    string.push(ch);
  }

  while let Some(ch) = reader.peek() {
    if ch == '.' {
      if dot_read {
        break;
      } else {
        dot_read = true;
        reader.consume();
        string.push(ch);
      }
    } else if ch.is_numeric() {
      reader.consume();
      string.push(ch);
    } else if ch == '_' {
      reader.consume();
    } else if ch == 'i' || ch == 'u' || ch == 'f' {
      if typ_read {
        break;
      } else {
        typ_read = true;
        reader.consume();
        typ_char = ch;

        while let Some(ch) = reader.peek() {
          if ch.is_alphabetic() {
            reader.consume();
            typ_size.push(ch);
          } else {
            break;
          }
        }
      }
    } else {
      break;
    }
  }

  if !typ_read {
    if dot_read {
      typ_char = 'f';
      typ_size.push_str("64")
    } else {
      typ_size.push_str("size")
    }
  }

  let number = match typ_char {
    'u' => match from_uint(scope, &string, &typ_size, meta) {
      Ok(n) => n,
      // Err(_) => new_nan_f32(scope).into_value(),
      Err(_) => unimplemented!(),
    },
    // 'f' => match from_float(scope, &string, &typ_size) {
    //     Ok(n) => n,
    //     Err(_) => new_nan_f64(scope).into_value(),
    // },
    // 'i'
    _ => match from_int(scope, &string, &typ_size, meta) {
      Ok(n) => n,
      // Err(_) => new_nan_f32(scope).into_value(),
      Err(_) => unimplemented!(),
    },
  };

  number
}

#[inline]
fn from_int(
  scope: &Gc<Object<PersistentScope>>,
  value: &String,
  typ_size: &String,
  meta: Gc<Object<Map>>,
) -> Result<Gc<dyn Value>, ParseIntError> {
  Ok(match typ_size.as_str() {
    "8" => {
      let mut n = new_i8(scope, i8::from_str(value)?);
      n.set_meta(meta);
      n
    }
    .into_value(),
    "16" => {
      let mut n = new_i16(scope, i16::from_str(value)?);
      n.set_meta(meta);
      n
    }
    .into_value(),
    "32" => {
      let mut n = new_i32(scope, i32::from_str(value)?);
      n.set_meta(meta);
      n
    }
    .into_value(),
    "64" => {
      let mut n = new_i64(scope, i64::from_str(value)?);
      n.set_meta(meta);
      n
    }
    .into_value(),
    _ => {
      let mut n = new_isize(scope, isize::from_str(value)?);
      n.set_meta(meta);
      n
    }
    .into_value(),
  })
}

#[inline]
fn from_uint(
  scope: &Gc<Object<PersistentScope>>,
  value: &String,
  typ_size: &String,
  meta: Gc<Object<Map>>,
) -> Result<Gc<dyn Value>, ParseIntError> {
  Ok(match typ_size.as_str() {
    "8" => {
      let mut n = new_u8(scope, u8::from_str(value)?);
      n.set_meta(meta);
      n
    }
    .into_value(),
    "16" => {
      let mut n = new_u16(scope, u16::from_str(value)?);
      n.set_meta(meta);
      n
    }
    .into_value(),
    "32" => {
      let mut n = new_u32(scope, u32::from_str(value)?);
      n.set_meta(meta);
      n
    }
    .into_value(),
    "64" => {
      let mut n = new_u64(scope, u64::from_str(value)?);
      n.set_meta(meta);
      n
    }
    .into_value(),
    _ => {
      let mut n = new_usize(scope, usize::from_str(value)?);
      n.set_meta(meta);
      n
    }
    .into_value(),
  })
}

// #[inline]
// fn from_float(
//     scope: &Gc<Object<PersistentScope>>,
//     value: &String,
//     typ_size: &String,
// ) -> Result<Gc<dyn Value>, ParseFloatError> {
//     Ok(match typ_size.as_str() {
//         "32" => new_f32(scope, f32::from_str(value)?).into_value(),
//         _ => new_f64(scope, f64::from_str(value)?).into_value(),
//     })
// }

#[inline]
fn create_meta(scope: &Gc<Object<PersistentScope>>, reader: &mut Reader) -> Gc<Object<Map>> {
  let mut meta = Map::new();
  meta.set(
    new_keyword(scope, "filename").into_value(),
    new_string(
      scope,
      reader
        .filename()
        .map(Clone::clone)
        .unwrap_or_else(String::new),
    )
    .into_value(),
  );
  meta.set(
    new_keyword(scope, "line").into_value(),
    new_usize(scope, reader.line()).into_value(),
  );
  meta.set(
    new_keyword(scope, "col").into_value(),
    new_usize(scope, reader.col()).into_value(),
  );
  new_map_from(scope, meta)
}

#[inline]
fn is_newline(ch: char) -> bool {
  ch == '\n'
}

#[inline]
fn is_whitespace(ch: char) -> bool {
  ch.is_whitespace() || ch == ','
}

#[inline]
fn is_closer(ch: char) -> bool {
  ch == ')' || ch == ']' || ch == '}'
}

#[inline]
fn is_numeric(reader: &mut Reader, ch: char) -> bool {
  if ch.is_numeric() {
    return true;
  }
  if ch == '-' {
    if let Some(next_ch) = reader.peek_nth(1) {
      return next_ch.is_numeric();
    }
  }
  return false;
}
