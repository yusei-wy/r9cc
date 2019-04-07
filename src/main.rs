extern crate r9cc;

use r9cc::codegen::*;
use r9cc::parse::*;
// use r9cc::container::*;
use r9cc::token::*;

use std::env;

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
