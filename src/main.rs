use std::env;

#[derive(Debug, PartialEq)]
enum Token {
    Num(i64), // Number Literal

    Lparen,
    Rparen,
    Plus,
    Minus,
    Slash,
    Asterisk,

    EOF,
}

#[derive(Debug)]
struct Node {
    token: Token,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    fn new(token: Token, lhs: Node, rhs: Node) -> Node {
        Node {
            token,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
        }
    }

    fn new_node_num(token: Token) -> Node {
        Node {
            token,
            lhs: None,
            rhs: None,
        }
    }
}

/// 文字列を数値に変換する
/// 文字列中の数値に当たる部分は変換後削除される
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

/// 文字列を字句トークンにトークナイズする
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
                b'*' => {
                    tokens.push(Token::Asterisk);
                    *p = &p[1..];
                    continue;
                }
                b'/' => {
                    tokens.push(Token::Slash);
                    *p = &p[1..];
                    continue;
                }
                b'(' => {
                    tokens.push(Token::Lparen);
                    *p = &p[1..];
                    continue;
                }
                b')' => {
                    tokens.push(Token::Rparen);
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

/// 次のトークンが期待した型かチェックし, 期待した型の場合だけ入力を1トークン進めて真を返す
fn consume(tokens: &mut Vec<Token>, tok: Token, pos: &mut usize) -> bool {
    if tokens[*pos] == tok {
        *pos += 1;
        true
    } else {
        false
    }
}

/// 足し算と引き算
fn add(tokens: &mut Vec<Token>, pos: &mut usize) -> Node {
    let mut node = mul(tokens, pos);

    loop {
        if consume(tokens, Token::Plus, pos) {
            node = Node::new(Token::Plus, node, mul(tokens, pos));
        } else if consume(tokens, Token::Minus, pos) {
            node = Node::new(Token::Minus, node, mul(tokens, pos));
        } else {
            return node;
        }
    }
}

/// 掛け算と割り算
fn mul(tokens: &mut Vec<Token>, pos: &mut usize) -> Node {
    let mut node = term(tokens, pos);

    loop {
        if consume(tokens, Token::Asterisk, pos) {
            node = Node::new(Token::Asterisk, node, term(tokens, pos));
        } else if consume(tokens, Token::Slash, pos) {
            node = Node::new(Token::Slash, node, term(tokens, pos));
        } else {
            return node;
        }
    }
}

/// カッコの計算
fn term(tokens: &mut Vec<Token>, pos: &mut usize) -> Node {
    if consume(tokens, Token::Lparen, pos) {
        let node = add(tokens, pos);
        if !consume(tokens, Token::Rparen, pos) {
            panic!(
                "開きカッコに対応する閉じカッコがありません: {:?}",
                tokens[*pos]
            );
        }
        return node;
    }

    match tokens[*pos] {
        Token::Num(num) => {
            *pos += 1;
            return Node::new_node_num(Token::Num(num));
        }
        _ => {}
    }

    panic!(
        "数値でも開きカッコでもないトークンです: {:?}",
        tokens[*pos]
    );
}

/// x86-64 のスタック操作命令でスタックマシンをエミュレート
fn gen(node: Node) {
    match node.token {
        Token::Num(num) => {
            println!("  push {}", num);
            return;
        }
        _ => {}
    }

    gen(*node.lhs.unwrap());
    gen(*node.rhs.unwrap());

    println!("  pop rdi");
    println!("  pop rax");

    match node.token {
        Token::Plus => println!("  add rax, rdi"),
        Token::Minus => println!("  sub rax, rdi"),
        Token::Asterisk => println!("  mul rdi"),
        Token::Slash => {
            println!("  mov rdx, 0");
            println!("  div rdi");
        }
        _ => {}
    }

    println!("  push rax");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数の個数が正しくありません\n");
    }

    let mut p: &str = &args[1];

    let mut tokens = tokenize(&mut p);
    let mut pos = 0;
    let node = add(&mut tokens, &mut pos);

    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");

    // 抽象構文木を下りながらコード生成
    gen(node);

    // スタックトップに残っているはずの式全体の値を rax にロードして関数の返り値とする
    println!("  pop rax");
    println!("  ret");
}
