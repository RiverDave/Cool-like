
COOL lexer spec so far:
```

digit : digit+
letter : (a-z | A-Z)
Identifiers: (letter+)(digit|_)*
Types : [A-Z] (letter | digit | _)*
Object Identifier : [a-z] (letter | digit | _)*
integer : digit


whitespace : \b | \t | \n | \f
special_null : \0

Self: self
Self_type: SELF_TYPE


single_line_comment : --[^\n]*
#comments (**TODO HOW TO DEFINE This?**)
multi_line_comment : --[^\n]*

#keywords(case insensitive)
/* Keywords (case insensitive except 'true'/'false') */
kw_class: [cC][lL][aA][sS][sS]
kw_else: [eE][lL][sS][eE]                   
kw_false: [fF][aA][lL][sS][eE]               
kw_fi: [fF][iI]                           
kw_if: [iI][fF]                           
kw_in: [iI][nN]                           
kw_inherits: [iI][nN][hH][eE][rR][iI][tT][sS]   ; }
kw_isvoid: [iI][sS][vV][oO][iI][dD]           }
kw_let: [lL][eE][tT]                       
kw_loop: [lL][oO][oO][pP]                   
kw_pool: [pP][oO][oO][lL]                   
kw_then: [tT][hH][eE][nN]                   
kw_while: [wW][hH][iI][lL][eE]               
kw_class: [cC][aA][sS][eE]                   
kw_esac: [eE][sS][aA][cC]                   
kw_new: [nN][eE][wW]                       
kw_of: [oO][fF]                           
kw_not:  [nN][oO][tT]                       
kw_true: [tT][rR][uU][eE]                   



string: "^(eof | special_null | '\n') *" => All except what's inside

#operator related, symbols
op_paren: (
close_paren: )
assign: <-
greater: >
less: <
eq : =
greater than or eq: >=
less than or eq: <=
plus: +
min: -
div: /
delim : ;
eof : EOF


```

I may also considering token related errors, eg: Non-closing strings, I assume I should tag

