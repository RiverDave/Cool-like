mod grammar;
use logos::Logos;
use crate::grammar::Token;


//TODO: We should have a generic callback, that adds text to a symbol table on each match

//TODO: Move this to a separate file later, All core
//components of this compilers must be separated from main


fn main() {

    //Logos seems to match every possible maximum regex per token
    //We need to wrap that API somehow so that it can yield lexemes accurate
    //to our requirements.
    let lexical_placeholder = Token::lexer("23a");
    lexical_placeholder.into_iter().for_each(|e| {
        println!("{:?}", e);
    });
}
