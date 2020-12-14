mod evaluate;
mod parser;
mod token;
fn main() {
    let c = evaluate::eval(
        &parser::expr_parser(&(token::token_parser("sin(pi/6)").unwrap()[..])).unwrap(),
    );
    println!("{}", c);
}
