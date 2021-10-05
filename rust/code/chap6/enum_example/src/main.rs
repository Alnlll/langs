// #[derive(Debug)]
// enum IpAddrKind {
//   V4,
//   V6,
// }
// #[derive(Debug)]
// struct IpAddr {
//   kind: IpAddrKind,
//   addr: String,
// }

#[derive(Debug)]
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String)
}

fn main() {
  // let exam: (u8, u8, u8, u8) = (127, 0, 0, 1);
  // println!("v4: {}.{}.{}.{}", exam.0, exam.1, exam.2, exam.3);

  let ip_addr_v4 = IpAddr::V4(127, 0, 0, 1);
  let ip_addr_v6 = IpAddr::V6(String::from("::1"));

  println!("v4: {:?}", ip_addr_v4);
  println!("v6: {:?}", ip_addr_v6);
}
