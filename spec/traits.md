# Traits

A trait is a collection of methods defined for an unknown type.

### Define a Trait

```
(pub (trait TMytrait <A, B, (B, (default, Self))> (Debug)
	(type MyType, (impl Debug), (default Self))

	(fn a ([(self Self), (a A)] Self))
	(fn b ([(self Self), (b B)] Self))

	(fn c ([self, (a A)] Self ))
))
```

```
// C defaults to Self
// Any thing that implements TMyTrait must implement Debug
pub trait TMyTrait <A, B, C = Self>: Debug {
	// MyType defaults to Self
	// MyType must implement Debug
	type MyType: Debug = Self;

	fn a (self, a: A) -> Self
	fn b (self, b: B) -> Self

	// implementors can override default definition
	fn c (self, c: C) -> C {
		c
	}
}

impl <A, B, C> TMyTrait::<A, B, C> for MyStruct {
	type MyType = F32

	fn a (self, a: A) -> Self {
		self
	}
	fn b (self, b: B) -> Self {
		self
	}
}
```

### Implement a Trait

```
// Rhs and Output defaults to Self
@lang #{ name: "ops::add" }
pub trait TAdd <Rhs = Self> {
	type Output = Self

	fn add (self, other: Rhs) -> Self::Output
}

pub struct Wrapping <T> (T)

impl <L, R, O> TAdd::<Wrapping::<R>> for Wrapping::<L>
	where L: TAdd::<R, Output = O>
{
	type Output = Wrapping::<O>

	fn add(self, other: Wrapping::<R>) -> Self::Output {
		Wrapping(core::ops::add(self.0, other.0))
	}
}

// TAdd is special because it is called by the compiler
Wrapping(1) + Wrapping(1) // Wrapping(2)

// most traits will be called like this
TAdd::add(Wrapping(1), Wrapping(1)) // Wrapping(2)

// if TAdd is in the current scope
Wrapping(1).add(Wrapping(1)) // Wrapping(2)
```
