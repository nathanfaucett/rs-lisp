# Structs

There are two types of structures that can be created using the struct keyword

### Map Structs

```
pub trait New<Args>
	where Args: TTuple
{
	fn new Args -> Self
}

pub struct MapStruct {
	a: Int32
}

impl<(a: Int32)> New<(a: Int32)> for MapStruct {
	fn new(a: Int32) -> Self MapStruct { a }
}

pub struct GenericMapStruct <A, B, C> {
	a: A,
	b: B,
	// c is public but a and b are private
	pub c: C,
}
```

### Tulpe Structs

Basically named tuples

```
pub struct TupleStruct (Int32)
pub struct GenericTupleStruct <A, B, C> (A, B, C)
pub struct UnitStruct()
```

### Add methods

```
pub struct Vector2 <T> {
	pub x: T,
	pub y: T,
}

impl <T> Vector2::<T> {
	// Static Method
	pub fn new(x: T, y: T) -> Self {
		// infer Vector2::<T>
		Vector2 { x: x, y: y }
	}

	// Member Method
	// only available if T implements TAdd, TMul, and TSqrt
	pub fn length(self) -> T
		where T: TAdd + TMul + TSqrt
	{
		Self::dot(self, self).sqrt()
	}

	// Static Method
	// only available if T implements TAdd and TMul
	pub fn dot(left: Self, right: Self) -> T
		where T: TAdd + TMul
	{
		left.x * right.x + left.y * right.y
	}
}
```

### Instantiate Structs

```
// create new map struct
Vector2{ x: 0, y: 0 } // Vector2::<ISize>

// create tuple struct
GenericTupleStruct("String", 0, 1.0) // GenericTupleStruct::<String, ISize, FSize>

// create unit struct
UnitStruct()
```
