# Traits and Lifetimes

## Traits
The `Trait` keyword allows us to define interfaces for other struct and classes. This allows us to group similar functionlity together and use polymorphism in our function calls.

To implement a trait we define it using the `pub trait <Name>` following by `impl <Name> for <Struct>`

Traits can also be used in function arguments lke so:
```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

A small nuance is if we want two items to be the same AND implement the trait then we would need to use a longer form to define the function i.e.,

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
// pub fn notify(item1: &impl Summary, item2: &impl Summary)  // This would allow any two types so long as they implement Summary
```

We can specify multiple traits by using the `+` syntax
```rust
pub fn notify(item: &(impl Summary + Display)) {
```
With these two trait bounds the item can use `{}` to format

We can also define trait bounds in a 'easier to read' format like so 

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

Does this compile? Both News Article and Tweet implment Summary
```rust

fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

No it doesnt! that becuase of how impl Trait is implemented in the comiler.

What is a blanket trait? \
A blanked trait defines a Trait on any type T that has type X i.e., 
```rust
imp <T : Display> ToString for T {
    // Impleements Trait ToString for all types T that has the trait Display
    fn to_string(&Self) -> String  {
        // default implementation
    }
} 
```
Rust moves the type and trait checking to compile time so we cannot call function that havn't been implemented for that type in runtime

## Validating References with Lifetimes

