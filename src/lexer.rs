use crate::traceable_token::TraceableToken;

pub trait Lexer<'a, TType> {
    fn load_source(&mut self, source: &'a str) -> ();
    fn next_token(&mut self) -> Option<TraceableToken<TType>>;
}
