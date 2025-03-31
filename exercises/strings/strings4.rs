// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!
//string_slice函数接受一个&str类型的参数。&str是不可变的字符串切片，它可以指向堆上的String，也可以指向静态的、编译时已知的字符串字面量
fn string_slice(arg: &str) {
    println!("{}", arg);
}
//string函数接受一个String类型的参数。String是一个可变的所有权字符串，它存储在堆上，可以自由地增长和修改。
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
