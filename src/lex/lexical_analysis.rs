use regex::Match;

use crate::lex::lex_spec::LexSpec;
use crate::text_location::TextLocation;
use crate::token::token::Token;
use crate::token::traceable_token::TraceableToken;

const FIRST_COL: u64 = 1u64;
const FIRST_LINE: u64 = 1u64;

pub struct LexicalAnalysis<'a, T> {
    current_location: TextLocation,
    lex_spec: &'a dyn LexSpec<T>,
    source: &'a str,
}

impl<T> LexicalAnalysis<'_, T> {

    fn get_match_length(regex_match: &Match) -> usize {
        regex_match.end() - regex_match.start()
    }

    fn get_starting_location() -> TextLocation {
        TextLocation::new(FIRST_COL, FIRST_LINE)
    }
}

impl<'a, T> LexicalAnalysis<'a, T> {
    pub fn new(source: &'a str, lex_spec: &'a dyn LexSpec<T>) -> Self {
        LexicalAnalysis {
            current_location: Self::get_starting_location(),
            lex_spec,
            source,
        }
    }
}

impl<'a, T: Copy> LexicalAnalysis<'a, T> {
    fn try_parse_next_token_from_source(&self) -> Option<Token<T>> {
        if self.source.len() == 0 {
            return Some(Token::new(self.lex_spec.get_eof_token(), String::from("")));
        }

        let mut current_token_wrap: Option<Token<T>> = Option::None;

        for token_parse_tuple in self.lex_spec.get_token_regex_collection() {
            let regex_match_wrap: Option<Match> = token_parse_tuple.regex.find(self.source);

            if regex_match_wrap.is_some() {
                let regex_match: Match = regex_match_wrap.unwrap();
                if current_token_wrap.is_none()
                    || current_token_wrap.as_ref().unwrap().text.len()
                    < Self::get_match_length(&regex_match)
                {
                    let source_slice_match: &'a str =
                        &self.source[regex_match.start()..regex_match.end()];
                    let token: Token<T> = Token::new(token_parse_tuple.t_type, String::from(source_slice_match));
                    current_token_wrap = Option::from(token);
                }
            }
        }
        current_token_wrap
    }
}

impl<'a, T: Copy + PartialEq> LexicalAnalysis<'a, T> {
    fn inner_next_token(&mut self) -> Option<TraceableToken<T>> {
        let next_token_parsed_option = self.try_parse_filtered_token();

        next_token_parsed_option.map(|token| -> TraceableToken<T> {
            let text_location: TextLocation = self.current_location.clone();
            let traceable_token: TraceableToken<T> = TraceableToken::new(text_location, token);

            self.update_source_location(&traceable_token.token);

            traceable_token
        })
    }

    fn process_non_skipped_token(
        &mut self,
        next_token_parsed_option: Option<&Token<T>>,
    ) -> () {
        match next_token_parsed_option {
            Some(next_token_parsed) => {
                self.source = Self::next_source_reference(
                    self.source,
                    next_token_parsed
                );
            },
            _ => {},
        };
    }

    fn process_skipped_token(
        &mut self,
        next_token_parsed_option: Option<Token<T>>,
    ) -> () {
        match next_token_parsed_option {
            Some(next_token_parsed) => {
                self.source = Self::next_source_reference(
                    self.source,
                    &next_token_parsed
                );

                self.update_source_location(&next_token_parsed);
            },
            _ => {},
        };
    }

    fn should_skip_token(&self, token_option: Option<&Token<T>>) -> bool {
        token_option.map(|token| -> bool {
            self.lex_spec.is_token_to_skip(token.t_type)
        }).unwrap_or(false)
    }

    fn try_parse_filtered_token(&mut self) -> Option<Token<T>> {
        let mut next_token_parsed_option: Option<Token<T>> =
            self.try_parse_next_token_from_source();

        while self.should_skip_token(next_token_parsed_option.as_ref()) {
            self.process_skipped_token(next_token_parsed_option);

            next_token_parsed_option = self.try_parse_next_token_from_source();
        }

        self.process_non_skipped_token(next_token_parsed_option.as_ref());

        next_token_parsed_option
    }

    fn update_source_location(&mut self, token: &Token<T>) -> () {
        if token.t_type == self.lex_spec.get_new_line_token_type() {
            self.current_location.column = FIRST_COL;
            self.current_location.line += 1u64;
        } else {
            self.current_location.column += token.text.len() as u64;
        }
    }

    fn next_source_reference(
        current_source_reference: &'a str,
        next_token_parsed: &Token<T>,
    ) -> &'a str {
        let index: usize = next_token_parsed.text.len();
        &current_source_reference[index..]
    }
}

impl<'a, T: Copy + PartialEq> Iterator for LexicalAnalysis<'a, T> {
    type Item = TraceableToken<T>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.inner_next_token()
    }
}

#[cfg(test)]
mod test {
    use regex::Regex;

