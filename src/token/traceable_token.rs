use crate::token::token::Token;
use crate::text_location::TextLocation;

pub struct TraceableToken<TType> {
    pub location: TextLocation,
    pub token: Token<TType>,
}

impl<TType> TraceableToken<TType> {
    pub fn new(location: TextLocation, token: Token<TType>) -> Self {
        TraceableToken { location, token }
    }
}

impl<TType: Clone> Clone for TraceableToken<TType> {
    fn clone(&self) -> Self {
        Self::new(self.location.clone(), self.token.clone())
    }
}
