use logos::{Filter, FilterResult, Logos, Skip};

#[derive(Debug, Clone, PartialEq)]
struct Position {
    col: usize,
    row: usize,
}

impl Default for Position {
    fn default() -> Self {
        return Self { col: 1, row: 1 };
    }
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
pub struct LexerControlLayer {
    recovery_mode: bool,
    comment_mode: bool,
    //Ideally a slice, however cannot due considering rust limitations
    //(Can't concat slices)
    current_invalid_token_slice: Option<String>,
    current_report: String,
    error_type: LexicalError,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexicalError)]
#[logos(extras = LexerControlLayer)]
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

fn process_identifier(lex: &mut logos::Lexer<Token>) -> FilterResult<Token, LexicalError> {
    let slice = lex.slice();
    let src = lex.source();

    if lex.extras.recovery_mode {
        //NOTE: The following logic here not only applies to id's but also to other tokens caught in between
        //Recovery mode,

        let remainder_start = lex.span().end;
        //FIXME: This should be removed in the case we can mutate our lexer after the end of the input

        let prev_slice = lex
            .extras
            .current_invalid_token_slice
            .as_deref()
            .unwrap_or_default();

        let mut new_invalid_str = prev_slice.to_string();
        new_invalid_str.push_str(slice);
        lex.extras.current_invalid_token_slice = Some(new_invalid_str);

        //current invalid token in recovery mode reached end of input, dump token
        if src[remainder_start..].chars().next().is_none() {
            let err = ErrorMetadata {
                slice: lex
                    .extras
                    .current_invalid_token_slice
                    .take()
                    .unwrap_or_default(),
                pos: Position::default(),
                msg: lex.extras.current_report.to_string(),
            };
            return FilterResult::Error(LexicalError::InvalidTokenSequence(err));
        }

        //TODO: add more metadata
        return FilterResult::Skip;
    }

    //return token with metadata as usual here.
    FilterResult::Emit(Token::Identifier)
}

fn process_number(lex: &mut logos::Lexer<Token>) -> Filter<Token> {
    /* Get next char c from end of this token
     * If c + 1 => num => Generate error(nums can't be followed with a char)
     */

    let slice = lex.slice();


    let source = lex.source();
    let remainder_start = lex.span().end;

    // Get the next character after this token, if any, If it's an alphanumeric or _, we should
    // enter recovery mode => Since we can't have a number followed by an alphanumeric or _
    if remainder_start < source.len() {
        if let Some(next_char) = source[remainder_start..].chars().next() {
            if slice.chars().next().unwrap().is_ascii_digit()
                && (next_char.is_alphabetic() || next_char == '_')
            {
                lex.extras.recovery_mode = true;
                lex.extras.current_report = format!(
                    "Number {} immediately followed by identifier character '{}'",
                    slice, next_char
                );
                lex.extras.current_invalid_token_slice = Some(slice.to_string());

                //handle end of input
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
// pub fn untangle_remaining_error(lex: &mut logos::Lexer<Token>) -> Result<(), LexicalError> {
//     if lex.extras.recovery_mode {
//         //In theory we expect only one error to be present
//         //TODO: Add metadata to the error type
//         return Err(lex.extras.error_type.clone());
//     }
//
//     Ok(())
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_identifiers() {
        let mut lex = Token::lexer("123-123");
        assert_eq!(lex.next().unwrap(), Ok(Token::Number));
        assert_eq!(lex.slice(), "123");
        assert_eq!(lex.next().unwrap(), Ok(Token::Minus));
        assert_eq!(lex.slice(), "-");
        assert_eq!(lex.next().unwrap(), Ok(Token::Number));
        assert_eq!(lex.slice(), "123");
        assert!(lex.next().is_none());
    }

    #[test]
    fn arithmetic_tokens() {
        let mut lex = Token::lexer("identifier 123");
        assert_eq!(lex.next().unwrap(), Ok(Token::Identifier));
        assert_eq!(lex.slice(), "identifier");
        assert_eq!(lex.next().unwrap(), Ok(Token::Number));
        assert_eq!(lex.slice(), "123");
        assert!(lex.next().is_none());
    }

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

        let mut lex = Token::lexer("23a ");
        assert!(lex.next().unwrap().is_err());
        assert!(lex.next().is_none()); // end of iteration
    }

    #[test]
    fn test_end_of_input_error() {
        let mut lex = Token::lexer("123a");
        assert!(lex.next().unwrap().is_err());

        // Check no more tokens
        assert!(lex.next().is_none());

        // Check for pending errors
        // assert!(check_for_pending_errors(&mut lex).is_ok());
    }
}
