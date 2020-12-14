use super::ast;
use super::lex;

pub fn eval(expr: &ast::Expr) -> f64 {
    match expr {
        ast::Expr::BinOp(op, expr1, expr2) => {
            let (num1, num2) = (eval(expr1), eval(expr2));
            match op {
                lex::Operator::Plus => num1 + num2,
                lex::Operator::Minus => num1 - num2,
                lex::Operator::Star => num1 * num2,
                lex::Operator::Slash => num1 / num2,
                lex::Operator::Percent => num1 % num2,
                lex::Operator::Caret => num1.powf(num2),
                _ => todo!(),
            }
        }
        ast::Expr::Function(func, expr1) => {
            let num = eval(expr1);
            match func {
                lex::Function::Sqrt => num.sqrt(),
                lex::Function::Sin => num.sin(),
                lex::Function::Cos => num.cos(),
                lex::Function::Tan => num.tan(),
                lex::Function::Log => num.log10(),
                lex::Function::Ln => num.ln(),
                lex::Function::Lg => num.log2(),
                _ => todo!(),
            }
        }
        ast::Expr::Neg(e) => -eval(e),
        ast::Expr::Constance(v) => *v,
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::eval(
                &super::ast::expr_parser(&(super::lex::token_parser("12.3+12.0").unwrap()[..]))
                    .unwrap(),
            ),
            12.3 + 12.0
        );

        assert_eq!(
            super::eval(
                &super::ast::expr_parser(
                    &(super::lex::token_parser("sin(12.0) + 10.0 * 2.5 + 7").unwrap()[..])
                )
                .unwrap(),
            ),
            (12.0 as f64).sin() + 10.0 * 2.5 + 7 as f64
        );

        assert_eq!(
            super::eval(
                &super::ast::expr_parser(
                    &(super::lex::token_parser("sin(12.0) + 10.0 * 2.5 + ln7").unwrap()[..])
                )
                .unwrap(),
            ),
            (12.0 as f64).sin() + 10.0 * 2.5 + (7 as f64).ln() as f64
        );
    }
}
