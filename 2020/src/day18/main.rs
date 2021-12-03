use eyre::Result;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Plus,
    Mult,
    Num,
    LeftParen,
    RightParen,
    Space,
    Invalid,
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    value: String,
}

impl Token {
    fn new(tok_str: String) -> Token {
        let token_type = match tok_str.as_str() {
            " " => TokenType::Space,
            "+" => TokenType::Plus,
            "*" => TokenType::Mult,
            "(" => TokenType::LeftParen,
            ")" => TokenType::RightParen,
            "\n" => TokenType::Invalid,
            _ => TokenType::Num,
        };
        Token {
            token_type,
            value: tok_str,
        }
    }
}

enum EquationPrecedence {
    Equal,    // all ops have equal precedence
    AddFirst, // addition before multiplication
}

struct Equation {
    tokens: Vec<Token>,
    precedence: EquationPrecedence,
}

impl Equation {
    fn new(precedence: EquationPrecedence, input_str: &str) -> Result<Self> {
        let mut tokens = vec![];
        let re = Regex::new(r"([\(]+)([0-9]*)|([0-9]*)([\)]+)")?;
        for tok_str in input_str.split_whitespace() {
            // only try to make it complicated if there is a sequence of parens and numbers
            if let Some(caps) = re.captures(tok_str) {
                //println!(
                //    "captures: {:?}",
                //    caps.iter().collect::<Vec<Option<regex::Match>>>()
                //);
                if let Some(l_parens) = caps.get(1) {
                    //println!("found l parens {:?}", l_parens.as_str());
                    for _ in 0..l_parens.as_str().len() {
                        tokens.push(Token::new("(".to_string()));
                    }
                }
                if let Some(num) = caps.get(2) {
                    tokens.push(Token::new(num.as_str().to_string()));
                }
                if let Some(num) = caps.get(3) {
                    tokens.push(Token::new(num.as_str().to_string()));
                }
                if let Some(r_parens) = caps.get(4) {
                    //println!("found r parens {:?}", r_parens.as_str());
                    for _ in 0..r_parens.as_str().len() {
                        tokens.push(Token::new(")".to_string()));
                    }
                }
            } else {
                tokens.push(Token::new(tok_str.to_string()));
            }
        }
        Ok(Equation { tokens, precedence })
    }

    fn has_greater_precedence_than(&self, token_type: &TokenType, other: &TokenType) -> bool {
        match self.precedence {
            EquationPrecedence::Equal => true,
            EquationPrecedence::AddFirst => token_type == &TokenType::Plus || token_type == other,
        }
    }

    fn to_postfix(&self) -> VecDeque<Token> {
        let mut op_stack: Vec<Token> = vec![];
        let mut output_queue = VecDeque::new();
        for tok in &self.tokens {
            //println!("token is {:?}", tok.value);
            match tok.token_type {
                TokenType::Num => {
                    output_queue.push_back(tok.clone());
                }
                TokenType::Mult | TokenType::Plus => {
                    if let Some(mut top_op) = op_stack.last() {
                        while self.has_greater_precedence_than(&top_op.token_type, &tok.token_type)
                            && top_op.token_type != TokenType::LeftParen
                        {
                            output_queue.push_back(op_stack.pop().unwrap().clone());
                            if let Some(next_top_op) = op_stack.last() {
                                top_op = next_top_op;
                            } else {
                                break;
                            }
                        }
                    }
                    op_stack.push(tok.clone());
                }
                TokenType::LeftParen => {
                    op_stack.push(tok.clone());
                }
                TokenType::RightParen => {
                    let mut top_op = op_stack.last().unwrap();
                    while top_op.value != "(" {
                        output_queue.push_back(op_stack.pop().unwrap().clone());
                        top_op = op_stack.last().unwrap();
                    }
                    if top_op.value == "(" {
                        op_stack.pop();
                    }
                }
                _ => (),
            }
        }
        while op_stack.is_empty() {
            output_queue.push_back(op_stack.pop().unwrap().clone());
        }
        //println!(
        //    "final postfix: {:?}",
        //    output_queue
        //        .iter()
        //        .map(|t| t.value.clone())
        //        .collect::<Vec<String>>()
        //);
        output_queue
    }

    fn eval(&self) -> Token {
        let pf = self.to_postfix();
        let mut stack = vec![];
        for token in pf {
            match token.token_type {
                TokenType::Num => {
                    stack.push(token.clone());
                }
                TokenType::Plus => {
                    let num1 = stack.pop().unwrap().value.parse::<u64>().unwrap();
                    let num2 = stack.pop().unwrap().value.parse::<u64>().unwrap();
                    //println!("{} + {} = {}", num2, num1, num1 + num2);
                    stack.push(Token::new((num1 + num2).to_string()));
                }
                TokenType::Mult => {
                    let num1 = stack.pop().unwrap().value.parse::<u64>().unwrap();
                    let num2 = stack.pop().unwrap().value.parse::<u64>().unwrap();
                    //println!("{} * {} = {}", num2, num1, num1 * num2);
                    stack.push(Token::new((num1 * num2).to_string()));
                }
                _ => eprintln!("error maytey: {:?}", token),
            }
        }
        stack.pop().unwrap()
    }
}

fn main() -> Result<()> {
    let input = read_to_string("src/day18/input.txt")?;
    let mut sum_result = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let equation = Equation::new(EquationPrecedence::Equal, &line)?;
        let result = equation.eval();
        //println!("got result: {}", result.value);
        sum_result += result.value.parse::<u64>()?;
    }
    println!("final sum result: {}", sum_result);

    println!("part2");
    let mut sum_result = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let equation = Equation::new(EquationPrecedence::AddFirst, &line)?;
        let result = equation.eval();
        //println!("got result: {}", result.value);
        sum_result += result.value.parse::<u64>()?;
    }
    println!("final sum result: {}", sum_result);
    Ok(())
}
