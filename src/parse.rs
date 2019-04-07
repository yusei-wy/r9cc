use crate::token::*;

pub fn program(tokens: &mut Vec<Token>, pos: &mut usize) -> Vec<Node> {
    let mut code: Vec<Node> = vec![];
    while tokens[*pos] != Token::EOF {
        code.push(stmt(tokens, pos));
    }
    code
}

pub fn stmt(tokens: &mut Vec<Token>, pos: &mut usize) -> Node {
    let node = assign(tokens, pos);
    if !consume(tokens, Token::Semicolon, pos) {
        panic!("';'ではないトークンです: {:?}", tokens[*pos]);
    }
    node
}

pub fn assign(tokens: &mut Vec<Token>, pos: &mut usize) -> Node {
    let mut node = add(tokens, pos);
    while consume(tokens, Token::Assign, pos) {
        node = Node::new(Token::Assign, node, assign(tokens, pos));
    }
    node
}
