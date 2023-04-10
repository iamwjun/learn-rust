// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// 	println!("result");
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// 	let mut buffer = String::new();

//     io::stdin().read_line(&mut buffer)?;

//     Ok(buffer)
// }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            print!("add to waitlist");
        }
    }
}

pub use crate::front_of_house::hosting;

fn eat_at_restaurant() {
	hosting::add_to_waitlist();
}

mod customer {
	use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
