use proc_macro::TokenStream;

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x * $x);
            )*
            temp_vec
        }
    };
}

pub trait HelloMacro {
    fn hello_macro();
}
