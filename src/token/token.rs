pub struct Token<TType> {
    pub t_type: TType,
    pub text: String,
}

impl<TType> Token<TType> {
    pub fn new(t_type: TType, text: String) -> Self {
        Token { t_type, text }
    }
}

impl<TType: Clone> Clone for Token<TType> {
    fn clone(&self) -> Self {
        Self::new(self.t_type.clone(), String::from(&self.text))
    }
}

#[cfg(test)]
mod test {
    use crate::token::token::Token;

    #[test]
    fn new_returns_instance() {
        const TOKEN_TYPE: &str = "sample_type";
        const TOKEN_TEXT: &str = "sample_text";

        let token: Token<&str> = Token::new(TOKEN_TYPE, String::from(TOKEN_TEXT));

        assert_eq!(token.t_type, TOKEN_TYPE);
        assert_eq!(token.text, TOKEN_TEXT);
    }
}
