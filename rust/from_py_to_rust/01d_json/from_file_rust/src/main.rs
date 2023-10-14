use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
struct ModelConfig {
  root_dir: String,
    model_name: String,
    input_nodes: HashMap<String, Vec<u32>>,
    output_nodes: Vec<Vec<String>>,
    cmp_output_nodes: Vec<Vec<String>>,
    quant: bool,
    quant_override: String,
    input_list: String,
    golden_list: String,
    weigth_bw: u32,
    act_bw: u32,
    bias_bw: u32,
    per_channel: bool,
    post_quant: bool,
    algorithms: String,
    param_quantizer: String,
    act_quantizer: String,
    cache_builder: String,
    debug: bool,
    convert_debug: bool,
    runtime: String,
    runner: String,
    perf_level: String,
    output_data_type: String,
    shared_buffer: bool,
    skip: bool,
}

fn read_file(path: String) -> String {
  let path = Path::new(&path);
  println!("config path is : {}", path.display());

  // open file
  let mut file = match File::open(&path) {
    Err(why) => panic!("couldn't open {} : {}", path.display(), why),
    Ok(file) => file,
  };

  // read the file contents into a string.
  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {} : {}", path.display(), why),
    Ok(_) => println!("{} was read.", path.display()),
  }

  return s;
}

fn main() {
  let content = read_file("./config.json".to_string());
  println!("content:\n {}", content);

  let json: serde_json::Value = serde_json::from_str(&content).unwrap();
  println!("json:\n {}", &json);
  println!("json:\n {}", json["inv3_demo"]);

  // let config: Config = serde_json::from_str(&content).unwrap();
  // println!("{:?}", config);
  // let result: serde_json::Result<HashMap<String, ModelConfig>> = serde_json::from_str(&content);
  // match result {
  //   Ok(json_config) => {
  //     for (k,v) in json_config {
  //       println!("{} : {:?}", k, v);
  //     }
  //   }
  //   Err(e) => {
  //     println!("Failed to parse json: {}", e);
  //   }
  // }
  let config_json : HashMap<String, ModelConfig> = serde_json::from_str(&content).unwrap();
  for (k,v) in config_json {
    println!("{} : {:?}", k, v);
  }
}
