fn largest_i32(list: &[i32]) -> i32 {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn largest_char(list: &[char]) -> char {
  let mut largest = list[0];

  for &item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

// fn largest<T>(list: &[T]) -> T {
//   let mut largest = list[0];

//   for &item in list {
//     if item > largest {
//       largest = item;
//     }
//   }

//   largest
// }

struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

impl Point<f32> {
  fn dis_from_origin(&self) -> f32 {
    return (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

fn main() {
  // let number_list = vec![34, 50, 25, 100, 65];

  // let largest_num = largest_i32(&number_list);
  // println!("largest_num = {}", largest_num);

  // let char_list = vec!['y', 'm', 'a', 'q'];
  // let largest_char = largest_char(&char_list);
  // println!("largest_char = {}", largest_char);


  // println!("largest_num = {}", largest(&number_list));

  // generic type struct
  {
    let integer = Point {x: 5, y:10 };
    let float = Point {x: 1.0, y: 4.0};

    println!("integer: x= {}, y = {}", integer.x, integer.y);
    println!("float: x= {}, y = {}", float.x, float.y);
    println!("integer: x = {}", integer.x());

    println!("Distance from origin: {}", float.dis_from_origin());
  }

}
