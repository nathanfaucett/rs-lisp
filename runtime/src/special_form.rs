use std::fmt;
use std::hash::{Hash, Hasher};

use super::Stack;

pub struct SpecialForm(Box<Fn(&mut Stack)>);

impl fmt::Debug for SpecialForm {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SpecialForm")
    }
}

impl PartialEq for SpecialForm {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        ::std::ptr::eq(self, other)
    }
}

impl Eq for SpecialForm {}

impl Hash for SpecialForm {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self as *const Self).hash(state)
    }
}

impl SpecialForm {
    #[inline(always)]
    pub fn new<F>(f: F) -> Self
    where
        F: 'static + Fn(&mut Stack),
    {
        SpecialForm(Box::new(f))
    }

    #[inline(always)]
    pub fn inner(&self) -> &Fn(&mut Stack) {
        &*self.0
    }
    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut Fn(&mut Stack) {
        &mut *self.0
    }
}

impl<'a> FnOnce<(&'a mut Stack)> for SpecialForm {
    type Output = ();

    #[inline(always)]
    extern "rust-call" fn call_once(self, stack: (&mut Stack)) -> Self::Output {
        (self.0)(stack)
    }
}

impl<'a> Fn<(&'a mut Stack)> for SpecialForm {
    #[inline(always)]
    extern "rust-call" fn call(&self, stack: (&mut Stack)) -> Self::Output {
        self.inner()(stack)
    }
}

impl<'a> FnMut<(&'a mut Stack)> for SpecialForm {
    #[inline(always)]
    extern "rust-call" fn call_mut(&mut self, stack: (&mut Stack)) -> Self::Output {
        self.inner_mut()(stack)
    }
}
