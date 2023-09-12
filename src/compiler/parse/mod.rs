use logos::Logos;

#[derive(Debug, Logos)]
#[logos(skip r"[ \t\r\n\f]+")]
enum Token {
    #[token("=")]
    Assign,

    #[token(".")]
    Dot,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,
}
