#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn coin_2_cent(coin: Coin) -> u32 {
  match coin {
    Coin::Penny => {
      println!("it is a penny.");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}", state);
      25
    },
  }
}

fn main() {
  let cent_penny = coin_2_cent(Coin::Penny);
  println!("penny: {}", cent_penny);
  let cent_quarter = coin_2_cent(Coin::Quarter(UsState::Alaska));
  println!("penny: {}", cent_quarter);

  let mut count = 0;
  let coin = Coin::Quarter(UsState::Alaska);
  if let Coin::Quarter(state) = coin {
    println!("State quarter is from {:?}", state);
  } else {
    count += 1;
  }
  println!("count = {}", count);
}
