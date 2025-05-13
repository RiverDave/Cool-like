mod grammar;

use std::fmt::Debug;
use logos::Logos;
use crate::grammar::Token;


//TODO: We should have a generic callback, that adds text to a symbol table on each match

//TODO: Move this to a separate file later, All core
//components of this compilers must be separated from main

fn main() {

    let mut tokens : Vec<Result<Token, grammar::LexicalError>> = vec![];

    let lexical_placeholder = Token::lexer("123identifier a");
    lexical_placeholder.into_iter().for_each(|e| {
        println!("{:?}", e);
        tokens.push(e);
    });
    
}
