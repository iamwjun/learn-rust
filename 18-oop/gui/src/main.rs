use gui::{Draw, Button, Screen};

struct SelectBox {
  width: u32,
  heigth: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
      // code to actually draw a select box
  }
}

fn main() {
    let screen = Screen {
		components: vec![
			Box::new(SelectBox {
			width: 75,
			heigth: 10,
			options: vec![
				String::from("Yes"),
				String::from("Maybe"),
				String::from("No"),
			]
			}),
			Box::new(Button {
			width:50,
			heigth: 10,
			label: String::from("OK"),
			})
		]
    };

  	let screen = Screen {
      	components: vec![Box::new(String::from("Hi"))],
  	};

  	screen.run();
}