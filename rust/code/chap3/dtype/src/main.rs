fn main() {
  let c = 'z';
  let z = 'ℤ';
  let heart_eyed_cat: char = '😻';

  println!("{} {} {} ", c, z, heart_eyed_cat);

  let tup: (char, i32, f64) = ('Z', 30, 34.56);
  println!("name: {}, age: {}, salary: {}", tup.0, tup.1, tup.2);


  let a = [1, 2, 3, 4, 5];
//   let first = a[0];
//   let second = a[1];
  println!("a[0] = {}", a[0]);

}