    use crate::lex::lex_spec::LexSpec;
    use crate::lex::lexical_analysis::LexicalAnalysis;
    use crate::token::traceable_token::TraceableToken;
    use crate::token::token_regex::TokenRegex;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum IgnoredTokenTypeTest {
        LineDelimiter,
        Space,
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum TokenTypeTest {
        Eof,
        Id,
        Ignored(IgnoredTokenTypeTest)
    }

    struct LexSpecTest {
        token_regex_collection: Vec<TokenRegex<TokenTypeTest>>,
    }

    impl LexSpecTest {
        fn new() -> LexSpecTest {
            let line_delimiter_token_regex: TokenRegex<TokenTypeTest> = TokenRegex::new(
                Regex::new(r"^\r?\n").unwrap(),
                TokenTypeTest::Ignored(IgnoredTokenTypeTest::LineDelimiter)
            );

            let space_token_regex: TokenRegex<TokenTypeTest> = TokenRegex::new(
                Regex::new(r"^[\t\f\v ]+").unwrap(),
                TokenTypeTest::Ignored(IgnoredTokenTypeTest::Space)
            );

            let id_token_regex: TokenRegex<TokenTypeTest> = TokenRegex::new(
                Regex::new(r"^\w([\w\d]+)?").unwrap(),
                TokenTypeTest::Id
            );

            let token_regex_collection: Vec<TokenRegex<TokenTypeTest>> = vec![
                line_delimiter_token_regex,
                space_token_regex,
                id_token_regex,
            ];

            LexSpecTest { token_regex_collection }
        }
    }

    impl LexSpec<TokenTypeTest> for LexSpecTest {
        fn get_eof_token(&self) -> TokenTypeTest {
            TokenTypeTest::Eof
        }

        fn get_new_line_token_type(&self) -> TokenTypeTest {
            TokenTypeTest::Ignored(IgnoredTokenTypeTest::LineDelimiter)
        }

        fn get_token_regex_collection(&self) -> &Vec<TokenRegex<TokenTypeTest>> {
            &self.token_regex_collection
        }

        fn is_token_to_skip(&self, t_type: TokenTypeTest) -> bool {
            match t_type {
                TokenTypeTest::Ignored(_) => {
                    true
                },
                _ => false
            }
        }
    }

    #[test]
    fn next_token_returns_valid_token() {
        let lex_spec: LexSpecTest = LexSpecTest::new();
        let mut lexical_analysis = LexicalAnalysis::new("a", &lex_spec);

        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analysis.next();

        let traceable_token: TraceableToken<TokenTypeTest> = traceable_token_option.unwrap();

        assert_eq!(traceable_token.token.t_type, TokenTypeTest::Id);
        assert_eq!(traceable_token.token.text, "a");
        assert_eq!(traceable_token.location.column, 1);
        assert_eq!(traceable_token.location.line, 1);
    }

    #[test]
    fn next_token_returns_valid_token_after_skip_ignored_symbol() {
        let lex_spec: LexSpecTest = LexSpecTest::new();
        let mut lexical_analysis = LexicalAnalysis::new("  a", &lex_spec);

        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analysis.next();

        let traceable_token: TraceableToken<TokenTypeTest> = traceable_token_option.unwrap();

        assert_eq!(traceable_token.token.t_type, TokenTypeTest::Id);
        assert_eq!(traceable_token.token.text, "a");
        assert_eq!(traceable_token.location.column, 3);
        assert_eq!(traceable_token.location.line, 1);
    }

    #[test]
    fn next_token_returns_valid_token_after_line_delimiter_symbol() {
        let lex_spec: LexSpecTest = LexSpecTest::new();
        let mut lexical_analysis = LexicalAnalysis::new("\na", &lex_spec);

        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analysis.next();

        let traceable_token: TraceableToken<TokenTypeTest> = traceable_token_option.unwrap();

        assert_eq!(traceable_token.token.t_type, TokenTypeTest::Id);
        assert_eq!(traceable_token.token.text, "a");
        assert_eq!(traceable_token.location.column, 1);
        assert_eq!(traceable_token.location.line, 2);
    }

    #[test]
    fn next_token_returns_valid_token_after_next_token_call() {
        let lex_spec: LexSpecTest = LexSpecTest::new();
        let mut lexical_analysis = LexicalAnalysis::new("a b", &lex_spec);

        lexical_analysis.next();

        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analysis.next();

        let traceable_token: TraceableToken<TokenTypeTest> = traceable_token_option.unwrap();

        assert_eq!(traceable_token.token.t_type, TokenTypeTest::Id);
        assert_eq!(traceable_token.token.text, "b");
        assert_eq!(traceable_token.location.column, 3);
        assert_eq!(traceable_token.location.line, 1);
    }

    #[test]
    fn next_token_returns_eof_token_on_empty_source() {
        let lex_spec: LexSpecTest = LexSpecTest::new();
        let mut lexical_analysis = LexicalAnalysis::new("", &lex_spec);

        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analysis.next();

        let traceable_token: TraceableToken<TokenTypeTest> = traceable_token_option.unwrap();

        assert_eq!(traceable_token.token.t_type, TokenTypeTest::Eof);
        assert_eq!(traceable_token.token.text, "");
        assert_eq!(traceable_token.location.column, 1);
        assert_eq!(traceable_token.location.line, 1);
    }

    #[test]
    fn next_token_returns_no_token() {
        let lex_spec: LexSpecTest = LexSpecTest::new();
        let mut lexical_analysis = LexicalAnalysis::new("()", &lex_spec);

        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analysis.next();

        assert!(traceable_token_option.is_none());
    }

    #[test]
    fn next_token_returns_no_token_after_skipped_token() {
        let lex_spec: LexSpecTest = LexSpecTest::new();
        let mut lexical_analysis = LexicalAnalysis::new("  ()", &lex_spec);

        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analysis.next();

        assert!(traceable_token_option.is_none());
    }
}
