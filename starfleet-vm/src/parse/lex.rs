//! Module containing the [Lexer] struct, used to transform an input source
//! string into a token stream which can then be parsed into an Abstract Syntax
//! Tree
use std::{str::{CharIndices, FromStr}, iter::Peekable, num::NonZeroU32, fmt};

#[derive(Debug, Clone)]
pub struct Lexer<'src> {
    /// An iterator over the characters in the source file
    src: CharStream<'src>,
}

impl<'src> Lexer<'src> {
    /// Create a new `Lexer` from the given source string
    pub fn new(source: &'src str) -> Self {
        Self {
            src: CharStream::new(source)
        }
    }
    
    /// Lex the next token from the source string
    pub fn tok(&mut self) -> Option<Token> {
        self.src.skip_whitespace();
        let next = self.src.next()?;
        Some(match next {
            '"' => Token(self.src.loc(), TokTy::Quote(QuoteTy::Double)),
            '\'' => Token(self.src.loc(), TokTy::Quote(QuoteTy::Single)),
            '`' => Token(self.src.loc(), TokTy::Quote(QuoteTy::Tilde)),

            '{' => Token(self.src.loc(), TokTy::OpenBrace(BraceTy::Squiggly)),
            '(' => Token(self.src.loc(), TokTy::OpenBrace(BraceTy::Smooth)),
            '[' => Token(self.src.loc(), TokTy::OpenBrace(BraceTy::Square)),

            '}' => Token(self.src.loc(), TokTy::CloseBrace(BraceTy::Squiggly)),
            ')' => Token(self.src.loc(), TokTy::CloseBrace(BraceTy::Smooth)),
            ']' => Token(self.src.loc(), TokTy::CloseBrace(BraceTy::Square)),
            
            '.' => Token(self.src.loc(), TokTy::Dot),
            ',' => Token(self.src.loc(), TokTy::Comma),
            ';' => Token(self.src.loc(), TokTy::Semicolon),
            ':' => Token(self.src.loc(), TokTy::Colon),

            '+' | '-' | '*' | '/' | '%' | 
            '&' | '|' | '^' | '~' | 
            '>' | '<' |
            '!' => {
                let op = if let Some(c) = self.src.peek() {
                    match (next, c) {
                        ('&', '&') => {
                            self.src.next();
                            Op::AndAnd
                        },
                        ('|', '|') => {
                            self.src.next(); 
                            Op::OrOr
                        },
                        ('>', '>') => { 
                            self.src.next();
                            Op::ShRight
                        },
                        ('<', '<') => {
                            self.src.next();
                            Op::ShLeft
                        },
                        ('<', '=') => {
                            self.src.next();
                            Op::LessEq
                        },
                        ('>', '=') => {
                            self.src.next();
                            Op::GreaterEq
                        },
                        _ => match next {
                            '+' => Op::Add,
                            '-' => Op::Sub,
                            '*' => Op::Mul,
                            '/' => Op::Div,
                            '%' => Op::Mod,
    
                            '&' => Op::AND,
                            '|' => Op::OR,
                            '^' => Op::XOR,
                            '~' => Op::INV,
    
                            '>' => Op::Greater,
                            '<' => Op::Less,
                            '!' => Op::Not,
                            _ => unreachable!()
                        }
                    }
                } else {
                    match next {
                        '+' => Op::Add,
                        '-' => Op::Sub,
                        '*' => Op::Mul,
                        '/' => Op::Div,
                        '%' => Op::Mod,

                        '&' => Op::AND,
                        '|' => Op::OR,
                        '^' => Op::XOR,
                        '~' => Op::INV,

                        '>' => Op::Greater,
                        '<' => Op::Less,
                        '!' => Op::Not,
                        _ => unreachable!()
                    }
                };
                Token(self.src.loc(), TokTy::Op(op))
            },
            c if c.is_alphabetic() => {
                let ident = self.src.slice_while(|c| c.is_alphanumeric() || *c == '_');
                let ident = match ident {
                    Some(rest) => String::from(c) + rest,
                    None => String::from(c)
                };
                match Key::from_str(ident.as_str()) {
                    Ok(key) => Token(self.src.loc(), TokTy::Key(key)),
                    Err(()) => Token(self.src.loc(), TokTy::Ident(ident))
                }
            }
            c if c.is_numeric() => {
                let num = self.src.slice_while(|c| c.is_alphanumeric() || *c == 'b' || *c == 'x');
                let num = match num {
                    Some(rest) => String::from(c) + rest,
                    None => String::from(c)
                };
                Token(self.src.loc(), TokTy::Num(num))
            }

            _ => return self.tok(),
        })
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.tok()
    }
}

/// One token, lexed from a source string
#[derive(Clone, Debug,)]
pub struct Token(pub CodeLoc, pub TokTy);

