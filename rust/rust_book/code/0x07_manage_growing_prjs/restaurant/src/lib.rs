mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
  }
}

fn deliver_order() {} // in parent
mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();
    super::deliver_order(); // find in parent
  }
  fn cook_order() {}

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

  pub enum Appetizer {
    Soup,
    Salad,
  }
}

pub fn eat_at_restaurant() {
  let mut meal = back_of_house::Breakfast::summer("Rye");
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // the below action won't work as the 'seasonal_fruit' was not made into public
  // meal.seasonal_fruit = String::from("blueberries");

  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;

  // // absolute path
  // // crate::front_of_house::hosting::add_to_waitlist();

  // // relative path
  //// front_of_house::hosting::add_to_waitlist();
}