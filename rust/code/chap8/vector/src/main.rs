fn main() {
  // create a new vector
  {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    println!("v = {:?}", v);
  }

  // updating Vector
  {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v = {:?}", v);
  }

  // Reading elements
  {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    match v.get(2) {
      Some(third) => println!("The third elem is {}", third),
      None => println!("There is no the third elem.")
    }
  }

  // iterating elements
  {
    let v = vec![1, 2, 3, 4, 5];
    for e in &v {
      println!("{}", e);
    }
  }

  // Enum to save multiple type
  {
    enum SpreadsheetCell {
      Int(i32),
      Float(f32),
      Text(String),
    }

    let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Float(10.13),
      SpreadsheetCell::Text(String::from("blue"))
    ];
  }
}
