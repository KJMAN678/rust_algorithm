fn main() {
    // 入力受付
    let mut num_str = String::new();
    std::io::stdin().read_line(&mut num_str).unwrap();
    num_str.trim_end().to_owned();

    let num :i32 = num_str.trim().parse().unwrap();

    let mut count :i32 = 0; // ゾロ目の数をカウントする変数

    let mut answer :i32 = 0; // 解答の数字を格納する変数

    // 上限の 55555 まで全探索
    for i in 0..=55555 {
        let i_str :String = i.to_string();

        // 桁数ごとに、各桁の一致を確かめ、一致したら count を増やす
        if i_str.len() == 1 {
            count += 1;
        } else if i_str.len() == 2 {
            if i_str.chars().nth(0).unwrap() == i_str.chars().nth(1).unwrap() {
                count += 1;
            }
        } else if i_str.len() == 3 {
            if i_str.chars().nth(0).unwrap() == i_str.chars().nth(1).unwrap() && i_str.chars().nth(1).unwrap() == i_str.chars().nth(2).unwrap() {
                count += 1;
            }
        } else if i_str.len() == 4 {
            if i_str.chars().nth(0).unwrap() == i_str.chars().nth(1).unwrap() && i_str.chars().nth(1).unwrap() == i_str.chars().nth(2).unwrap() && i_str.chars().nth(2).unwrap() == i_str.chars().nth(3).unwrap() {
                count += 1;
            }
        } else if i_str.len() == 5 {
            if i_str.chars().nth(0).unwrap() == i_str.chars().nth(1).unwrap() && i_str.chars().nth(1).unwrap() == i_str.chars().nth(2).unwrap() && i_str.chars().nth(2).unwrap() == i_str.chars().nth(3).unwrap() && i_str.chars().nth(3).unwrap() == i_str.chars().nth(4).unwrap() {
                count += 1;
            }
        }
        answer = i;
        if count == num {
          break;
        }
    }
    println!("{}", answer);
}