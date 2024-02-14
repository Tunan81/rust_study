// 3.1变量与可变性
//const MAX_POINTS: u32 = 100_000;

// 变量和可变性
// fn main() {
//     let x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// 常量
//const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;

// 遮蔽（shadow）
fn main(){
    let x = 5;
    let x = x + 1;
    {
        let  x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

// 遮蔽和将变量标记为 mut 的方式不同，因为除非我们再次使用 let 关键字，否则若是我们不小心尝试重新赋值给这个变量，我们将得到一个编译错误。
// 通过使用 let，我们可以对一个值进行一些转换，但在这些转换完成后，变量将是不可变的。

// true
// fn main() {
//     let spaces = "   ";
//     let spaces = spaces.len();
// }

// false
// fn main() {
//     let mut spaces = "   ";
//     spaces = spaces.len();
// }

