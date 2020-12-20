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
            }
        }
        ast::Expr::Neg(e) => -eval(e),
        ast::Expr::Constance(v) => *v,
    }
}

pub fn to_string(expr: &ast::Expr) -> String {
    match expr {
        ast::Expr::BinOp(op, expr1, expr2) => match op {
            lex::Operator::Plus => format!("({}+{})", to_string(expr1), to_string(expr2)),
            lex::Operator::Minus => format!("({}-{})", to_string(expr1), to_string(expr2)),
            lex::Operator::Star => format!("({}*{})", to_string(expr1), to_string(expr2)),
            lex::Operator::Slash => format!("({}/{})", to_string(expr1), to_string(expr2)),
            lex::Operator::Percent => format!("({}%{})", to_string(expr1), to_string(expr2)),
            lex::Operator::Caret => format!("({}^{})", to_string(expr1), to_string(expr2)),
        },
        ast::Expr::Function(func, expr1) => match func {
            lex::Function::Sqrt => format!("Sqrt({}) ", to_string(expr1)),
            lex::Function::Sin => format!("Sin({}) ", to_string(expr1)),
            lex::Function::Cos => format!("Cos({}) ", to_string(expr1)),
            lex::Function::Tan => format!("Tan({}) ", to_string(expr1)),
            lex::Function::Log => format!("Log({}) ", to_string(expr1)),
            lex::Function::Ln => format!("Ln({}) ", to_string(expr1)),
            lex::Function::Lg => format!("Lg({}) ", to_string(expr1)),
        },
        ast::Expr::Neg(e) => format!("(-{})", to_string(e)),
        ast::Expr::Constance(v) => format!("{}", v.to_string()),
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

        assert_eq!(
            super::to_string(
                &super::ast::expr_parser(&(super::lex::token_parser("1+2").unwrap()[..])).unwrap()
            ),
            String::from("(1+2)")
        );
    }
}
