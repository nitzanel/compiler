use compiler::ast::*;
use compiler::{BinaryOp, Token};
use std::collections::HashMap;

/* Grammar:
 * program          : [[statement | expression] Delimiter ? ]*;
 * statement        : [declaration | definition];
 * declaration      : Extern prototype;
 * definition       : Def prototype expression;
 * prototype        : Ident OpeningParenthesis [Ident Comma ?]* ClosingParenthesis;
 * expression       : [primary_expr (Op primary_expr)*];
 * primary_expr     : [Ident | Number | call_expr | parenthesis_expr];
 * call_expr        : Ident OpeningParenthesis [expression Comma ?]* ClosingParenthesis;
 * parenthesis_expr : OpeningParenthesis expression ClosingParenthesis;
 */

pub type ParsingResult = Result<(Vec<ASTNode>, Vec<Token>), String>;

pub struct ParserSettings {
    operator_precedence: HashMap<BinaryOp, i32>,
}

impl Default for ParserSettings {
    fn default() -> ParserSettings {
        let mut operator_precedence = HashMap::new();
        operator_precedence.insert(BinaryOp::Shift, 10);
        operator_precedence.insert(BinaryOp::Add, 20);
        operator_precedence.insert(BinaryOp::Sub, 20);
        operator_precedence.insert(BinaryOp::Mul, 40);
        operator_precedence.insert(BinaryOp::Div, 40);
        ParserSettings {
            operator_precedence,
        }
    }
}

pub enum PartParsingResult<T> {
    Good(T, Vec<Token>),
    NotComplete,
    Bad(String),
}

pub fn parse(
    tokens: &[Token],
    parsed_tree: &[ASTNode],
    settings: &mut ParserSettings,
) -> ParsingResult {
    let mut rest = tokens.to_vec();
    rest.reverse();

    let mut ast = parsed_tree.to_vec();

    loop {
        let current_token = match rest.last() {
            Some(token) => token.clone(),
            None => break,
        };

        let result = match current_token {
            Token::Def => parse_function(&mut rest, settings),
            Token::Extern => parse_extern(&mut rest, settings),
            Token::Delimeter => {
                rest.pop();
                continue;
            }
            _ => parse_expression(&mut rest, settings),
        };

        match result {
            PartParsingResult::Good(ast_node, _) => ast.push(ast_node),
            PartParsingResult::NotComplete => break,
            PartParsingResult::Bad(message) => return Err(message),
        }
    }

    // unparsed tokens
    rest.reverse();
    Ok((ast, rest))
}

fn error<T>(message: &str) -> PartParsingResult<T> {
    PartParsingResult::Bad(message.to_string())
}

macro_rules! parse_try (
    ($function:ident, $tokens:ident, $settings:ident, $parsed_tokens:ident) => (
        parse_try!($function, $tokens, $settings, $parsed_tokens,)
    );

    ($function:ident, $tokens:ident, $settings:ident, $parsed_tokens:ident, $($arg:expr), *) => (
        match $function($tokens, $settings, $($arg), *) {
            PartParsingResult::Good(ast, toks) => {
                $parsed_tokens.extend(toks.into_iter());
                ast
            },

            PartParsingResult::NotComplete => {
                $parsed_tokens.reverse();
                $tokens.extend($parsed_tokens.into_iter());
                return PartParsingResult::NotComplete;
            },

            PartParsingResult::Bad(message) => return PartParsingResult::Bad(message),
        }
        )
);

macro_rules! expect_token (
    ([ $($token:pat, $value:expr, $result:stmt);+ ] <= $tokens:ident, $parsed_tokens:ident, $error:expr) => (
        match $tokens.pop () {
            $(
                Some($token) => {
                    $parsed_tokens.push($value);
                    $result
                },
            )+
                None => {
                    $parsed_tokens.reverse();
                    $tokens.extend($parsed_tokens.into_iter());
                    return PartParsingResult::NotComplete;
                },
                _ => return error($error)
        }

    );

    ([ $($token:pat, $value:expr, $result:stmt);+ ] else $not_matched:block <= $tokens:ident, $parsed_tokens: ident) => (
        match $tokens.last().map(|i| {i.clone()}) {
            $(
                Some($token) => {
                    $tokens.pop();
                    $parsed_tokens.push($value);
                    $result
                },
                )+
                _ => {$not_matched}
        }
        )
);

