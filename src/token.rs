#[derive(Debug, PartialEq)]
pub enum Token {
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
pub struct Node {
    pub token: Token,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
    name: String,
}

impl Node {
    pub fn new(token: Token, lhs: Node, rhs: Node) -> Node {
        Node {
            token,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
            name: String::from(""),
        }
    }

    pub fn new_node_num(num: i64) -> Node {
        Node {
            token: Token::Num(num),
            lhs: None,
            rhs: None,
            name: String::from(""),
        }
    }

    pub fn new_node_ident(name: &str) -> Node {
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
pub fn strtonum(p: &mut &str) -> i64 {
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
pub fn tokenize(p: &mut &str) -> Vec<Token> {
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
pub fn consume(tokens: &mut Vec<Token>, tok: Token, pos: &mut usize) -> bool {
    if tokens[*pos] == tok {
        *pos += 1;
        true
    } else {
        false
    }
}

/// 足し算と引き算
pub fn add(tokens: &mut Vec<Token>, pos: &mut usize) -> Node {
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
pub fn mul(tokens: &mut Vec<Token>, pos: &mut usize) -> Node {
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
pub fn term(tokens: &mut Vec<Token>, pos: &mut usize) -> Node {
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
