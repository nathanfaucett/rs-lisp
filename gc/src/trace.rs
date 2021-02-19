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
        match self.as_ref() {
            Some(value) => value.is_marked(),
            None => true,
        }
    }
    #[inline(always)]
    fn trace(&mut self, marked: bool) {
        match self.as_mut() {
            Some(value) => value.trace(marked),
            None => {}
        }
    }
}

impl<T, E> Trace for Result<T, E>
where
    T: Trace,
    E: Trace,
{
    #[inline(always)]
    fn is_marked(&self) -> bool {
        match self.as_ref() {
            Ok(ok) => ok.is_marked(),
            Err(err) => err.is_marked(),
        }
    }
    #[inline(always)]
    fn trace(&mut self, marked: bool) {
        match self.as_mut() {
            Ok(ok) => ok.trace(marked),
            Err(err) => err.trace(marked),
        }
    }
}
