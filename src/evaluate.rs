use super::parser;
use super::token;

pub fn eval(expr: &parser::Expr) -> f64 {
    match expr {
        parser::Expr::BinOp(op, expr1, expr2) => {
            let (num1, num2) = (eval(expr1), eval(expr2));
            match op {
                token::Operator::Plus => num1 + num2,
                token::Operator::Minus => num1 - num2,
                token::Operator::Star => num1 * num2,
                token::Operator::Slash => num1 / num2,
                token::Operator::Percent => num1 % num2,
                token::Operator::Caret => num1.powf(num2),
                _ => todo!(),
            }
        }
        parser::Expr::Function(func, expr1) => {
            let num = eval(expr1);
            match func {
                token::Function::Sqrt => num.sqrt(),
                token::Function::Sin => num.sin(),
                token::Function::Cos => num.cos(),
                token::Function::Tan => num.tan(),
                token::Function::Log => num.log10(),
                token::Function::Ln => num.ln(),
                _ => todo!(),
            }
        }
        parser::Expr::Neg(e) => -eval(e),
        parser::Expr::Constance(v) => *v,
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::eval(
                &super::parser::expr_parser(
                    &(super::token::token_parser("12.3+12.0").unwrap()[..])
                )
                .unwrap(),
            ),
            12.3 + 12.0
        );

        assert_eq!(
            super::eval(
                &super::parser::expr_parser(
                    &(super::token::token_parser("sin(12.0) + 10.0 * 2.5 + 7").unwrap()[..])
                )
                .unwrap(),
            ),
            (12.0 as f64).sin() + 10.0 * 2.5 + 7 as f64
        );

        assert_eq!(
            super::eval(
                &super::parser::expr_parser(
                    &(super::token::token_parser("sin(12.0) + 10.0 * 2.5 + ln7").unwrap()[..])
                )
                .unwrap(),
            ),
            (12.0 as f64).sin() + 10.0 * 2.5 + (7 as f64).ln() as f64
        );
    }
}
