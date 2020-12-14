use super::lex;
use super::lex::Token;
use std::iter::*;
use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub enum Expr {
    BinOp(lex::Operator, Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Function(lex::Function, Box<Expr>),
    Constance(f64),
}

#[derive(Debug)]
pub enum ExprError {
    ExprInvalidParenthes,
    ExprInvalidFactor(Option<lex::Token>),
}

/// 生成表达式树
/// 运算法则优先级 (+ -) (* / %) (^) (sin cos tan log) (取反)
pub fn expr_parser(tokens: &[lex::Token]) -> Result<Expr, ExprError> {
    additive_expr(&mut tokens.iter().peekable())
}

fn additive_expr(token: &mut Peekable<Iter<lex::Token>>) -> Result<Expr, ExprError> {
    let mut expr = multiplicative_expr(token)?;
    loop {
        match token.peek() {
            Some(Token::Operator(op))
                if op == &lex::Operator::Plus || op == &lex::Operator::Minus =>
            {
                token.next();
                let next_expr = multiplicative_expr(token)?;
                expr = Expr::BinOp(op.clone(), Box::new(expr), Box::new(next_expr));
            }
            _ => break,
        }
    }
    Ok(expr)
}
fn multiplicative_expr(token: &mut Peekable<Iter<lex::Token>>) -> Result<Expr, ExprError> {
    let mut expr = parenthetical_multiplicative_expr(token)?;
    loop {
        match token.peek() {
            Some(Token::Operator(op))
                if op == &lex::Operator::Star
                    || op == &lex::Operator::Slash
                    || op == &lex::Operator::Percent =>
            {
                token.next();
                let new_expr = parenthetical_multiplicative_expr(token)?;
                expr = Expr::BinOp(op.clone(), Box::new(expr), Box::new(new_expr));
            }
            _ => break,
        }
    }
    Ok(expr)
}

//a(b)==a*(b)
fn parenthetical_multiplicative_expr(
    token: &mut Peekable<Iter<lex::Token>>,
) -> Result<Expr, ExprError> {
    let mut expr = power_expr(token)?;
    loop {
        match token.peek() {
            Some(Token::Operator(lex::Operator::LParen)) => {
                token.next();
                let new_expr = additive_expr(token)?;
                match token.next() {
                    Some(Token::Operator(lex::Operator::RParen)) => {
                        expr = Expr::BinOp(lex::Operator::Star, Box::new(expr), Box::new(new_expr));
                    }
                    _ => return Err(ExprError::ExprInvalidParenthes),
                }
            }
            _ => break,
        }
    }
    Ok(expr)
}

fn power_expr(token: &mut Peekable<Iter<lex::Token>>) -> Result<Expr, ExprError> {
    let mut expr = factor(token)?;
    loop {
        match token.peek() {
            Some(Token::Operator(lex::Operator::Caret)) => {
                token.next();
                let new_expr = factor(token)?;
                expr = Expr::BinOp(lex::Operator::Caret, Box::new(expr), Box::new(new_expr));
            }
            _ => break,
        }
    }
    Ok(expr)
}
//基本因子
fn factor(token: &mut Peekable<Iter<lex::Token>>) -> Result<Expr, ExprError> {
    match token.next() {
        Some(Token::Operator(lex::Operator::LParen)) => {
            let expr = additive_expr(token)?;
            match token.next() {
                Some(Token::Operator(lex::Operator::RParen)) => Ok(expr),
                _ => Err(ExprError::ExprInvalidParenthes),
            }
        }
        Some(Token::Function(fun)) => Ok(Expr::Function(*fun, Box::new(factor(token)?))),
        Some(Token::Operator(lex::Operator::Minus)) => Ok(Expr::Neg(Box::new(factor(token)?))),
        Some(Token::Constance(c)) => match c {
            &lex::Constance::PI => Ok(Expr::Constance(std::f64::consts::PI)),
            &lex::Constance::E => Ok(Expr::Constance(std::f64::consts::E)),
            _ => Err(ExprError::ExprInvalidFactor(Some(Token::Constance(*c)))),
        },
        Some(Token::Number(v)) => Ok(Expr::Constance(*v)),
        Some(t) => Err(ExprError::ExprInvalidFactor(Some(t.clone()))),
        None => Err(ExprError::ExprInvalidFactor(None)),
    }
}

#[cfg(test)]
mod tests {
    use super::{expr_parser, Expr};
    #[test]
    fn it_works() {
        use super::expr_parser;
        use super::lex;
        use super::lex::Token;
        use super::Expr;
        use super::ExprError;
        //1+2
        let tokens: Vec<Token> = vec![
            Token::Number(1 as f64),
            Token::Operator(lex::Operator::Plus),
            Token::Number(2 as f64),
        ];
        let v = expr_parser(&tokens[..]).unwrap_or_else(|e| Expr::Constance(0 as f64));
        let r = Expr::BinOp(
            lex::Operator::Plus,
            Box::new(Expr::Constance(1 as f64)),
            Box::new(Expr::Constance(2 as f64)),
        );
        assert_eq!(v, r);
    }
}
