#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    ($($val:expr),* ) => {
        println!("Look at this one more macro!:"); 
        $(
            println!("{}", $val);
        )*
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
    my_macro!(1111, 2222, 3333, 4444, 5555, 6666, 7777);
}
