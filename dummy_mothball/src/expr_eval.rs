use std::collections::HashMap;
use regex::Regex;
use indexmap::IndexMap;
use crate::parser::Data;
use once_cell::sync::Lazy;

#[derive(Clone)]
enum TokenKind {
    Number,
    Plus,
    Minus,
    Pow,
    Times,
    Divide,
    Lparen,
    Rparen,
    Id,
    Comma,
}

#[derive(Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Pow,
    Times,
    Divide,
    Lparen,
    Rparen,
    Id(String),
    Comma,
}

impl Token {
    fn kind(&self) -> TokenKind {
        match self {
            Token::Number(_) => TokenKind::Number,
            Token::Plus => TokenKind::Plus,
            Token::Minus => TokenKind::Minus,
            Token::Pow => TokenKind::Pow,
            Token::Times => TokenKind::Times,
            Token::Divide => TokenKind::Divide,
            Token::Lparen => TokenKind::Lparen,
            Token::Rparen => TokenKind::Rparen,
            Token::Id(_) => TokenKind::Id,
            Token::Comma => TokenKind::Comma,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum Identity {
    Variable(String),
    Function(fn(Vec<f64>) -> f64),
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum Operator {
    Plus,
    Minus,
    UnaryMinus,
    Pow,
    Times,
    Divide,
    Lparen,
    Id(Identity),
}

use std::fmt;

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::UnaryMinus => write!(f, "unary -"),
            Operator::Pow => write!(f, "**"),
            Operator::Times => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Lparen => write!(f, "("),
            Operator::Id(_) => write!(f, "id"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Pow => write!(f, "**"),
            Token::Times => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Lparen => write!(f, "("),
            Token::Id(_) => write!(f, "id"),
            Token::Comma => write!(f, ","),
            Token::Rparen => write!(f, ")"),
            Token::Number(_) => write!(f, "ﬂoat"),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Pow => write!(f, "**"),
            TokenKind::Times => write!(f, "*"),
            TokenKind::Divide => write!(f, "/"),
            TokenKind::Lparen => write!(f, "("),
            TokenKind::Id => write!(f, "id"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Rparen => write!(f, ")"),
            TokenKind::Number => write!(f, "ﬂoat"),
        }
    }
}

static TOKEN_SPECIFICATION: &[(&str, &str)] = &[
    ("NUMBER", r"\d+(\.\d+)?[eE]-\d+|\d+(\.\d+)?[eE]\d+|\d+\.\d+|\d+\.|\.\d+|\d+"),
    ("PLUS", r"\+"),
    ("MINUS", r"-"),
    ("POW", r"\*\*"),
    ("TIMES", r"\*"),
    ("DIVIDE", r"/"),
    ("LPAREN", r"\("),
    ("RPAREN", r"\)"),
    ("ID", r"[A-Za-z_][A-Za-z_0-9]*"),
    ("WHITESPACE", r"\s+"),
    ("COMMA", r","),
    ("MISMATCH", r"."),
];

static TOKEN_REGEX: Lazy<Regex> = Lazy::new(|| {
    let tok_regex = TOKEN_SPECIFICATION
        .iter()
        .map(|(token_type, pattern)| format!("(?P<{}>{})", token_type, pattern))
        .collect::<Vec<_>>()
        .join("|");

    Regex::new(
        &tok_regex
    ).unwrap()
});

fn _tokenize(expression: &str) -> Vec<Token> {
    let mut r = Vec::new();
    let mut nest = 0;

    let mut prevkind: Option<String> = None;

    for caps in TOKEN_REGEX.captures_iter(expression) {
        let match_object = caps.get(0).unwrap();
        let value: &str = match_object.as_str();

        let mut kind = "";

        for (name, _) in TOKEN_SPECIFICATION {
            if caps.name(name).is_some() {
                kind = name;
                break;
            }
        }

        if kind == "MISMATCH" {
            panic!("Invalid token '{}'", value);
        } else if kind == "NUMBER" {
            if prevkind.as_deref() == Some("RPAREN") {
                panic!("Invalid Expression, number after right parethesis");
            }
            let parsed_value = value.parse::<f64>().unwrap();
            r.push(Token::Number(parsed_value));
        } else if kind == "LPAREN" {
            nest += 1;
            if prevkind.as_deref() == Some("NUMBER") {
                panic!("Invalid Expression, left parenthesis after number");
            }
            r.push(Token::Lparen);
        } else if kind == "RPAREN" {
            if nest == 0 {
                panic!("Unbalanced Parenthesis");
            }
            r.push(Token::Rparen);
            nest -= 1;
        } else if kind == "PLUS" {
            r.push(Token::Plus);
        } else if kind == "MINUS" {
            r.push(Token::Minus);
        } else if kind == "POW" {
            r.push(Token::Pow);
        } else if kind == "TIMES" {
            r.push(Token::Times);
        } else if kind == "DIVIDE" {
            r.push(Token::Divide);
        } else if kind == "ID" {
            let parsed_value = value.to_string();
            r.push(Token::Id(parsed_value));
        } else if kind == "COMMA" {
            r.push(Token::Comma);
        }

        if kind != "WHITESPACE" {
            if (prevkind.as_deref() == Some("ID") || prevkind.as_deref() == Some("NUMBER"))
                && (kind == "NUMBER" && kind == "ID")
            {
                panic!("Invalid Expression, number and variable have no operation in between");
            }

            prevkind = Some(kind.to_string());
        }
    }

    if nest != 0 {
        panic!("Unbalanced Parenthesis");
    }

    // println!("{:?}", r);
    r
}


fn _apply_operator(operands: &mut Vec<f64>, operator: Operator) {
    if matches!(operator, Operator::UnaryMinus)  {
        if operands.is_empty() {
            panic!("Invalid expression");
        }

        let a = operands.pop().unwrap();
        operands.push(-a);
    }

    if operands.len() <= 1 {
        panic!("Invalid Expression");
    }

    let b = operands.pop().unwrap();
    let a = operands.pop().unwrap();

    if matches!(operator, Operator::Plus) {
        operands.push(a + b);
    } else if matches!(operator, Operator::Minus) {
        operands.push(a - b);
    } else if matches!(operator, Operator::Times) {
        operands.push(a * b);
    } else if matches!(operator, Operator::Divide) {
        operands.push(a / b);
    } else if matches!(operator, Operator::Pow) {
        operands.push(a.powf(b));
    }
}

fn _evaluate(tokens: Vec<Token>, variables: IndexMap<String, Data>) -> f64 {
    use crate::expr_eval::Token::{Number, Id, Minus, Plus, Times, Divide, Pow, Lparen, Rparen, Comma};
    // println!("{:?}", tokens);

    fn abs(floats: Vec<f64>) -> f64 {
        return floats[0].abs()
    }

    fn min(floats: Vec<f64>) -> f64 {
        let min_value = floats
            .iter()
            .copied()
            .reduce(f64::min);

        match min_value {
            Some(min) => {min}
            None => panic!("Min function received something unexpected!")
        }
    }

    fn max(floats: Vec<f64>) -> f64 {
        let max_value = floats
            .iter()
            .copied()
            .reduce(f64::max);

        match max_value {
            Some(max) => {max}
            None => panic!("Max function received something unexpected!")
        }
    }

    let functions: HashMap<&'static str, Identity> = HashMap::from([
        ("abs", Identity::Function(abs)),
        ("max", Identity::Function(max)),
        ("min", Identity::Function(min)),
    ]);

    let mut precedence: HashMap<Operator, usize> = HashMap::new();
    precedence.insert(Operator::Plus, 1);
    precedence.insert(Operator::Minus, 1);
    precedence.insert(Operator::Times, 2);
    precedence.insert(Operator::Divide, 2);
    precedence.insert(Operator::Pow, 3);
    precedence.insert(Operator::UnaryMinus, 4);

    let mut operands: Vec<f64> = Vec::new();
    let mut operators: Vec<Operator> = Vec::new();

    let mut prevkind: Option<TokenKind> = None;
//    let mut prevvalue: Option<Value> = None;

    let mut argument_counts: Vec<usize> = Vec::new();

    for token in tokens {
        let kind = token.kind();
        match token {
            Number(float) => {
                operands.push(float)
            }

            Id(name) => {
                if functions.contains_key(name.as_str()) {
                    argument_counts.push(1);
                    operators.push(Operator::Id(functions[name.as_str()].clone()));
                } else if variables.contains_key(&name) {
                    operands.push(variables[&name].clone().get_as_f64());
                } else {
                    panic!("Unknown variable or function: {}", name);
                }
            }

            Minus => {
                if prevkind.is_none()
                    || matches!(
                        prevkind,
                        Some(TokenKind::Plus)
                            | Some(TokenKind::Minus)
                            | Some(TokenKind::Times)
                            | Some(TokenKind::Divide)
                            | Some(TokenKind::Pow)
                            | Some(TokenKind::Lparen)
                            | Some(TokenKind::Comma)
                    )
                {
                    // unary minus occurs at start or after another operator or after a left parenthesis;
                    while !operators.is_empty()
                        && precedence.contains_key(operators.last().unwrap())
                        && precedence.get(operators.last().unwrap()).unwrap() >= precedence.get(&Operator::UnaryMinus).unwrap()
                    {
                        _apply_operator(&mut operands, operators.pop().unwrap());
                    }
                    operators.push(Operator::UnaryMinus); 
                }

                else 
                {
                    while !operators.is_empty()
                        && precedence.contains_key(operators.last().unwrap())
                        && precedence.get(operators.last().unwrap()).unwrap() >= precedence.get(&Operator::Minus).unwrap() 
                        {
                            _apply_operator(&mut operands, operators.pop().unwrap());
                        }
                    operators.push(Operator::Minus);
                }
            }

            Plus => {
                while !operators.is_empty()
                    && precedence.contains_key(operators.last().unwrap())
                    && precedence.get(operators.last().unwrap()).unwrap() >= precedence.get(&Operator::Plus).unwrap()
                {
                    _apply_operator(&mut operands, operators.pop().unwrap());
                }

                operators.push(Operator::Plus);
            }

            Times => {
                while !operators.is_empty()
                    && precedence.contains_key(operators.last().unwrap())
                    && precedence.get(operators.last().unwrap()).unwrap() >= precedence.get(&Operator::Times).unwrap()
                {
                    _apply_operator(&mut operands, operators.pop().unwrap());
                }

                operators.push(Operator::Times);
            }

            Divide => {
                while !operators.is_empty()
                    && precedence.contains_key(operators.last().unwrap())
                    && precedence.get(operators.last().unwrap()).unwrap() >= precedence.get(&Operator::Divide).unwrap()
                {
                    _apply_operator(&mut operands, operators.pop().unwrap());
                }

                operators.push(Operator::Divide);
            }

            Pow => {
                while !operators.is_empty()
                    && precedence.contains_key(operators.last().unwrap())
                    && precedence.get(operators.last().unwrap()).unwrap() >= precedence.get(&Operator::Pow).unwrap()
                {
                    _apply_operator(&mut operands, operators.pop().unwrap());
                }

                operators.push(Operator::Pow);  
            }

            Lparen => {
                operators.push(Operator::Lparen);
            }

            Rparen => {
                while !matches!(operators.last().unwrap(), Operator::Lparen) {
                    _apply_operator(&mut operands, operators.pop().unwrap());
                }

                operators.pop();

                let last = operators.last().unwrap().clone();
                if !operators.is_empty() && let Operator::Id(Identity::Function(func)) = operators.pop().unwrap() {
                    let nargs = argument_counts.pop().unwrap();
                    let args = operands[operands.len() - nargs..].to_vec();

                    operands.truncate(operands.len() - nargs);

                    operands.push(func(args));
                } else {
                    operators.push(last);
                }
            }

            Comma => {
                *argument_counts.last_mut().unwrap() += 1;

                while !matches!(operators.last().unwrap(), Operator::Lparen) {
                    _apply_operator(&mut operands, operators.pop().unwrap());
                }

                if operators.is_empty() {
                    panic!("Unexpected ','");
                }
            }
        }

        prevkind = Some(kind);
//        prevvalue = Some(value);
    }


    while !operators.is_empty() {
        _apply_operator(&mut operands, operators.pop().unwrap());
    }

    operands[0].clone()
}

pub fn evaluate(expression: &str, variables: IndexMap<String, Data>) -> f64 {
    if expression.is_empty() {
        return 0.into();
    }

    fn is_float(s: &str) -> bool {
        s.parse::<f64>().is_ok()
    }

    if is_float(expression) {
        return expression.parse::<f64>().unwrap();
    }

 //   let variables = variables.unwrap_or_else(HashMap::new);

    // println!("{}", expression);

    let tokens = _tokenize(expression);

    let result = match std::panic::catch_unwind(|| _evaluate(tokens, variables)) {
        Ok(result) => {
            result
        }
        Err(e) => {
            panic!("SyntaxError: {:?} in expression '{}'", e, expression);
        }
    };
    result
}