fn parse_extern(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<ASTNode> {
    // eat Extern token
    tokens.pop();
    let mut parsed_tokens = vec![Token::Extern];
    let prototype = parse_try!(parse_prototype, tokens, settings, parsed_tokens);
    PartParsingResult::Good(ASTNode::ExternNode(prototype), parsed_tokens)
}

fn parse_function(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<ASTNode> {
    // eat Def token
    tokens.pop();
    let mut parsed_tokens = vec![Token::Def];
    let prototype = parse_try!(parse_prototype, tokens, settings, parsed_tokens);
    let body = parse_try!(parse_expr, tokens, settings, parsed_tokens);

    PartParsingResult::Good(
        ASTNode::FunctionNode(Function { prototype, body }),
        parsed_tokens,
    )
}

fn parse_prototype(
    tokens: &mut Vec<Token>,
    _settings: &mut ParserSettings,
) -> PartParsingResult<Prototype> {
    let mut parsed_tokens = Vec::new();

    let name = expect_token!(
        [Token::Identity(name), Token::Identity(name.clone()), name] <= tokens,
        parsed_tokens,
        "expected function name in prototype"
    );

    expect_token!(
        [Token::LParen, Token::LParen, ()] <= tokens,
        parsed_tokens,
        "expected '(' in prototype"
    );

    let mut args = Vec::new();
    loop {
        expect_token!([
              Token::Identity(arg), Token::Identity(arg.clone()), args.push(arg.clone());
              Token::Comma, Token::Comma, continue;
              Token::RParen, Token::RParen, break
        ] <= tokens, parsed_tokens, "expected ')' in prototype");
    }

    PartParsingResult::Good(Prototype { name, args }, parsed_tokens)
}

fn parse_expression(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<ASTNode> {
    let mut parsed_tokens = Vec::new();
    let expression = parse_try!(parse_expr, tokens, settings, parsed_tokens);
    let prototype = Prototype {
        name: "".to_string(),
        args: vec![],
    };
    let lambda = Function {
        prototype: prototype,
        body: expression,
    };
    PartParsingResult::Good(ASTNode::FunctionNode(lambda), parsed_tokens)
}

fn parse_primary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<Expression> {
    match tokens.last().cloned() {
        Some(Token::Identity(_)) => parse_ident_expr(tokens, settings),
        Some(Token::Number(_)) => parse_literal_expr(tokens, settings),
        Some(Token::LParen) => parse_parenthesis_expr(tokens, settings),
        Some(Token::Let) => parse_assignment_expr(tokens, settings),
        None => return PartParsingResult::NotComplete,
        token => error(format!("unknown token {:?} when expecting an expression.", token).as_str()),
    }
}

fn parse_assignment_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    expect_token!(
        [Token::Let, Token::Let, ()]
        else { return PartParsingResult::Bad("Expected let keyword".to_string())}
        <= tokens, parsed_tokens
        );

    let ident = if let Expression::VariableExpr(name) = parse_try!(parse_ident_expr, tokens, settings, parsed_tokens)  {
            name
        } else {
            return PartParsingResult::Bad("Expected identifactor after let keyword".to_string());
        };

    expect_token!(
        [Token::Assign, Token::Assign, ()]
        else { return PartParsingResult::Bad("Expected = token".to_string())}
        <= tokens, parsed_tokens
        );

    let assign_to = Box::new(parse_try!(parse_expr, tokens, settings, parsed_tokens));

    PartParsingResult::Good(Expression::AssignmentOp(ident, assign_to), parsed_tokens)
}

fn parse_ident_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let name = expect_token!(
        [Token::Identity(name), Token::Identity(name.clone()), name] <= tokens,
        parsed_tokens,
        "identificator expected"
    );

    expect_token!(
        [Token::LParen, Token::LParen, ()]
        else { return PartParsingResult::Good(Expression::VariableExpr(name), parsed_tokens)}
        <= tokens, parsed_tokens);

    let mut args = Vec::new();
    loop {
        expect_token!(
                [Token::RParen, Token::RParen, break; Token::Comma, Token::Comma, continue]
                else {
                    args.push(parse_try!(parse_expr, tokens, settings, parsed_tokens));
                }
                <= tokens, parsed_tokens);
    }

    PartParsingResult::Good(Expression::CallExpr(name, args), parsed_tokens)
}

fn parse_literal_expr(
    tokens: &mut Vec<Token>,
    _settings: &mut ParserSettings,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let value = expect_token!(
        [Token::Number(val), Token::Number(val), val] <= tokens,
        parsed_tokens,
        "literal expected"
    );

    PartParsingResult::Good(Expression::LiteralExpr(value), parsed_tokens)
}

fn parse_parenthesis_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<Expression> {
    // eat the opening parenthesis
    tokens.pop();
    let mut parsed_tokens = vec![Token::LParen];

    let expr = parse_try!(parse_expr, tokens, settings, parsed_tokens);

    expect_token!(
        [Token::RParen, Token::RParen, ()] <= tokens,
        parsed_tokens,
        "')' expected"
    );

    PartParsingResult::Good(expr, parsed_tokens)
}

fn parse_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();
    let lhs = parse_try!(parse_primary_expr, tokens, settings, parsed_tokens);
    let expr = parse_try!(parse_binary_expr, tokens, settings, parsed_tokens, 0, &lhs);

    PartParsingResult::Good(expr, parsed_tokens)
}

