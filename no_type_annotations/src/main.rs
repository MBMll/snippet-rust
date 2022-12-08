#![allow(unused)]

fn main() {
    let a = [3; 5];
    println!("{a:?}");
    println!("{a:#?}");
    let a = [3, 3, 3, 3, 3];
    println!("{a:?}");
    println!("{a:#?}");
    let a = [3;5, 4;5]; // expected one of `.`, `?`, `]`, or an operator
    println!("{a:?}", );
}
