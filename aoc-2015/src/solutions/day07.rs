use super::utils;
use std::collections::HashMap;

const DAY: i32 = 7;

pub fn solve(use_dummy_inputs: bool) {
    utils::solve(part01, part02, DAY, use_dummy_inputs)
}

// Part A of mini-piler (not quite a full compiler 😂 just map the stringy bits to a defined representation (token).
// Which then moves nicely to part B where we take the tokens & build + evaluate expressions
// Likely will use a RIGHT parse tree (evaluate expressions right -> left)
// For example:
//      `x AND y -> b` is one expression
// parsing from right we get two expressions:
//      `x AND y ->` and `b`
// Evaluating both sides we get

// The general idea is we need to split and logically connect operations (and, or, not, ...) from
// their operands / arguments, (123, a, 456, ...)

#[derive(Debug)]
enum Token {
    Variable(String),
    Value(i16),
    AND,
    OR,
    NOT,
    LSHIFT,
    RSHIFT,
    MOV,
}

fn tokenize(script: String) -> Vec<Vec<Token>> {
    // Take a string of commands (aka script) parse out the lines + operations & operands
    let mut script_tokenised: Vec<Vec<Token>> = Vec::new();

    // Each line is a complete expression
    for line in script.lines() {
        let mut tokens: Vec<Token> = Vec::new();

        for token_str in line.split_whitespace() {
            // Here we either detect a keyword which is easy to match, or assign None if it's a variable / value
            // so we can parse that later
            let pre_token: Option<Token> = match token_str {
                "AND" => Option::Some(Token::AND), // This null handling stuff is quite interesting...
                "OR" => Option::Some(Token::OR),
                "NOT" => Option::Some(Token::NOT),
                "LSHIFT" => Option::Some(Token::LSHIFT),
                "RSHIFT" => Option::Some(Token::RSHIFT),
                "->" => Option::Some(Token::MOV),
                _ => Option::None,
            };

            // Second part of parsing where we either transparently return the keyword, or for other types
            // pull out the values and return that.
            let token: Token = match pre_token {
                Some(x) => x,
                None => match token_str.parse::<i16>() {
                    Ok(x) => Token::Value(x),
                    // We should ideally validate the variable name, but cba as I know inputs should be consistent
                    Err(_) => Token::Variable(token_str.to_string()),
                },
            };

            tokens.push(token);
        }
        script_tokenised.push(tokens);
    }

    script_tokenised
}

fn parse(tokens: &Vec<Token>) -> ParseTree {
    panic!("Not implemented");
}

struct ParseTree {
    l: ParseNode,
    r: ParseNode,
}

struct ParseNode {
    stack: Program,
    token: Token,
}
trait Expression {
    fn get_value(&self) -> i16;
}

impl Expression for ParseNode {
    fn get_value(&self) -> i16 {
        match self.token {
            Token::Value(x) => x,
            _ => panic!(
                "Operation <get_value> undefined on token type {:?}",
                self.token
            ),
        }
    }
}

struct Program {
    // Using global stack where we never push / pop so every operation occurs within the same frame
    register: HashMap<String, i16>,
}

impl Program {
    // Memory access
    fn get_var(&self, var_name: String) -> i16 {
        if !self.register.contains_key(&var_name) {
            panic!(
                "Error accessing variable \"{}\". Has it been assigned to yet?",
                var_name
            );
        }
        *self.register.get(&var_name).unwrap()
    }

    fn assign(&mut self, var_name: String, value: i16) {
        self.register.insert(var_name, value);
    }

    // Define operations
    fn and<E: Expression>(lhs: &E, rhs: &E) -> i16 {
        lhs.get_value() & rhs.get_value()
    }

    fn or<E: Expression>(lhs: &E, rhs: &E) -> i16 {
        lhs.get_value() | rhs.get_value()
    }

    fn not<E: Expression>(val: &E) -> i16 {
        !val.get_value()
    }

    fn lshift<E: Expression>(lhs: &E, rhs: &E) -> i16 {
        lhs.get_value() << rhs.get_value()
    }

    fn rshift<E: Expression>(lhs: &E, rhs: &E) -> i16 {
        lhs.get_value() >> rhs.get_value()
    }
}

fn part01(_inputs: &str) {
    let tokens = tokenize(_inputs.to_string());
    dbg!(tokens);
}

fn part02(_inputs: &str) {
    println!("unimplemented");
}
