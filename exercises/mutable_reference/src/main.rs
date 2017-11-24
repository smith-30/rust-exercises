/*
ゴール：
1. concatを参照を使って書き換えてください
2. 1で書き換えたconcatを次のように呼び出すと何が起きるか確認してください
   concat(&mut word_a, &word_b)
3. もし2で問題が起きたなら、それを修正してください
*/

fn main() {
    let (mut word_a, word_b) = words();
    println!("単語1：{:?}\n単語2：{:?}", word_a, word_b);
    let text = concat(&mut word_a, &word_b);
    println!("結合された結果：{:?}", text);
}

fn words() -> (String, String) {
    (format!("こんにちは"), format!("世界"))
}

fn concat<'a>(prefix: &'a mut String, postfix: &String) -> &'a String {
    prefix.push(' ');
    for c in postfix.chars() {
        prefix.push(c);
    }
    prefix
}

// fn main() {
//     let (mut word_a, word_b) = words();
//     println!("単語1：{:?}\n単語2：{:?}", word_a, word_b);
//     concat(&mut word_a, &word_b); //ミュータブル参照を渡す
//     println!("結合された結果：{:?}", word_a);
// }
//
// fn words() -> (String, String) {
//     (format!("こんにちは"), format!("世界"))
// }
//
// fn concat(prefix: &mut String, postfix: &String) { //ミュータブル参照を受け取る
//     prefix.push(' ');
//     for c in postfix.chars() {
//         prefix.push(c);
//     }
// }
