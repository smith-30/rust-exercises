/*
ゴール：
1. コンパイルし、実行してください
2. worldを自分の名前に置き換えてみてください
3. 変数 name を使って2を実現してください
4. println!("{}", name)を試しください
5. println!("{}", name)とprintln!("{:?}", name)の違いを確認してください
println!("{}", name)
kouhei

println!("{:?}", name)
"kohei"

6. greetingsを呼ぶように変更してください
*/
#![allow(dead_code)]

fn main() {

    let name = "kohei".to_string();

    greetings(&name);
    greetings2(name.clone());

    println!("end, {}!", name);
}

fn greetings(name: &String) {
    println!("Hello, {}!", name);
}

fn greetings2(name: String) {
    println!("Hello, {}!", name);
}
