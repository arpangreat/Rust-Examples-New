macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e;

            println!("{} = {}", stringify!{$e}, val);
        }
    };

    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es), +}
    }};
}
fn main() {
    calculate! {
        eval 1 + 2,
        eval (2*3) / (3*4),
        eval (6*4) + (6*7)
    }
}
