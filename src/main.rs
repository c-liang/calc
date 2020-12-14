mod ast;
mod evaluate;
mod lex;
fn main() {
    let c =
        evaluate::eval(&ast::expr_parser(&(lex::token_parser("sin(pi/6)").unwrap()[..])).unwrap());
    println!("{}", c);
}
