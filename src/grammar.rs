use logos::{Logos, Skip, Filter};

#[derive(Debug, Clone, PartialEq)]
struct Position {
    col: u64,
    row: u64,
}


#[derive(Debug, PartialEq, Clone)]
pub struct ErrorMetadata {
//TODO, Use lifetime specifier here, hold a &str preferably
    slice: String,
    pos: Position,
    msg: String,
}

#[derive(Debug, Default, Clone, PartialEq)]
//This may be treated like a token in the parser
pub enum LexicalError {
    #[default]
    GenericError,
    InvalidTokenSequence(ErrorMetadata),
}

#[derive(Default)]
pub struct LexerControlLayer<'a> {
    recovery_mode: bool,
    current_invalid_token_slice: Option<&'a str>,
//TODO, Use lifetime specifier here, hold a &str preferably
    invalid_report: String,
    error_type: LexicalError
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexicalError)]
#[logos(extras = LexerControlLayer<'s>)]
pub enum Token {
    // Type identifier: starts with uppercase letter
     #[regex(r"([[:alpha:]_])([[:alnum:]_])*", process_identifier)]
    Identifier,

    //TODO: We should have a generic callback, that adds each token src input to a symbol table on each match
    #[regex(r"[[:digit:]]+", process_number)]
    Number,

    #[regex(r"[ \t\n\f]+", process_whitespace_or_eof)]
    Whitespace,

    #[token("-")]
    Minus,

    #[token("<-")]
    Assign,
}


fn process_identifier(lex: &mut logos::Lexer<Token>) -> Filter<Token> {
    let slice = lex.slice();

    if lex.extras.recovery_mode {
    //The logic here not only applies to id's but also to other tokens caught in between
    //Recovery mode
        return Filter::Skip;
    }
    //return token with metadata as usual here.
    todo!()
}

fn process_number(lex: &mut logos::Lexer<Token>) -> Filter<Token> {
    /* Get next char c from end of this token
     * If c + 1 => num => Generate error(nums can't be followed with a char)
     */

    let slice = lex.slice();

    // Get the next character after this token, if any
    let source = lex.source();
    let remainder_start = lex.span().end;

    if remainder_start < source.len() {
        if let Some(next_char) = source[remainder_start..].chars().next() {
            if slice.chars().next().unwrap().is_ascii_digit()
                && (next_char.is_alphabetic() || next_char == '_')
            {
                //after encountering an error we should implement some kind of recovery so that:
                //Our lexer starts from a valid position back again (whitespace)
                lex.extras.recovery_mode = true;
                lex.extras.invalid_report = format!(
                    "Number {} immediately followed by identifier character '{}'",
                    slice, next_char
                );
                // lex.extras.current_invalid_token_slice = 

                return Filter::Skip;
            }
        }
    }
    //TODO: Return valid token metadata here
    Filter::Emit(Token::Number)
}

//This function applies to all whitespace tokens
fn process_whitespace_or_eof(lex: &mut logos::Lexer<Token>) -> Result<Skip, LexicalError> {
    if lex.extras.recovery_mode {
        lex.extras.recovery_mode = false;
        //TODO: emit invalid token metadata here...
        
        //NOTE: Why are we cloning this?
        return Err(lex.extras.error_type.clone());
    }

    Ok(Skip)
}

//TODO: Evaluate the need for this function, for now errors are left behind after
//we reach end of input, fix that
pub fn untangle_remaining_error(lex: &mut logos::Lexer<Token>) -> Result<(), LexicalError> {
    if lex.extras.recovery_mode {
        //In theory we expect only one error to be present
        //TODO: Add metadata to the error type
        return Err(lex.extras.error_type.clone());
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn special_symbols() {
        let mut lex = Token::lexer("<--");
        assert_eq!(lex.next().unwrap(), Ok(Token::Assign));
        assert_eq!(lex.slice(), "<-");
        assert_eq!(lex.next().unwrap(), Ok(Token::Minus));
        assert_eq!(lex.slice(), "-");
    }

    #[test]
    fn sequential_disambiguation() {
        //In theory we should expect this whole input to be labelled as invalid
        //Weirdly enough Logos handles this very weirdly -> It seems to apply the
        //maximum bunch principle and lexes => {number, identifier} when it should => {invalid}

        let mut lex = Token::lexer("23a");
        assert!(lex.next().unwrap().is_err());
        assert!(lex.next().is_none()); // end of iteration
    }
}
