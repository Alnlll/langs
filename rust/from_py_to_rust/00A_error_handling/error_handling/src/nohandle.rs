fn multiply(first_str: &str, second_str: &str) -> i32 {
  let first = first_str.parse::<i32>().unwrap();
  let second = second_str.parse::<i32>().unwrap();

  first * second
}

fn main() {
  let twenty = multiply("2", "10");
  println!("double 10 is {}", twenty);

  let tt = multiply("t", "2");
  println!("double t is {}", tt);
}
