use std::error;
use std::fmt;

// define a enum type, T is the success ret type, E is the error ret type
// Box<dyn error::Error> points to a trait "error::Error"
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;


// define the error : "EmptyVec"
#[derive(Debug)]
struct EmptyVec;

// define how to display error
impl fmt::Display for EmptyVec {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Empty vector!!!")
  }
}

// empty trait
impl error::Error for EmptyVec {}

fn double_list(vec: Vec<&str>) -> Result<Vec<i32>> {
  let _ = vec.first().ok_or(Box::new(EmptyVec))?;
  let mut res = Vec::new();
  for num in vec {
    let parsed = num.parse::<i32>()?;
    res.push(parsed * 2)
  }
  return Ok(res);
}

fn print(result: Result<Vec<i32>>) {
  match result {
    Ok(vec) => println!("success, double vec is {:?}", vec),
    Err(e) => println!("error, {}", e)
  }
}

fn main() {
  let numbers = vec!{"23", "33", "43"};
  let empty = vec!{};
  let tofu = vec!{"tofu", "33", "43"};

  print(double_list(numbers));
  print(double_list(empty));
  print(double_list(tofu));
}