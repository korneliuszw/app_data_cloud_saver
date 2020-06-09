#[macro_export]
macro_rules! some_or_continue {
    ($val : expr) => {
        if $val.is_none() {
            continue;
        }
    }
}