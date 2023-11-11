fn do_something(x: i32) -> i32 {
  fn do_something_else(y: i32) -> i32 {
    y + 1
  }
  do_something_else(x)
}

fn main() {
  println!("{}", do_something(1));

  // the type will be inferred as the first instance.
  let example_closure = |x| x;

  let s: String = example_closure(String::from("hello"));
  let n = example_closure(5.to_string());
  println!("{} {}", s, n);
  
  let x: Vec<_> = vec!{1, 2, 3};
  let equal_to_x = move |z| x == z;
  assert_eq!(equal_to_x(vec!{1, 2, 3}), true);

  let items: Vec<_> = vec![1, 2, 3, 4, 5];
  let plus_one = |x| x + 1;
  let added_one = items.iter().map(plus_one).collect::<Vec<_>>();
  let added_one_sum = added_one.iter().fold(0, |x, y| x + y);
  println!("{:?} sum: {:?}", added_one, added_one_sum);

  let fizz_buzz = |x| {
    if x % 15 == 0 {
      println!("FizzBuzz");
    } else if x % 5 == 0 {
      println!("Fizz");
    } else if x % 3 == 0 {
      println!("Buzz");
    } else {
      println!("{}", x);
    }
  };

  for i in 1..16 {
    fizz_buzz(i);
  }
  println!("---------------");
  (1..16).into_iter().for_each(fizz_buzz);
}
