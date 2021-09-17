```ebnf

<fundef> ::= "fun" "(" ( <ident> ":" <typename> "," )* <ident> ? ")" ( ":" <typename> )? <body>
<structdef> ::= "struct" <ident> "{" ( <ident> ":" <typename> "," )* ( <ident> ":" <typename> )? "}"

<body> ::= "{" ( <stmt> | <expr> ";" )* "}"

<breakstmt> ::= "break" 
<returnstmt> ::= "return" <expr>?
<whilestmt> ::= "while" <expr> <body>
<ifstmt> ::= "if" <expr> <body> ( "else" <body> )?

<expr> ::= <binexpr> | <prefixexpr> | <op> <expr> | <assignexpr>

<assignexpr> ::= <prefixexpr> ":=" <expr>

<prefixexpr> ::= <funcall> | <literal> | "(" <expr> ")" | <variable> | <memberexpr> | <prefixexpr> "[" <expr> "]"

<funcall> ::= <prefixexpr> "(" ( <expr> "," )* <expr>? ")"

<binexpr> ::= <expr> <op> <expr>

<variable> ::= <ident>

<literal> ::= <charliteral> | <arrayliteral> | <strliteral> | <boolliteral> | <numliteral> 

<charliteral> ::= "'" <letter>+ "'"
<arrayliteral> ::= "[" ( <expr> "," )* <expr>? "]"
<strliteral> ::= "\"" <letter>* "\""
<boolliteral> ::= "true" | "false"
<numliteral> ::= <digit> ( <digit> | "x" | "b" )*

<typename> ::= "bool" | <arraytype> | <inttype> | <ident> | "float" 
<arraytype> ::= "[" <typename> "," <numliteral> "]"
<inttype> ::= ( "i" | "u" ) ( "8" | "16" | "32" | "64" )

<op> ::= "+" | "-" | "*" | "/" | "%"
      |  "&" | "|" | "^" | "~" 
      |  "&&" | "||" | "!" | "=="
      | <op> "="

<ident> ::= ( <letter> | "_" ) ( <letter> | "_" | <digit> )*
<digit> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
<letter> ::= "A" | "B" | "C" | "D" | "E" | "F" | "G"
       | "H" | "I" | "J" | "K" | "L" | "M" | "N"
       | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
       | "V" | "W" | "X" | "Y" | "Z" | "a" | "b"
       | "c" | "d" | "e" | "f" | "g" | "h" | "i"
       | "j" | "k" | "l" | "m" | "n" | "o" | "p"
       | "q" | "r" | "s" | "t" | "u" | "v" | "w"
       | "x" | "y" | "z" 
```