fn parse_binary_expr(
    tokens: &mut Vec<Token>,
    settings: &mut ParserSettings,
    expr_precedence: i32,
    lhs: &Expression,
) -> PartParsingResult<Expression> {
    let mut result = lhs.clone();
    let mut parsed_tokens = Vec::new();

    loop {
        let (operator, precedence) = match tokens.last() {
            Some(&Token::BinOp(ref op)) => match settings.operator_precedence.get(op) {
                Some(pr) if *pr >= expr_precedence => (op.clone(), *pr),
                None => return error("unknown operator found"),
                _ => break,
            },

            _ => break,
        };

        tokens.pop();
        parsed_tokens.push(Token::BinOp(operator.clone()));

        let mut rhs = parse_try!(parse_primary_expr, tokens, settings, parsed_tokens);
        // parse all ths operator untill their precedence is \
        // bigger then the current one
        loop {
            let binary_rhs = match tokens.last().cloned() {
                Some(Token::BinOp(ref op)) => {
                    match settings.operator_precedence.get(op).map(|i| *i) {
                        Some(pr) if pr > precedence => {
                            parse_try!(parse_binary_expr, tokens, settings, parsed_tokens, pr, &rhs)
                        }
                        None => return error("unknown operator found"),
                        _ => break,
                    }
                }
                _ => break,
            };

            rhs = binary_rhs;
        }

        result = Expression::BinaryExpr(operator, box result, box rhs);
    }

    PartParsingResult::Good(result, parsed_tokens)
}

#[cfg(test)]
mod tests {
    use self::Token::*;
    use super::*;

    #[test]
    fn test_function_dec_statement() {
        let mut parser_settings = ParserSettings::default();
        let tokens = vec![
            Def,
            Identity("test".to_string()),
            LParen,
            Identity("x".to_string()),
            RParen,
            Identity("x".to_string()),
        ];
        let result = parse(
            tokens.as_slice(),
            Vec::new().as_slice(),
            &mut parser_settings,
        );
        let expected_left_over_tokens = Vec::new();
        let expected_function = Function {
            prototype: Prototype {
                name: "test".to_string(),
                args: vec!["x".to_string()],
            },
            body: Expression::VariableExpr("x".to_string()),
        };
        let expected_ast = vec![ASTNode::FunctionNode(expected_function)];
        let expected = Ok((expected_ast, expected_left_over_tokens));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_extern_decleration() {
        let mut parser_settings = ParserSettings::default();
        let tokens = vec![
            Extern,
            Identity("sin".to_string()),
            LParen,
            Identity("x".to_string()),
            RParen,
        ];
        let result = parse(
            tokens.as_slice(),
            Vec::new().as_slice(),
            &mut parser_settings,
        );

        let expected_tokens = vec![];
        let expected_ast = vec![ASTNode::ExternNode(Prototype {
            name: "sin".to_string(),
            args: vec!["x".to_string()],
        })];

        let expected = Ok((expected_ast, expected_tokens));

        assert_eq!(expected, result);
    }
}
