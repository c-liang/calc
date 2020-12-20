mod ast;
mod evaluate;
mod lex;
fn main() {
    let c = evaluate::eval(
        &ast::expr_parser(&(lex::token_parser("sin1^2+cos1^2").unwrap()[..])).unwrap(),
    );
    let d = evaluate::to_string(
        &ast::expr_parser(&(lex::token_parser("sin cos1^2+cos1^2+(1-3)-1*5-2").unwrap()[..]))
            .unwrap(),
    );
    println!("{}, {}", c, d);
}
