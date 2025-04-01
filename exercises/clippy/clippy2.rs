// clippy2.rs
//
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    //如果option是Some，则将res加上option中的值
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
