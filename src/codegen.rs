use crate::token::*;

/// x86-64 のスタック操作命令でスタックマシンをエミュレート
pub fn gen(node: Node) {
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

pub fn gen_lval(node: Node) {
    let name = match node.token {
        Token::Ident(name) => name,
        _ => panic!("代入の左辺値が変数ではありません"),
    };

    let offset = (b'z' - name.as_bytes()[0] + 1) * 8;
    println!("  mov rax, rbp");
    println!("  sub rax, {}", offset);
    println!("  push rax");
}
