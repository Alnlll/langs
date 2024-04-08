use crate::garden::vegetables::Asparagus; // import Asparagus from -->garden-->vegetables

pub mod garden; // public garden mode --> garden.rs or garden/mod.rs

fn main() {
  let plant = Asparagus {};
  println!("I'm growing {:?}", plant);
}
