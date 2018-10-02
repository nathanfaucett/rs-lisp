# Modules

A module is a collection of items: functions, structs, traits, and even other modules.

### File hierarchy

```bash
$ tree .
.
|-- a
|   |-- mod.l
|   |-- private.l
|   `-- public.l
|-- b.l
`-- lib.l
```

In lib.l

```
mod a
mod b

// a::public::constant is accessiable
// a::private::constant is not
```

In a/mod.l

```
mod private
pub mod public
```

In a/private.l

```
pub static constant: String = "private a"
```

In a/public.l

```
pub static constant: String = "public a"
```

In b.l

```
pub static constant: String = "b"
```

### Modules in Modules

```
pub mod some_name {
	static constant: String = "some_constant"
}
```

### package, super, and self

The package, super, and self keywords can be used in the path to find paths relative to the current one

```
// reference to module math in this module
self::math
// reference to module helpers in the parent module
super::helpers
// reference to module utils in the package root module
package::utils
// use `::` to reference to root global namespace
::some_package::path
```

### Visibility

By default, the items in a module have private visibility, but this can be overridden with the pub keyword. Only the public items of a module can be accessed from outside the module scope.

```
// public to module and children
pub
// public to package
pub::(package)
// public to module path and children
pub::(super::some::path)
```

### Use

```
// makes TAdd avialable in scope now
use core::ops::TAdd

// makes TAdd and TSub avialable in scope now
use core::ops::(TAdd, TSub)

// makes core, ops, TAdd, and TSub avialable in scope now
use core::(self, ops::(self, TAdd, TSub))
```
