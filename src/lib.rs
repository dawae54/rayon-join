#[macro_export]
macro_rules! join {
    ($f1:expr, $f2:expr) => {
        rayon::join($f1, $f2)
    };
    ($f1:expr, $f2:expr, $($rest:expr),+) => {
        rayon::join($f1, || join!($f2, $($rest),+))
    };

}
