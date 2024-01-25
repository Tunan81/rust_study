// 3.1变量与可变性
//const MAX_POINTS: u32 = 100_000;

fn main() {
    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("Hello, world!");
    println!("{}", x);

    let spaces = "  ";
    let spaces = spaces.len();
    println!("{}", spaces);
}
