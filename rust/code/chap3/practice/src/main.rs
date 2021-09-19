fn f2c_temperature(f: f64) -> f64 {
  let c = (f - 32.0) / 1.8;
  c
}

fn fibonacci(n: u32) -> u32 {
  if 3 > n {
    println!("Error number < 3.");
    1
  } else {
    let mut a1 = 1;
    let mut a2 = 2;
    let mut an = 0;
    for _ in {3..n+1} {
      an = a1 + a2;
      a1 = a2;
      a2 = an;
    }
    an
  }

}


fn main() {
  let f = 100.0;
  println!("F = 100.f -> C = {}", f2c_temperature(f));

  let n = 10;
  println!("fibo = {}", fibonacci(n));
}
