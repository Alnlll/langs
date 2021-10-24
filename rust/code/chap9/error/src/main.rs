fn main() {
  // panic!("crash and burn");
  // let v = vec![1, 2, 3, 4, 5];
  // v[99];

  // Result<T, E>
  // {
  //   use std::fs::File;
  //   let f = match File::open("hello.txt") {
  //     Ok(file) => file,
  //     Err(error) => panic!("failed to open file: {:?}", error),
  //   };
  // }

  // Error kind
  // {
  //   use std::fs::File;
  //   use std::io::ErrorKind;

  //   let f = match File::open("hello.txt") {
  //     Ok(file) => file,
  //     Err(error) => match error.kind() {
  //       ErrorKind::NotFound => match File::create("hello.txt") {
  //         Ok(fc) => fc,
  //         Err(e) => panic!("failed to create file: {:?}", e),
  //       }
  //       other_error => {
  //         panic!("failed to open file: {:?}", other_error)
  //       }
  //     }
  //   };
  // }

  // unwrap and expect
  // {
  //   use std::fs::File;
  //   // let f = File::open("hello.txt").unwrap();
  //   // println!("f = {:?}", f);
  //   let f = File::open("hello.txt").expect("failed to open hello.txt");
  // }

  // shortcut for propagating errors: ?
  {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
      let mut f = File::open("hello.txt")?;
      let mut s = String::new();
      f.read_to_string(&mut s)?;
      Ok(s)
    }

    println!("{:?}", read_username_from_file());
  }
}
