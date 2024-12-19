# Ownership
Ownership is a fundamental concept in rust which allows for its memory safe guarentees

1. Every value has a single owner
2. The when an owner goes out of scope the value is dropped
3. Owner transfership (move) when an value is passed not as a reference or reassigned then ownership is moved and the original owner becomes invalid i.e.,
```rust
let a = String::from("hello");
let b = a;  // Ownership of the value is moved from `a` to `b`
// println!("{}", a);  // This would cause a compile-time error
// or
do_something_function(b)
```

4. Borrowing - Allows you to access the values of an owner witout taking owner ship (&T)
```rust
let s = String::from("hello");
let r1 = &s;  // Immutable borrow
let r2 = &s;  // Another immutable borrow
println!("{}", r1);  // Can use r1

```
5. There are non-mutable borrow (&T) and mutable borrow (mut &T). Only one part of the code can mutably borrow and no other parts can borrow at that time (immutably or mutably)
```rust
let mut s = String::from("hello");
let r = &mut s;  // Mutable borrow
r.push_str(", world");
println!("{}", r);  // Can use r after mutation
```