#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
}
use self::Operator::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiters {
    LParen,
    RParen,
}
use self::Delimiters::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Function {
    Sqrt,
    Sin,
    Cos,
    Tan,
    Log,
    Ln,
    Lg,
}
use self::Function::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Constance {
    PI,
    E,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Literals {
    Number(f64),
    Constance(Constance),
}
use self::Literals::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Literals(Literals),
    Operator(Operator),
    Function(Function),
    Delimiters(Delimiters),
}
#[derive(Debug)]
pub enum TokenError {
    InvalidCharacter(char),
    InvalidNumber(String),
    InvalidIdentifier(String),
}
//lexical parser
pub fn token_parser(inputs: &str) -> Result<Vec<Token>, TokenError> {
    let characters: Vec<char> = inputs.chars().collect();
    let mut i = 0;
    let mut tokens = Vec::new();
    while i < characters.len() {
        match characters[i] {
            '+' => tokens.push(Token::Operator(Plus)),
            '-' => tokens.push(Token::Operator(Minus)),
            '*' => tokens.push(Token::Operator(Star)),
            '/' => tokens.push(Token::Operator(Slash)),
            '%' => tokens.push(Token::Operator(Percent)),
            '^' => tokens.push(Token::Operator(Caret)),
            '(' => tokens.push(Token::Delimiters(LParen)),
            ')' => tokens.push(Token::Delimiters(RParen)),
            '√' => tokens.push(Token::Function(Sqrt)),
            'π' => tokens.push(Token::Literals(Constance(Constance::PI))),
            c => {
                if c.is_whitespace() {
                    i += 1;
                    continue;
                } else if c.is_digit(10) || c == '.' {
                    //
                    let mut nums = String::from(c);
                    i = i + 1;
                    while i < characters.len()
                        && (characters[i].is_digit(10) || characters[i] == '.')
                    {
                        nums.push(characters[i]);
                        i += 1;
                    }
                    match nums.parse::<f64>() {
                        Ok(v) => tokens.push(Token::Literals(Number(v))),
                        Err(_) => return Err(TokenError::InvalidNumber(nums)),
                    }
                    continue;
                } else if c.is_alphabetic() {
                    let mut id = String::from(c);
                    i = i + 1;
                    while i < characters.len() && characters[i].is_alphabetic() {
                        id.push(characters[i]);
                        i += 1;
                    }
                    match &id.to_lowercase()[..] {
                        "e" => tokens.push(Token::Literals(Constance(Constance::E))),
                        "pi" => tokens.push(Token::Literals(Constance(Constance::PI))),
                        "sqrt" => tokens.push(Token::Function(Sqrt)),
                        "sin" => tokens.push(Token::Function(Sin)),
                        "cos" => tokens.push(Token::Function(Cos)),
                        "tan" => tokens.push(Token::Function(Tan)),
                        "log" => tokens.push(Token::Function(Log)),
                        "ln" => tokens.push(Token::Function(Ln)),
                        "lg" => tokens.push(Token::Function(Lg)),
                        _ => return Err(TokenError::InvalidIdentifier(id)),
                    }
                    continue;
                } else {
                    return Err(TokenError::InvalidCharacter(c));
                }
            }
        }
        i += 1;
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::Delimiters;
        use super::Operator;
        use super::Token::*;
        use super::*;
        assert_eq!(
            token_parser("log1+2*4+(1+2)").unwrap_or_default(),
            vec![
                Token::Function(super::Function::Log),
                Token::Literals(Number(1 as f64)),
                Operator(Operator::Plus),
                Token::Literals(Number(2 as f64)),
                Operator(Operator::Star),
                Token::Literals(Number(4 as f64)),
                Operator(Operator::Plus),
                Delimiters(Delimiters::LParen),
                Token::Literals(Number(1 as f64)),
                Operator(Operator::Plus),
                Token::Literals(Number(2 as f64)),
                Delimiters(Delimiters::RParen),
            ]
        );
        assert_eq!(
            token_parser("pi+π*4+sin(1)").unwrap_or_default(),
            vec![
                Token::Literals(Constance(super::Constance::PI)),
                Operator(Operator::Plus),
                Token::Literals(Constance(super::Constance::PI)),
                Operator(Operator::Star),
                Token::Literals(Number(4 as f64)),
                Operator(Operator::Plus),
                Token::Function(super::Function::Sin),
                Delimiters(Delimiters::LParen),
                Token::Literals(Number(1 as f64)),
                Delimiters(Delimiters::RParen),
            ]
        );
        assert_eq!(
            token_parser("ln2").unwrap_or_default(),
            vec![
                Token::Function(super::Function::Ln),
                Token::Literals(Number(2 as f64)),
            ]
        );
        assert_eq!(
            token_parser("lg2").unwrap_or_default(),
            vec![
                Token::Function(super::Function::Lg),
                Token::Literals(Number(2 as f64)),
            ]
        );
    }
}
