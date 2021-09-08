use std::mem;

fn type_of<T>(_: T) -> String{
  let a = std::any::type_name::<T>();
  return a.to_string();
}

fn main (){

  let mut vec: Vec<i32> = Vec::new();

  // 3 x 3 の数字の入力を受付け、ベクタに変換
  for i in 0..3 {
    // 入力した数字を文字列として取得
    let mut num_vec = String::new();
    std::io::stdin().read_line(&mut num_vec).unwrap();
    num_vec.trim_end().to_owned();
    num_vec = num_vec.replace("\r\n", ""); // 改行コードを削除

    // 文字列を分割して文字列のベクターに変換
    let vec_str: Vec<&str> = num_vec.split(' ').collect();
    
    // 文字列のベクターを数字のベクターに変換
    for v in vec_str {
      let n :i32 = v.parse::<i32>().unwrap();
      vec.push(n);
    }
  }

  // 8個まで数字を受け付け、先ほどのベクタの数字に一致するか判定する
  let times :i32 = 8;

  for i in 0..times {
    let mut num_str = String::new();
    std::io::stdin().read_line(&mut num_str).unwrap();
    num_str.trim_end().to_owned();
    let num: i32 = num_str.trim().parse().unwrap();

    // ベクター全体をチェックして入力した数字と一致しているベクタの要素を 999 に置換
    for i in 0..vec.len() {
      if vec[i] == num {
        let got = std::mem::replace(&mut vec[i], 999);
      }
    }
  }

  // ビンゴしているかどうかチェック
  let mut is_bingo = false;
  for i in 0..vec.len() {
    if vec[0] == 999 && vec[3] == 999 && vec[6] == 999 {
      is_bingo = true;
      break;
    } else if vec[1] == 999 && vec[4] == 999 && vec[7] == 999 {
      is_bingo = true;
      break;
    } else if vec[2] == 999 && vec[5] == 999 && vec[8] == 999 {
      is_bingo = true;
      break;
    } else if vec[0] == 999 && vec[1] == 999 && vec[2] == 999 {
      is_bingo = true;
      break;
    } else if vec[3] == 999 && vec[4] == 999 && vec[5] == 999 {
      is_bingo = true;
      break;
    } else if vec[6] == 999 && vec[7] == 999 && vec[8] == 999 {
      is_bingo = true;
      break;
    } else if vec[0] == 999 && vec[4] == 999 && vec[8] == 999 {
      is_bingo = true;
      break;
    } else if vec[2] == 999 && vec[4] == 999 && vec[6] == 999 {
      is_bingo = true;
      break;
    } 
  }
  if is_bingo {
    println!("YES");
  } else {
    println!("NO");
  }
}