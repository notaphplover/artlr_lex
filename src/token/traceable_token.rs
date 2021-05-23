use crate::token::token::Token;
use crate::text_location::TextLocation;

pub struct TraceableToken<'a, TType> {
    pub location: TextLocation,
    pub token: Token<'a, TType>,
}

impl<'a, TType> TraceableToken<'a, TType> {
    pub fn new(location: TextLocation, token: Token<'a, TType>) -> Self {
        TraceableToken { location, token }
    }
}
