#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
}
use self::Operator::*;
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
pub enum Token {
    Number(f64),
    Operator(Operator),
    Function(Function),
    Constance(Constance),
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
            '(' => tokens.push(Token::Operator(LParen)),
            ')' => tokens.push(Token::Operator(RParen)),
            '√' => tokens.push(Token::Function(Sqrt)),
            'π' => tokens.push(Token::Constance(Constance::PI)),
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
                        Ok(v) => tokens.push(Token::Number(v)),
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
                        "e" => tokens.push(Token::Constance(Constance::E)),
                        "pi" => tokens.push(Token::Constance(Constance::PI)),
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
    use super::{token_parser, Constance};

    #[test]
    fn it_works() {
        use super::Operator;
        use super::Token::*;
        use super::*;
        assert_eq!(
            token_parser("log1+2*4+(1+2)").unwrap_or_default(),
            vec![
                Token::Function(super::Function::Log),
                Number(1 as f64),
                Operator(Operator::Plus),
                Number(2 as f64),
                Operator(Operator::Star),
                Number(4 as f64),
                Operator(Operator::Plus),
                Operator(Operator::LParen),
                Number(1 as f64),
                Operator(Operator::Plus),
                Number(2 as f64),
                Operator(Operator::RParen),
            ]
        );
        assert_eq!(
            token_parser("pi+π*4+sin(1)").unwrap_or_default(),
            vec![
                Token::Constance(super::Constance::PI),
                Operator(Operator::Plus),
                Token::Constance(super::Constance::PI),
                Operator(Operator::Star),
                Number(4 as f64),
                Operator(Operator::Plus),
                Token::Function(super::Function::Sin),
                Operator(Operator::LParen),
                Number(1 as f64),
                Operator(Operator::RParen),
            ]
        );
        assert_eq!(
            token_parser("ln2").unwrap_or_default(),
            vec![Token::Function(super::Function::Ln), Number(2 as f64),]
        );
        assert_eq!(
            token_parser("lg2").unwrap_or_default(),
            vec![Token::Function(super::Function::Lg), Number(2 as f64),]
        );
    }
}
