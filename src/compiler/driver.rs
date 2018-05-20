use compiler::parser;
use compiler::Lexer;
use std::io;
use std::io::Write;

#[derive(PartialEq, Clone, Debug)]
pub enum Stage {
    AST,
    Tokens,
}

pub fn driver_loop(stage: Stage) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut parser_settings = parser::ParserSettings::default();

    'main: loop {
        print!("> ");
        stdout.flush().unwrap();
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        if input.as_str() == ".quit\n" {
            break;
        }

        // the constructed AST
        let mut ast = Vec::new();
        // tokens left from the previous lines
        let mut prev = Vec::new();
        loop {
            let tokens = Lexer::new(input.as_str()).get_all_tokens();

            if stage == Stage::Tokens {
                println!("{:?}", tokens);
                continue 'main;
            }

            prev.extend(tokens.into_iter());
            let parsing_result =
                parser::parse(prev.as_slice(), ast.as_slice(), &mut parser_settings);
            match parsing_result {
                Ok((parsed_ast, rest)) => {
                    ast.extend(parsed_ast.into_iter());
                    if rest.is_empty() {
                        // parsed a full expression
                        break;
                    } else {
                        prev = rest;
                    }
                }
                Err(message) => {
                    println!("Error occured: {}", message);
                    continue 'main;
                }
            }

            print!(". ");
            stdout.flush().unwrap();
            input.clear();
            stdin.read_line(&mut input).expect("Failed to read line");
        }

        if stage == Stage::AST {
            println!("{:#?}", ast);
            continue;
        }
    }
}
