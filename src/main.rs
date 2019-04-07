use std::env;

#[derive(Debug, PartialEq)]
enum Token {
    Num(i64),
    Ident(String),

    Assign,
    Semicolon,

    Lparen,
    Rparen,
    Plus,
    Minus,
    Slash,
    Asterisk,

    EOF,
}

// enum NodeKind {
//     Num(i64),
//     Ident(String),
// }

#[derive(Debug)]
struct Node {
    token: Token,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
    name: String,
}

impl Node {
    fn new(token: Token, lhs: Node, rhs: Node) -> Node {
        Node {
            token,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
            name: String::from(""),
        }
    }

    fn new_node_num(num: i64) -> Node {
        Node {
            token: Token::Num(num),
            lhs: None,
            rhs: None,
            name: String::from(""),
        }
    }

    fn new_node_ident(name: &str) -> Node {
        let name = name.to_string();
        Node {
            token: Token::Ident(name),
            lhs: None,
            rhs: None,
            name: String::from(""),
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
                b'=' => {
                    tokens.push(Token::Assign);
                    *p = &p[1..];
                    continue;
                }
                b';' => {
                    tokens.push(Token::Semicolon);
                    *p = &p[1..];
                    continue;
                }
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
                b'a'...b'z' => {
                    tokens.push(Token::Ident((b as char).to_string()));
                    *p = &p[1..];
                    continue;
                }
                b'0'...b'9' => {
                    // strtonum で p をすすめるので p の書き換えはここでは不要
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
            return Node::new_node_num(num);
        }
        Token::Ident(ref name) => {
            *pos += 1;
            return Node::new_node_ident(name);
        }
        _ => {}
    }

    panic!(
        "数値でも開きカッコでもないトークンです: {:?}",
        tokens[*pos]
    );
}

fn gen_lval(node: Node) {
    let name = match node.token {
        Token::Ident(name) => name,
        _ => panic!("代入の左辺値が変数ではありません"),
    };

    let offset = (b'z' - name.as_bytes()[0] + 1) * 8;
    println!("  mov rax, rbp");
    println!("  sub rax, {}", offset);
    println!("  push rax");
}

/// x86-64 のスタック操作命令でスタックマシンをエミュレート
fn gen(node: Node) {
    match node.token {
        Token::Num(num) => {
            println!("  push {}", num);
            return;
        }
        Token::Ident(_) => {
            gen_lval(node);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        }
        Token::Assign => {
            gen_lval(*node.lhs.unwrap());
            gen(*node.rhs.unwrap());

            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
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

fn program(tokens: &mut Vec<Token>, pos: &mut usize) -> Vec<Node> {
    let mut code: Vec<Node> = vec![];
    while tokens[*pos] != Token::EOF {
        code.push(stmt(tokens, pos));
    }
    code
}

fn stmt(tokens: &mut Vec<Token>, pos: &mut usize) -> Node {
    let node = assign(tokens, pos);
    if !consume(tokens, Token::Semicolon, pos) {
        panic!("';'ではないトークンです: {:?}", tokens[*pos]);
    }
    node
}

fn assign(tokens: &mut Vec<Token>, pos: &mut usize) -> Node {
    let mut node = add(tokens, pos);
    while consume(tokens, Token::Assign, pos) {
        node = Node::new(Token::Assign, node, assign(tokens, pos));
    }
    node
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数の個数が正しくありません\n");
    }

    let mut p: &str = &args[1];

    // トークナイズしてパースする
    // 結果は code に保存される
    let mut tokens = tokenize(&mut p);
    let mut pos = 0;
    let code: Vec<Node> = program(&mut tokens, &mut pos);

    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");
    println!(".global _main");
    println!("_main:");

    // プロローグ
    // 変数26個分の領域を確保する
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    // 先頭の式から順にコードに変換
    for c in code {
        gen(c);

        // 式の評価結果としてスタックに1つの値が残っているはずなので
        // スタックが溢れないようにポップしておく
        println!("  pop rax");
    }

    // エピローグ
    // 最後の式の結果が RAX に残っているのでそれが返り値になる
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
