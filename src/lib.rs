#[macro_export]
/// Executes multiple closures in parallel using `rayon::join`.
/// ## Usage
/// You can pass any number of closures:
/// ```
/// rayon_join::join!(|| { /* task 1 */ }, || { /* task 2 */ }, || { /* task 3 */ });
/// ```
/// You can also pass functions:
/// ```
/// fn task1() { /* ... */ }
/// fn task2() { /* ... */ }
/// fn task3() { /* ... */ }
/// rayon_join::join!(task1, task2, task3);
/// ```
/// If you need to pass arguments to the functions, use a closure:
/// ```
/// fn task_with_arg(x: i32) { /* ... */ }
/// rayon_join::join!(|| task_with_arg(42), || task_with_arg(7));
/// ```
macro_rules! join {
    ($f1:expr, $f2:expr) => {
        rayon::join($f1, $f2)
    };
    ($f1:expr, $f2:expr, $($rest:expr),+) => {
        rayon::join($f1, || join!($f2, $($rest),+))
    };

}
