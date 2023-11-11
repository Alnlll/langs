// use eyre::{eyre, Result};

// fn double_list(vec: Vec<&str>) -> Result<Vec<i32>> {
//     let _ = vec.first().ok_or_else(|| eyre!("No first item"))?;
//     let mut res = Vec::new();
//     for num in vec {
//       let parsed = num.parse::<i32>()?;
//       res.push(parsed * 2)
//     }
//     return Ok(res);
//   }

fn print(result: Result<Vec<i32>>) {
    match result {
        Ok(vec) => println!("{:?}", vec),
        Err(e) => println!("error {}", e),
    }
}

use eyre::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum DoubleError{
    #[error("no first item")]
    EmptyVec,
    #[error("invalid first item, error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

fn double_list(vec: Vec<&str>) -> Result<Vec<i32>> {
    let _ = vec.first().ok_or(DoubleError::EmptyVec)?;
    let mut res = Vec::new();
    for num in vec {
      let parsed = num.parse::<i32>().map_err(|e| DoubleError::Parse(e))?;
      res.push(parsed * 2)
    }
    return Ok(res);
  }

fn main() {
    let numbers = vec!{"23", "33", "43"};
    let empty = vec!{};
    let tofu = vec!{"tofu", "33", "43"};

    print(double_list(numbers));
    print(double_list(empty));
    print(double_list(tofu));
}
