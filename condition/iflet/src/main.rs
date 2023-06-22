enum Symbol {
    Char(char),
    Number(),
}
// 用于枚举的匹配模式
fn main() {
    let letter = Symbol::Char('A');
    if let Symbol::Char(x) = letter {
        println!("{}", x)
    }
}
