# Functions and Closures

functions and Closures implement the Fn trait

```
pub trait Fn <Args> {
	type Output

	fn call(args: Args) -> Self::Output
}
```

### Define a Function

```
pub fn add (a, b) a + b

// add some types
pub fn add(a: ISize, b: ISize) -> ISize
	a + b

// make it generic
pub fn add <T> (a: T, b: T) -> T
	where T: core::ops::TAdd
	a + b
```

### Function overloading

```
pub fn hello_world
	() -> String
		os::println("Hello, unknown person!")

	(number: ISize) -> String
		os::println("Your Numer is " + number.to_string() + "!")

	<T> (name: T) -> Result<()>
		where T: TToString
		os::println("Hello, " + name.to_string() + "!")

hello_world() // prints "Hello, unknown person!"
hello_world(10) // prints "Your Number is 10!"
hello_world("Bobby") // prints "Hello, Bobby!"
```

### Function matching

```
// this will be compiled to a match function
pub fn factorial
	(0: ISize) -> ISize 1
	(x: ISize) -> ISIze
		x * fac(x - 1)

// internal
pub fn factorial(x: ISize) -> ISize
	match x {
		0 -> 1
		x -> x * fac(x - 1)
	}

factorial(5) // 120
```

### Closures

closures capture variables from the outer scope

```
let add = fn (a, b) a + b

// add some types
let add = (a: ISize, b: ISize) -> ISize
	a + b

let add = <T> (a: T, b: T) -> T
	where T: core::ops::TAdd ->
	a + b
```

example with maps

```
// MAP is HashMap::<Keyword, Keyword>
let MAP = #{ :key -> :value }

// map_get = fn (Keyword) -> HashMap::<Keyword, Keyword>
let map_get = key -> MAP.get(key)

map_get(:key) // :value

let map_set = (key, value) -> MAP.set(key, value)
let new_map = map_set(:key, :new_value)
```

iters

```
[0, 1, 2, 3].iter().for_each(fn (item, _index)
	os::println("{}", item)
)
```
