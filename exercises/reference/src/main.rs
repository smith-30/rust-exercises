/*
ゴール：
1. データをコピーしなくても、greetを呼び出せるようにしてください
2. "Hello dear rustaceans"ではなく、"Hello rustaceans"となるように、
   2回目のgreetの呼び出しを行なってください
3. 2をスライスを使って実現してください
*/

fn main() {
    let name = format!("dear rustaceans");
    let r = &name[..];//全ての文字列が対象
    let s = &name[5..];//この数字は byteの始まり。日本語だとアウト
    greet(r);
    greet(s);
}

fn greet(name: &str) {
    println!("Hello {}", name);
}

// Hello dear rustaceans
// Hello rustaceans
