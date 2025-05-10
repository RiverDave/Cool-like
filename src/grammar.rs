use logos::Logos;

#[derive(Debug, Clone, PartialEq)]
struct Position {
    col: u64,
    row: u64,
}

//TODO, Use lifetime specifier here, hold a &str preferably
#[derive(Debug, PartialEq, Clone)]
pub struct ErrorMetadata {
    src: String,
    pos: Position,
    msg: String,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum LexicalError {
    #[default]
    GenericError,
    InvalidTokenSequence(ErrorMetadata),
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexicalError)]
pub enum Token {
    //TODO: We should have a generic callback, that adds each token src input to a symbol table on each match
    #[regex(r"[[:digit:]]+", validate_token)]
    Number,

    #[regex(r"([[:alpha:]_])([[:alnum:]_])*")]
    Identifier,

    #[regex(r"[ \t\n\f]+")]
    Whitespace,
}

fn validate_token(lex: &mut logos::Lexer<Token>) -> Result<(), LexicalError> {
    /* Get next char c from end of this token
     * If c + 1 => num => Generate error(nums can't be followed with a char)
     */

    let slice = lex.slice();

    // Get the next character after this token, if any
    let source = lex.source();
    let remainder_start = lex.span().end;

    if remainder_start < source.len() {
        // Peek at the first character after current token
        if let Some(next_char) = source[remainder_start..].chars().next() {
            // If we just matched a number and next char is start of identifier, report error
            if slice.chars().next().unwrap().is_ascii_digit()
                && (next_char.is_alphabetic() || next_char == '_')
            {
                //after encountering, that error we should implement some kind of recovery so that:
                //Our lexer starts from a valid position back again
                return Err(LexicalError::InvalidTokenSequence(ErrorMetadata {
                    msg: format!(
                        "Number {} immediately followed by identifier character '{}'",
                        slice, next_char
                    ),
                    src: slice.to_string(),
                    pos: Position { col: 0, row: 0 },
                }));
            }
        }
    }

    Ok(())
}
