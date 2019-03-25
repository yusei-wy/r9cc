use std::env;

fn strtonum(p: &mut String) -> i32 {
    let mut result = String::from("");

    for (i, c) in p.bytes().enumerate() {
        if c.is_ascii_digit() {
            result.push(c as char);
        } else {
            break;
        }
    }

    *p = p[result.len()..].to_string();

    result.parse::<i32>().unwrap()
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        panic!("引数の個数が正しくありません\n");
    }
    args.next();

    let mut p: String = match args.next() {
        Some(arg) => arg,
        None => panic!("引数を取得できませんでした"),
    };

    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");
    println!("  mov rax, {}", strtonum(&mut p));

    loop {
        if let Some(b) = p.bytes().next() {
            if b == b'+' {
                p = p[1..].to_string();
                println!("  add rax, {}", strtonum(&mut p));
                continue;
            }

            if b == b'-' {
                p = p[1..].to_string();
                println!("  sub rax, {}", strtonum(&mut p));
                continue;
            }

            panic!("予期しない文字です: {}", b as char);
        } else {
            break;
        }
    }

    println!("  ret");
}
