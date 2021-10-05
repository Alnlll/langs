#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle { width: size, height: size}
  }
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let rect = Rectangle {
    width: 30, 
    height: 40, 
  };
  let rect1 = Rectangle { width: 40, height: 50 };

  println!("rect: {:?}", rect);
  println!("area is {}", rect.area());

  println!("rect: {:?}, rect1: {:?}", rect, rect1);
  println!("can hold: {}", rect.can_hold(&rect1));

  let s = Rectangle::square(32);
  println!("s: {:?}", s);
  println!("area is {}", s.area());
}
