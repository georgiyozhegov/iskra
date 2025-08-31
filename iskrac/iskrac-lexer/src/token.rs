use iskrac_interner::Symbol;

#[derive(Debug)]
pub enum Token {
    Let,
    Function,
    Do,
    End,
    Mutable,
    While,
    Equal,
    MinusEqual,
    Plus,
    Minus,
    Asterisk,
    Greater,
    OpenRound,
    CloseRound,
    OpenCurly,
    CloseCurly,
    Identifier(Symbol),
    Integer(i128),
    Invisible,
}
