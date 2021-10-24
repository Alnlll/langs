fn main() {
  // new
  {
    let mut s = String::new();
    println!("{}", s);
  }

  // string literal
  {
    let s = "initial contents".to_string();
    println!("{}", s);
    let s = String::from("initial contents - from");
    println!("{}", s);
  }

  // UTF-8
  {
    let hello0 = String::from("السلام عليكم");
    println!("{}", hello0);

    let hello1 = String::from("你好！");
    println!("{}", hello1);
  }

  // updating a String
  {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("s = {}", s);

    let mut s = String::from("Hello");
    let s1 = ", world!";
    s.push_str(s1);
    println!("s = {}", s);

    let mut s = String::from("lo");
    s.push('l');
    println!("s = {}", s);
  }
  {
    let s1 = String::from("Hello,");
    let s2 = String::from(" World!");
    let s3 = s1 + &s2;
    println!("s = {}", s3);
    // println!("s = {}", s1);
  }
  {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s = {}", s);
    println!("s = {}", s1);
  }

  // slice
  {
    let s = String::from("Здравствуйте");
    let s1 = &s[0..4];
    println!("slice = {}.", s1);
  }

  // iterating
  {
    let s = String::from("नमस्ते");
    for c in s.chars() {
      println!("{}", c);
    }
    for b in s.bytes() {
      println!("{}", b);
    }
  }

}
