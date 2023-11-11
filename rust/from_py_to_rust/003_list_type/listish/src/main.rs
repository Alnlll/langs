fn reverse(pair: (i32, bool)) -> (bool, i32) {
  let (integer, boolean) = pair;
  (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
  println!("\n-------------Tuple-------------");
  let many_type = (1u8, 2u16, 'o', true);
  println!("first val: {}", many_type.0);
  println!("first val: {}", many_type.1);

  let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
  println!("tuple of tuples: {:?}", tuple_of_tuples);

  let pair = (1, true);
  println!("pair: {:?}", pair);
  println!("the reversed pair: {:?}", reverse(pair));

  println!("one element tuple: {:?}", (5u32,));
  println!("just on integer: {:?}", (5u32));

  let tuple = (1, "hello", 4.5, true, 9527u16);
  let (a, b, c, d, e) = tuple;
  println!("a={:?} b={:?} c={:?} d={:?} e={:?}", a,b,c,d,e);

  let matrix: Matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{:?}", matrix);

  println!("\n-------------Array-------------");
  // no mem allocation way
  let xs = [1, 2, 3, 4, 5];
  
  let ys: [i32; 500] = [0; 500];
  let _ys = [0_u64; 500];

  println!("first val of the array: {}", xs[0]);
  println!("second val of the array: {}", xs[1]);

  println!("xs size: {}", xs.len());
  println!("ys size: {}", ys.len());

  println!("borrow section from an array as a slice: {:?}", &xs[1..3]);

  println!("xs[4] = {}", xs[4]);

  println!("\n-------------Vector-------------");
  let mut xs = vec![1_i32, 2, 3, 4];
  println!("xs = {:?}", xs);
  xs.push(5);
  println!("xs (pushed 5) = {:?}", xs);

  println!("xs size = {}", xs.len());
  println!("xs[-1] = {}", xs[xs.len()-1]);
  println!("pop xs last elem: {}", xs.pop().unwrap());
  println!("xs size(poped) = {}", xs.len());
  println!("xs[-1] = {}", xs[xs.len()-1]);

  println!("foreach the vector:");
  for x in xs {
    // borrow and not use into_iter()
    println!("x = {}", x);
  }
  // enumrate
  for (i, x) in xs.iter().enumerate() {
    println!("x[{}] = {}", i, x);
  }


}
