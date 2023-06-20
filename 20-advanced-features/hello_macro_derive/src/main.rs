#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x + 1);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = vec![1, 2, 3];

    println!("v is {:?}", v)
}