// mod front_of_house {
//   mod hosting {
//     fn add_to_waitlist() {}
//     fn seat_at_table() {}
//   }

//   mod serving {
//     fn take_order() {}
//     fn serve_order() {}
//     fn take_payment() {}
//   }
// }

// mod front_of_house {
//   pub mod hosting {
//     pub fn add_to_waitlist() {}
//   }
// }

// pub fn eat_at_restaurant() {
//   // absolute path
//   crate::front_of_house::hosting::add_to_waitlist();

//   // relative path
//   front_of_house::hosting::add_to_waitlist();
// }

// fn serve_order() {}

// mod back_of_house {
//   fn fix_incorrect_order() {
//     cook_order();
//     super::serve_order();
//   }

//   fn cook_order() {}
// }

// mod back_of_house {
//   pub struct Breakfast {
//     pub toast: String,
//     seasonal: String,
//   }

//   impl Breakfast {
//     pub fn summer(toast: &str) -> Breakfast {
//       Breakfast {
//         toast: String::from(toast),
//         seasonal: String::from("peaches"),
//       }
//     }
//   }
// }

// pub fn eat_at_restaurant() {
//   let mut meal = back_of_house::Breakfast::summer("Rye");

//   meal.toast = String::from("Wheat");
//   println!("I'd like {} toast please", meal.toast);
// }

mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

// use crate::front_of_house::hosting;
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}

