macro_rules! debug {
    ($($val:expr);*) => {
        $(
            eprintln!(
                "[{}:{}] {} = {:?}",
                file!(),
                line!(),
                stringify!($val),
                $val
            );
        )*
    };
}

fn main() {
    debug!(1; 2);
}
