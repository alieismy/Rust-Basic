// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    // 将 string2 移到外部作用域
    let string2 = String::from("xyz");
    // result 的声明紧跟在使用之前
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'", result);
}
