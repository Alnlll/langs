fn str_type() {
  let mut s = String::from("hello");
  s.push_str(", world!");
  println!("s = {}", s);
}

fn main() {
  {
    // s is not valid here, it's not yet declared
    let s = "hello"; // s is valid from here
    // do the staff with s
    println!("s = {}", s);
  } // the scope is over, s is no longer valid
  // println!("s = {}", s);

  str_type();

  // move
  // scalar
  {
    let x = 5;
    let x1 = x;
    println!("x = {}, x1 = {}", x, x1);
  }

  // String
  {
    let s = String::from("hello");
    let s1 = s;
    // println!("s = {}, s1 = {}", s, s1);
    println!("s1 = {}", s1);
  }


  // clone
  {
    let s = String::from("hello");
    let s1 = s.clone();
    println!("s = {}, s1 = {}", s, s1);
  }

  println!("\n-------functions-------");
  // functions passing values
  {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("s = {}", s);

    let x = 5;
    make_copy(x);
    println!("x = {}", x);
  }

  // functions return values
  {
    let s = gives_ownership();
    println!("s = {}", s);

    let s = takes_and_give_back(s);
    println!("s = {}", s);
  }

  println!("\n-------reference-------");
  {
    let s = String::from("reference");
    let len = cal_string_len(&s);
    println!("s {} len is {}", s, len);
  }
  {
    let mut s = String::from("reference");
    change_str(&mut s);
    println!("s = {}", s);
  }
  {
    let mut s = String::from("reference");
    let r1 = &mut s;
    // let r2 = &mut s;
    // println!("r1 = {}, r2 = {}", r1, r2);
  }
  {
    // let r = dangle();
  }

  // slice
  println!("\n-------slice-------");
  let s = String::from("hello world");
  let word = first_word(&s[..]);
  // s.clear();
  println!("the first word is {}", word);

  let s1 = "hello world";
  let word1 = first_word(&s1[..]);
  println!("the first word is {}", word1);

}

fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if b' ' == item {
      return &s[..i];
    }
  }

  &s[..]
}

// fn dangle() -> &String {
//   let s = String::from("dangle");
//   &s
// }

fn change_str(s: &mut String) {
  s.push_str(", added.");
}
fn cal_string_len(s: &String) -> usize {
  s.len()
}

fn takes_ownership(s: String) {
  println!("s = {}", s);
}
fn make_copy(i: i32) {
  println!("i = {}", i);
}

fn gives_ownership() -> String {
  let s = String::from("gives_ownership");
  s
}
fn takes_and_give_back(s: String) -> String {
  println!("got s = {}", s);
  s
}
