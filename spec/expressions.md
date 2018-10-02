# Expressions

there is no return keyword, the last expressions is always returned

### Let

```
// Type can be inferred
let name: Type = Expr
// x is type F32
let x = 1.0_F32
```

```
let expressions return themselves
so let x = let y = 10
x = 10 and y = 10
```

#### Destructuring

```
// let is a match assignment so...
let #{x: x, y: y}: Vec2::<ISize> = Vec2 { x: 1, y: 5 }
let #{x: x, y: y} = Vec2 { x: 1, y: 5 }
let Vec2 {x: x, y: y} = Vec2 { x: 1, y: 5 }

let (x, y, z): (String, ISize, FSize) = ("Hello", -10, 2.0)
let (x, y, z) = ("Hello", -10, 2.0)
```

### match

match is an expression

```
match Expr { MatchCase -> Expr ... }

match x {
    0 -> os::println("Zero")
    1 -> os::println("One")
    // catch all
    _ -> os::println("A Number")
}

// where person is the struct Person { age: USize, name: String }
match person {
    Person { name: "Bob" } -> os::println("Hello, Bob!")
    // reassign age to personAge, and capture the name field
    Person { age: person_age, name: person_name } -> {
        os::println("I don't know you {}, you are {} years old", person_name, person_age)
    }
}
```

### if

if is an expression, else is required

```
if Expr { Expr } else { Expr }
if Expr { Expr } else if Expr { Expr } else if { Expr } else { Expr }

if variable {
    os::println("if branch")
} else {
    os::println("else branch")
}

if variable {
    os::println("if branch")
} else if other_variable {
    os::println("if else branch")
} else {
    os::println("else branch")
}
```
