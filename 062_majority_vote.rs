use std::collections::HashMap;

fn main (){

  // 入力受付
  let mut vote = String::new();
  std::io::stdin().read_line(&mut vote).unwrap();
  vote.trim_end().to_owned();

  let mut na = 0;
  let mut nb = 0;
  let mut nc = 0;

  for c in vote.chars() {
      if c.to_string() == "a".to_string() {
        na += 1;
      } else if c.to_string() == "b".to_string() {
        nb += 1;
      } else if c.to_string() == "c".to_string() {
        nc += 1;
      }
  }

  let mut map = HashMap::new();
  map.insert("a", na);
  map.insert("b", nb);
  map.insert("c", nc);

  let mut max_value : i32 = 0;
  let mut max_key : String = "".to_string();

  for (key, value) in &map {
    if max_value < *value {
      max_value = *value;
      max_key = key.to_string();
    }
  }
  println!("{}", max_key);
}