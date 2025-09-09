# rayon-join
Simple macro to call join from rayon with more than 2 arguments.

## Installation
```sh
cargo add rayon-join
```

## Usage
You can pass any number of closures:
```rust
 rayon_join::join!(|| { /* task 1 */ }, || { /* task 2 */ }, || { /* task 3 */ });
```

You can also pass functions:
```rust
fn task1() { /* ... */ }
fn task2() { /* ... */ }
fn task3() { /* ... */ }
rayon_join::join!(task1, task2, task3);
```

If you need to pass arguments to the functions, use a closure:
```rust
fn task_with_arg(x: i32) { /* ... */ }
rayon_join::join!(|| task_with_arg(42), || task_with_arg(7));
```
