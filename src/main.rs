use std::result;

fn split_string(s: String, delimiter: char, field: usize) -> String {
  let parts: Vec<&str> = s.split(delimiter).collect();
  let result = parts.get(field);
  // This will notcompile
  result.expect("oops! something went wrong").to_string()
}

fn main () {
  let chunk = split_string("hello,world".to_string(), ',', 1);
  println!("Split string: {}", chunk);
}

// The return value()