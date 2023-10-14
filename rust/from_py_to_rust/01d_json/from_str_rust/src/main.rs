// #![feature(type_ascription)]
// #![feature(type_ascription)]
use serde_json::json;

fn main() {
  // from str
  let o_json = r#"["foo", {"bar": ["bax", null, 1.0, 2]}]"#;
  let o: serde_json::Value = serde_json::from_str(o_json).unwrap();
  println!("{:?}", &o);

  let o = json!(["foo", {"bar": ("bax", 0, 1.0, 2)}]);
  println!("{:?}", &o);

  // to string
  let s = &o.to_string();
  println!("{}", &s);
}
