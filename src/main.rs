use logos::Logos;


#[derive(Logos, Debug, PartialEq)]
pub enum Token {

    #[regex("[0-9]+")]
    Num,

    #[regex(r"[ \t\n\f]+")]
    Whitespace,


    // #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    // Identifier,

    //variables - identifiers
    #[regex("[A-Z][a-z0-9_]*")]
    Types,

    #[regex("[a-z][a-z0-9_]*")]
    ObjectIdentifier

}


fn main() {

    let lex = Token::lexer("Type_23");
    lex.into_iter().for_each(|e| {
        println!("{:#?}", e.unwrap());

    })
}
