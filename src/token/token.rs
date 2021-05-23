pub struct Token<'a, TType> {
    pub t_type: TType,
    pub text: &'a str,
}

impl<'a, TType> Token<'a, TType> {
    pub fn new(t_type: TType, text: &'a str) -> Self {
        Token { t_type, text }
    }
}

#[cfg(test)]
mod test {
    use crate::token::token::Token;

    #[test]
    fn new_returns_instance() {
        const TOKEN_TYPE: &str = "sample_type";
        const TOKEN_TEXT: &str = "sample_text";

        let token: Token<&str> = Token::new(TOKEN_TYPE, TOKEN_TEXT);

        assert_eq!(token.t_type, TOKEN_TYPE);
        assert_eq!(token.text, TOKEN_TEXT);
    }
}