/// An enum representing all types of tokens lexed by the lexer
#[derive(Clone, Debug)]
pub enum TokTy {
    OpenBrace(BraceTy),
    CloseBrace(BraceTy),

    Dot,
    Comma,
    Colon,
    Semicolon,
    
    Quote(QuoteTy),
    Ident(String),
    Num(String),
    Op(Op),
    Key(Key)
}

/// All binary and unary operators
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div, 
    Mod,
    
    XOR,
    AND,
    OR,
    INV,
    ShRight,
    ShLeft,
    
    Less,
    Greater,
    Eq,
    LessEq,
    GreaterEq,
    AndAnd,
    OrOr,
    
    Not,    
}

/// Every keyword in arc
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    /// Declare a new local variable
    Let,
    /// Conditional statement
    If,
    Else,
    
    /// Function declaration
    Fun,

    /// Return statement
    Return,
    
    /// Loop while a condition is true
    While,
    
    /// Exit a loop
    Break,
}

impl FromStr for Key {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "let" => Self::Let,
            "if"  => Self::If,
            "else" => Self::Else,
            "fun"  => Self::Fun,
            "return" => Self::Return,
            "while" => Self::While,
            "break" => Self::Break,
            _ => return Err(())
        })
    }
}

/// An enum naming all accepted quote types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuoteTy {
    Single,
    Double,
    Tilde
}

/// All types of braces, given names for clarity
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BraceTy {
    Smooth,
    Square,
    Squiggly
}

/// A struct representing line and collumn number in a source file
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodeLoc(NonZeroU32, u32);

impl CodeLoc {
    /// Create a new `CodeLoc` from a line and collumn number
    #[inline(always)]
    pub const fn new(line: NonZeroU32, col: u32) -> Self {
        Self(line, col)
    }
        
    /// Get the line number of this location
    #[inline]
    pub const fn line(&self) -> NonZeroU32 {
        self.0
    }
    
    /// Get the collumn number of this location
    #[inline]
    pub const fn col(&self) -> u32 {
        self.1
    }
}

impl fmt::Display for CodeLoc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

/// An iterator over characters in a source string, which tracks the current position, line number,
/// and collumn of the stream
#[derive(Debug, Clone)]
struct CharStream<'src> {
    /// A reference to the original source string
    source: &'src str,
    /// An iterator over the characters of the source string
    chars: Peekable<CharIndices<'src>>,
    /// The current line number
    line: NonZeroU32,
    /// The current collumn
    col: u32,
}

impl<'src> CharStream<'src> {
    /// Create a new character stream from the given source code string
    pub fn new(source: &'src str) -> Self {
        Self {
            chars: source.char_indices().peekable(),
            source,
            line: unsafe{ NonZeroU32::new_unchecked(1) },
            col: 0,
        }
    }
    
    /// Skip whitespace, leaving the iterator at a position where the next character is either a
    /// non-whitespace or EOF
    pub fn skip_whitespace(&mut self) {
        while match self.chars.peek() {
            Some((_, c)) if c.is_whitespace() => true,
            _ => false
        } {
            self.next_char(); //Consume the whitespace character
        }
    }
    
    /// Take the next character from the stream, incrementing line numbers if a newline is
    /// encountered
    fn next_char(&mut self) -> Option<(usize, char)> {
        match self.chars.next() {
            Some((idx, '\n')) => {
                self.line = NonZeroU32::new(self.line.get() + 1).unwrap();
                self.col = 0;
                Some((idx, '\n'))
            },
            Some((idx, c)) => {
                self.col += 1;
                Some((idx, c))
            },
            None => None
        }
    }
    
    /// Get the location that the iterator is at in the string
    pub const fn loc(&self) -> CodeLoc {
        CodeLoc(self.line, self.col)
    }
    
    /// Take a slice of the input stream so long as `pred` returns `true`.
    /// When it returns `false` or EOF is reached, a slice is returned, unless
    /// `pred` returns `false` on the first character tested, in which case `None` is returned
    pub fn slice_while<F: Fn(&char) -> bool>(&mut self, pred: F) -> Option<&'src str> {
        let mut len = 0usize;
        let start = self.chars.peek()
            .map(|(idx, _)| *idx)
            .unwrap_or(self.source.len()); 
        loop {
            let peeked = match self.chars.peek() {
                Some((_, c)) => c,
                None => break
            };
            if !pred(peeked) {
                break
            }
            self.next_char();
            len += 1;
        }
        match len {
            0 => None,
            _ => Some(&self.source[start..len])
        }
    }
    
    /// Peek the next character in the iterator
    pub fn peek(&mut self) -> Option<&char> {
        self.chars.peek().map(|(_, c)| c)
    }
}

impl Iterator for CharStream<'_> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_char().map(|(_, c)| c)
    }
}
