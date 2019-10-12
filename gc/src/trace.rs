use alloc::string::String;

pub trait Trace {
  #[inline(always)]
  fn is_marked(&self) -> bool {
    true
  }
  #[inline(always)]
  fn trace(&mut self, _marked: bool) {}
}

impl Trace for () {}

impl Trace for String {}

impl Trace for bool {}

impl Trace for char {}

impl Trace for u8 {}
impl Trace for u16 {}
impl Trace for u32 {}
impl Trace for u64 {}
impl Trace for usize {}

impl Trace for i8 {}
impl Trace for i16 {}
impl Trace for i32 {}
impl Trace for i64 {}
impl Trace for isize {}

impl Trace for f32 {}
impl Trace for f64 {}

impl<T> Trace for Option<T>
where
  T: Trace,
{
  #[inline(always)]
  fn is_marked(&self) -> bool {
    self.as_ref().map(|value| value.is_marked()).unwrap_or(true)
  }
  #[inline(always)]
  fn trace(&mut self, marked: bool) {
    self.as_mut().map(|value| value.trace(marked));
  }
}
