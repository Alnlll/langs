#[derive(Debug)]
struct User {
  name: String,
  email: String,
  sign_in_count: u64,
  activate: bool
}

fn main() {
  // basic
  println!("\n---------basic:");
  let user1 = User {
    email: String::from("www.example.com"),
    name: String::from("Rust"),
    activate: true,
    sign_in_count: 10,
  };

  println!("use1 : {:?}", user1);

  // access element
  println!("\n---------access element:");
  println!("user.name: {}\nuser.email: {}\nuser.sign_in_count: {}\nuser.activate: {}",
    user1.name, user1.email, user1.sign_in_count, user1.activate);

  // modify element
  println!("\n---------modify element:");
  let mut user2 = User {
    email: String::from("www.example.com"),
    name: String::from("Rust mut"),
    activate: true,
    sign_in_count: 10,
  };
  println!("user2: {:?}", user2);
  user2.name = String::from("John");
  println!("user2: {:?}", user2);

  // from another structure instance
  println!("\n---------from another:");
  let user3 = User {
    name: String::from("user3"),
    email: String::from("www.user3.com"),
    ..user1
  };
  println!("user3: {:?}", user3);
}
