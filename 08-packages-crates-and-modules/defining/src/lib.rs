mod front_of_house {
  pub mod hosting {
      pub fn add_to_waitlist() {
        println!("add to waitlist");
      }

      fn seat_at_table() {
        println!("seat at table");
      }
  }

  pub mod serving {
      pub fn take_order() {}

      pub fn serve_order() {}

      pub fn take_payment() {}
  }
}

pub fn eat_act_restaurant() {
  // Absolute path
  crate::front_of_house::hosting::add_to_waitlist();

  // Relative path
  front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {
  println!("deliver order")
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

mod back_of_house_02 {
  pub struct Breakfast {
      pub toast: String,
      seasonal_fruit: String,
  }

  impl Breakfast {
      pub fn summer(toast: &str) -> Breakfast {
          Breakfast {
              toast: String::from(toast),
              seasonal_fruit: String::from("peaches"),
          }
      }
  }
}

pub fn eat_at_restaurant() {
  let mut meal = back_of_house_02::Breakfast::summer("Rye");

  meal.toast = String::from("toast");
  println!("I'd like {} toast please", meal.toast);
}

mod back_of_house_03 {
  #[derive(Debug)]

  pub enum Appetizer {
      Soup,
      Salad,
  }
}


pub fn eat_at_restaurant_02() {
  let order1 = back_of_house_03::Appetizer::Soup;
  let order2 = back_of_house_03::Appetizer::Salad;
  
  println!("order1 is {:#?}, order2 is {:#?}", order1, order2);
}
