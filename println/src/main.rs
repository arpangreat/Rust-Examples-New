#![feature(allow_internal_unstable)]
#[macro_export]
#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! printnew {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        std::io::_print(std::format_args_nl!($($arg)*));
    })
}

fn main() {
    let num: i32 = 90;
    printnew!("New");
    printnew!("{}", num);
}
