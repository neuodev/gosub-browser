use std::fmt::{self, Debug, Formatter};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    At,
    Hash,
    Dot,
    Number,
    String,
    CDO,
    CDC,
    Semicolon,
    Colon,
    LCurly,
    RCurly,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comment,
    /// A `~=` [`<include-match-token>`](https://drafts.csswg.org/css-syntax/#include-match-token-diagram)
    IncludeMatch,
    /// A `|=` [`<dash-match-token>`](https://drafts.csswg.org/css-syntax/#dash-match-token-diagram)
    DashMatch,
    /// A `^=` [`<prefix-match-token>`](https://drafts.csswg.org/css-syntax/#prefix-match-token-diagram)
    PrefixMatch,
    /// A `$=` [`<suffix-match-token>`](https://drafts.csswg.org/css-syntax/#suffix-match-token-diagram)
    SuffixMatch,
    /// A `*=` [`<substring-match-token>`](https://drafts.csswg.org/css-syntax/#substring-match-token-diagram)
    SubstringMatch,
    Equal,
    Ident,
    Important,
    // todo: add unicode & ascii
}

#[derive(PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({})", self.token_type, self.value)
    }
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token { token_type, value }
    }
}

// todo: support float numbers: ^([0-9]*\.[0-9]+)|[0-9]+
static NUMBER: &str = r"^[0-9]+";
static NUM_CHAR: &str = r"[_a-zA-Z0-9-]";
static NM_START: &str = r"[_a-zA-Z]";
static STRING: &str = r#"^(\"|\')([^\n\r\f\\"])*(\"|')"#;
static IDENT: &str = r"^[-]?[_a-zA-Z][_a-zA-Z0-9-]*";
static NAME: &str = r"^[_a-zA-Z0-9-]*";
static HASH: &str = r"^#";
static AT: &str = r"^@";
static DOT: &str = r"^\.";
static SEMICOLON: &str = r"^;";
static COLON: &str = r"^:";
static LCURLY: &str = r"^\{";
static RCURLY: &str = r"^\}";
static LBRACKET: &str = r"^\[";
static RBRACKET: &str = r"^\]";
static INCLUDE_MATCH: &str = r"^\~=";
static DASH_MATCH: &str = r"^\|=";
static PREFIX_MATCH: &str = r"^\^=";
static SUFFIX_MATCH: &str = r"^\$=";
static SUBSTRING_MATCH: &str = r"^\*=";
static EQUAL: &str = r"^=";
static SPACE: &str = r"^[ \t\r\n\f]+";
static IMPORTANT: &str = "^!important";
static COMMENT: &str = r"^\/\*[^*]*\*+([^/*][^*]*\*+)*\/";

pub static TOKEN_REFS: [(&str, Option<TokenType>); 21] = [
    (SPACE, None),
    (COMMENT, None),
    (NUMBER, Some(TokenType::Number)),
    (IDENT, Some(TokenType::Ident)),
    (HASH, Some(TokenType::Hash)),
    (AT, Some(TokenType::At)),
    (DOT, Some(TokenType::Dot)),
    (SEMICOLON, Some(TokenType::Semicolon)),
    (LCURLY, Some(TokenType::LCurly)),
    (RCURLY, Some(TokenType::RCurly)),
    (LBRACKET, Some(TokenType::LBracket)),
    (RBRACKET, Some(TokenType::RBracket)),
    (COLON, Some(TokenType::Colon)),
    (IMPORTANT, Some(TokenType::Important)),
    (STRING, Some(TokenType::String)),
    (INCLUDE_MATCH, Some(TokenType::IncludeMatch)),
    (DASH_MATCH, Some(TokenType::DashMatch)),
    (PREFIX_MATCH, Some(TokenType::PrefixMatch)),
    (SUFFIX_MATCH, Some(TokenType::SuffixMatch)),
    (SUBSTRING_MATCH, Some(TokenType::SubstringMatch)),
    (EQUAL, Some(TokenType::Equal)),
];
