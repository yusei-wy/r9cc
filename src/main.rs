use std::env;

#[derive(Debug)]
enum Token {
    Num(i64),
    EOF,

    Plus,
    Minus,
}

fn strtonum(p: &mut &str) -> i64 {
    let mut result = String::from("");

    for c in p.bytes() {
        if c.is_ascii_digit() {
            result.push(c as char);
        } else {
            break;
        }
    }

    *p = &p[result.len()..];

    result.parse::<i64>().unwrap()
}

fn tokenize(p: &mut &str) -> Vec<Token> {
    let mut tokens = vec![];

    loop {
        if let Some(b) = p.bytes().next() {
            if b.is_ascii_whitespace() {
                *p = &p[1..];
                continue;
            }

            match b {
                b'+' => {
                    tokens.push(Token::Plus);
                    *p = &p[1..];
                    continue;
                }
                b'-' => {
                    tokens.push(Token::Minus);
                    *p = &p[1..];
                    continue;
                }
                b'0'...b'9' => {
                    tokens.push(Token::Num(strtonum(p)));
                    continue;
                }
                _ => {
                    panic!("トークナイズできません: {}", p);
                }
            }
        } else {
            tokens.push(Token::EOF);
            break;
        }
    }

    tokens
}

fn error(token: &Token) {
    panic!("予期しないトークンです: {:?}", token);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数の個数が正しくありません\n");
    }

    let mut p: &str = &args[1];

    let tokens = tokenize(&mut p);

    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");

    let mut iter = tokens.iter();

    // 式の最初は数で始まらなければならないのでチェックしてから最初の mov 命令を出力
    if let Some(token) = iter.next() {
        match token {
            Token::Num(num) => println!("  mov rax, {}", num),
            _ => {}
        }
    }

    // `+ <数>` あるいは `- <数>` というトークンを消費しつつアセンブリを出力
    let mut iter = iter.peekable();
    loop {
        if let Some(token) = iter.next() {
            match token {
                Token::EOF => break,
                Token::Plus => {
                    let peek = iter.peek().cloned();
                    if let Some(token) = peek {
                        match token {
                            Token::Num(num) => {
                                println!("  add rax, {}", num);
                                iter.next();
                            }
                            _ => error(token),
                        }
                    }
                    continue;
                }
                Token::Minus => {
                    let peek = iter.peek().cloned();
                    if let Some(token) = peek {
                        match token {
                            Token::Num(num) => {
                                println!("  sub rax, {}", num);
                                iter.next();
                            }
                            _ => error(token),
                        }
                    }
                    continue;
                }
                _ => error(token),
            }
        } else {
            break;
        }
    }

    println!("  ret");
}
