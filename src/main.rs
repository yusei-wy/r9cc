use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数の個数が正しくありません\n");
    }

    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");
    println!("  mov rax, {}", &args[1].parse::<i32>().unwrap());
    println!("  ret");
}
