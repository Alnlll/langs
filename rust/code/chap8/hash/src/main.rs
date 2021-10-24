use std::collections::HashMap;

fn main() {
  // new hash map
  {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    println!("scores = {:?}", scores);
  }

  {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 50];

    let mut scores: HashMap<_,_> =
      teams.into_iter().zip(init_scores.into_iter()).collect();

    println!("scores = {:?}", scores);
  }

  // Accessing
  {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score = {:?}", score);
  }

  {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    for (k, v) in &scores {
      println!("{} = {}", k, v);
    }
  }

  // updating values
  println!("\n----updating values.");
  {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 30);
    println!("scores = {:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores = {:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
    }
    println!("map = {:?}", map);
  }

}
