// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    my_macro!();
}
//宏定义遵循如下规则：
//1. 宏定义必须以 `macro_rules!` 开始，后跟宏名和参数列表。
//2. 宏定义中的 `=>` 右边是宏的定义体，可以包含任意 Rust 代码。
//3.如果宏定义定义在调用者下面, 则调用者必须使用 `#[macro_use]` 标注来导入该宏。
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
