fn main (){
  // 入力した数字を文字列として取得
  let mut num_vec = String::new();
  std::io::stdin().read_line(&mut num_vec).unwrap();
  num_vec.trim_end().to_owned();
  num_vec = num_vec.replace("\r\n", ""); // 改行コードを削除

  // 文字列を分割して文字列のベクターに変換
  let vec_str: Vec<&str> = num_vec.split(' ').collect();
  let mut vec: Vec<i32> = Vec::new();

  // 文字列のベクターを数字のベクターに変換
  for v in vec_str {
    let n :i32 = v.parse::<i32>().unwrap();
    vec.push(n);
  }

  // ソート
  vec.sort();

  // 3番目に小さい数字を表示
  println!("{}", vec[2]);
}

