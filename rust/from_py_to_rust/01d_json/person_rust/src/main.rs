use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, Serialize)]
#[derive(Serialize, Deserialize, Debug)]
struct Person {
  name: String,
  age: usize,
  job: Option<String>,
  verified: bool,
  parents: Vec<String>,
}

fn main() {
  let bob = Person {
    name: "Bob".to_string(),
    age: 12,
    verified: true,
    job: None,
    parents: ["Alice".to_string(), "Eve".to_string()].to_vec(),
  };
  println!("{:?}", bob);

  let json = serde_json::to_string(&bob).unwrap();
  println!("{}", json);

  let bob_obj: Person = serde_json::from_str(&json).unwrap();
  println!("{:?}", bob_obj);
}
