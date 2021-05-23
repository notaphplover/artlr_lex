use crate::token::traceable_token::TraceableToken;

pub trait LexicalAnalyzer<'a, TType> {
    fn load_source(&mut self, source: &'a str) -> ();
    fn next_token(&mut self) -> Option<TraceableToken<TType>>;
}
