fn main() {
  let number: i32 = 100;

  if 10 < number {
    println!("number {} > 10", number);
  } else {
    println!("number {} <= 10", number);
  }

  if 10 < number {
    println!("number {} > 10", number);
  } else if 10 == number {
    println!("number {} = 10", number);
  } else {
    println!("number {} < 10", number);
  }

  let condition = true;
  let x = if condition {5} else {10};
  println!("x = {}", x);

  // loop {
  //   println!("Again!")
  // }

  let mut i = 10;
  while 0 != i {
    i -= 1;
    println!("i = {}", i);
  }
  println!("Loop FINISHED.");

  // container iter for
  let a = [10, 20, 30, 40, 50, 60, 70, 80];
  for elem in a.iter() {
    println!("val is {}.", elem);
  }

  for i in {1..4}.rev() {
    println!("i = {}", i);
  }
}
