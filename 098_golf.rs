fn main() {
  // 入力受付 1
  let mut num_str = String::new();
  std::io::stdin().read_line(&mut num_str).unwrap();
  num_str.trim_end().to_owned();  
  let num :i32 = num_str.trim().parse().unwrap();

  // 入力受付 2 ベクタ
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

  let mut flag :bool = false;
  // ベクタ間の中でnumで割った余りを求め、余りがゼロのものがあれば OK を出力
  for v in vec[0]..=vec[1] {
    if v % num == 0 {
      flag = true;
      break;
    }
  }

  if flag {
    println!("OK");
  } else {
    println!("NG");
  }
}