# Basic Error handling 
https://doc.rust-lang.org/book/ch09-00-error-handling.html

## Panic!
### Unrecoverable errors with `panic!`
panic! macro is used to stop the program running and begin cleaning up and data from the stack, this is called unwinding. Unwinding can be time consuming hence we can abort immediatly by using `panic='abort'`

The following code panics due to a out of range exeption
```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```
To get a backtrace on windows `$env:RUST_BACKTRACE=1; cargo run` using the $env to set a windows variablle

## Handling errors with `Result`


``` rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Genearlly we can handle the result by using match statement however it is much cleaner to use closures for example when opening a file, that may fail due to many reasons such as file not found

```rust
fn main() {
    let greeting_file_result = File::open("somefile")
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => // ... etc
        }
    }
}

// Alternatively we should use closures to prevent nested math statement which can be hard to read

fn main() {
    let greeting_file = File::open("somefile").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {

        } else {
            panic!("Problem opening file!")
        }
    })
}
```
<br>
Expect vs Unwrap

Expect should be used over unwrap as it also gives context to the exception and can let us write our assumption - thus when debugging we can check those assumptions
<br>

We can us e the `?` operator to bubble up errors that is return an Error from a function. for example
```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

