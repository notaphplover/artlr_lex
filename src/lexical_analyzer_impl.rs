use crate::lex_spec::LexSpec;
use crate::text_location::TextLocation;
use crate::lexical_analyzer::LexicalAnalyzer as LexicalAnalyzerTrait;
use crate::traceable_token::TraceableToken;
use crate::token::Token;
use regex::Match;

const FIRST_COL: u64 = 1u64;
const FIRST_LINE: u64 = 1u64;

pub struct LexicalAnalyzer<'a, 'b, TType> {
    current_source_reference_option: Option<&'a str>,
    current_location: TextLocation,
    lex_spec: &'b dyn LexSpec<TType>,
}

impl<'a, 'b, TType> LexicalAnalyzer<'a, 'b, TType> {
    pub fn new(lex_spec: &'b dyn LexSpec<TType>) -> Self {
        LexicalAnalyzer {
            current_location: <LexicalAnalyzer<TType>>::get_starting_location(),
            current_source_reference_option: None,
            lex_spec
        }
    }

    fn get_starting_location() -> TextLocation {
        TextLocation::new(FIRST_COL, FIRST_LINE)
    }
}

impl<'a, 'b, TType> LexicalAnalyzer<'a, 'b, TType> {
    fn get_match_length(regex_match: &Match) -> usize {
        regex_match.end() - regex_match.start()
    }
}

impl<'a, 'b, TType: Copy> LexicalAnalyzer<'a, 'b, TType> {
    fn try_parse_next_token_from_source(lex_spec: &'b dyn LexSpec<TType>, source: &'a str) -> Option<Token<'a, TType>> {
        if source.len() == 0 {
            return Some(Token::new(lex_spec.get_eof_token(), ""));
        }

        let mut current_token_wrap: Option<Token<TType>> = Option::None;

        for token_parse_tuple in lex_spec.get_token_regex_collection() {
            let regex_match_wrap: Option<Match> = token_parse_tuple.regex.find(source);

            if regex_match_wrap.is_some() {
                let regex_match: Match = regex_match_wrap.unwrap();
                if current_token_wrap.is_none()
                    || current_token_wrap.as_ref().unwrap().text.len()
                    < <LexicalAnalyzer<TType>>::get_match_length(&regex_match)
                {
                    let source_slice_match: &'a str =
                        &source[regex_match.start()..regex_match.end()];
                    let token: Token<TType> = Token::new(token_parse_tuple.t_type, source_slice_match);
                    current_token_wrap = Option::from(token);
                }
            }
        }
        current_token_wrap
    }
}

impl<'a, 'b, TType: Copy + PartialEq> LexicalAnalyzer<'a, 'b, TType> {

    fn inner_next_token(&mut self) -> Option<TraceableToken<TType>> {
        let source_reference_option: Option<&mut &str> = self.current_source_reference_option.as_mut();

        match source_reference_option {
            Some(current_source_reference) => {
                let next_token_parsed_option=
                    LexicalAnalyzer::try_parse_filtered_token(
                        current_source_reference,
                        &mut self.current_location,
                        self.lex_spec
                    );

                next_token_parsed_option.map(|token| -> TraceableToken<TType> {
                    let text_location: TextLocation = self.current_location.clone();
                    let traceable_token: TraceableToken<TType> = TraceableToken::new(text_location, token);

                    LexicalAnalyzer::update_source_location(
                        &mut self.current_location,
                        self.lex_spec,
                        &traceable_token.token
                    );

                    traceable_token
                })
            }
            None => None
        }
    }

    fn process_non_skipped_token(
        current_source_reference: &mut &'a str,
        next_token_parsed_option: Option<&Token<'a, TType>>,
    ) -> () {
        match next_token_parsed_option {
            Some(next_token_parsed) => {
                LexicalAnalyzer::update_source_reference(
                    current_source_reference,
                    next_token_parsed
                );
            },
            _ => {},
        };
    }

    fn process_skipped_token(
        current_source_reference: &mut &'a str,
        source_location: &mut TextLocation,
        lex_spec: &'b dyn LexSpec<TType>,
        next_token_parsed_option: Option<Token<'a, TType>>,
    ) -> () {
        match next_token_parsed_option {
            Some(next_token_parsed) => {
                LexicalAnalyzer::update_source_reference(
                    current_source_reference,
                    &next_token_parsed
                );

                <LexicalAnalyzer<TType>>::update_source_location(source_location, lex_spec, &next_token_parsed);
            },
            _ => {},
        };
    }

    fn should_skip_token(lex_spec: &'b dyn LexSpec<TType>, token_option: Option<&Token<TType>>) -> bool {
        token_option.map(|token| -> bool {
            lex_spec.is_token_to_skip(token.t_type)
        }).unwrap_or(false)
    }

    fn try_parse_filtered_token(
        current_source_reference: &mut &'a str,
        source_location: &mut TextLocation,
        lex_spec: &'b dyn LexSpec<TType>,
    ) -> Option<Token<'a, TType>> {
        let mut next_token_parsed_option: Option<Token<'a, TType>> =
            LexicalAnalyzer::try_parse_next_token_from_source(lex_spec, current_source_reference);

        while LexicalAnalyzer::should_skip_token(lex_spec, next_token_parsed_option.as_ref()) {
            LexicalAnalyzer::process_skipped_token(
                current_source_reference,
                source_location,
                lex_spec,
                next_token_parsed_option
            );

            next_token_parsed_option = LexicalAnalyzer::try_parse_next_token_from_source(
                lex_spec,
                current_source_reference
            );
        }

        LexicalAnalyzer::process_non_skipped_token(
            current_source_reference,
            next_token_parsed_option.as_ref()
        );

        next_token_parsed_option
    }

    fn update_source_location(source_location: &mut TextLocation, lex_spec: &'b dyn LexSpec<TType>, token: &Token<TType>) -> () {
        if token.t_type == lex_spec.get_new_line_token_type() {
            source_location.column = FIRST_COL;
            source_location.line += 1u64;
        } else {
            source_location.column += token.text.len() as u64;
        }
    }

    fn update_source_reference<'c>(
        current_source_reference: &mut &'a str,
        next_token_parsed: &Token<TType>,
    ) -> () {
        let index: usize = next_token_parsed.text.len();
        *current_source_reference = &current_source_reference[index..];
    }
}

impl<'a, 'b, TType: Copy + PartialEq> LexicalAnalyzerTrait<'a, TType> for LexicalAnalyzer<'a, 'b, TType> {
    fn load_source(&mut self, source: &'a str) -> () {
        self.current_source_reference_option = Some(source);
        self.current_location = <LexicalAnalyzer<TType>>::get_starting_location();
    }

    fn next_token(&mut self) -> Option<TraceableToken<TType>> {
        self.inner_next_token()
    }
}

#[cfg(test)]
mod test {
    use crate::lex_spec::LexSpec;
    use crate::token_regex::TokenRegex;
    use regex::Regex;
    use crate::lexical_analyzer_impl::LexicalAnalyzer;
    use crate::lexical_analyzer::LexicalAnalyzer as LexicalAnalyzerTrait;
    use crate::traceable_token::TraceableToken;

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
        let mut lexical_analyzer: LexicalAnalyzer<TokenTypeTest> = LexicalAnalyzer::new(&lex_spec);

        lexical_analyzer.load_source("a");

        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analyzer.next_token();

        let traceable_token: TraceableToken<TokenTypeTest> = traceable_token_option.unwrap();

        assert_eq!(traceable_token.token.t_type, TokenTypeTest::Id);
        assert_eq!(traceable_token.token.text, "a");
        assert_eq!(traceable_token.location.column, 1);
        assert_eq!(traceable_token.location.line, 1);
    }

    #[test]
    fn next_token_returns_valid_token_after_skip_ignored_symbol() {
        let lex_spec: LexSpecTest = LexSpecTest::new();
        let mut lexical_analyzer: LexicalAnalyzer<TokenTypeTest> = LexicalAnalyzer::new(&lex_spec);

        lexical_analyzer.load_source("  a");

        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analyzer.next_token();

        let traceable_token: TraceableToken<TokenTypeTest> = traceable_token_option.unwrap();

        assert_eq!(traceable_token.token.t_type, TokenTypeTest::Id);
        assert_eq!(traceable_token.token.text, "a");
        assert_eq!(traceable_token.location.column, 3);
        assert_eq!(traceable_token.location.line, 1);
    }

    #[test]
    fn next_token_returns_valid_token_after_line_delimiter_symbol() {
        let lex_spec: LexSpecTest = LexSpecTest::new();
        let mut lexical_analyzer: LexicalAnalyzer<TokenTypeTest> = LexicalAnalyzer::new(&lex_spec);

        lexical_analyzer.load_source("\na");

        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analyzer.next_token();

        let traceable_token: TraceableToken<TokenTypeTest> = traceable_token_option.unwrap();

        assert_eq!(traceable_token.token.t_type, TokenTypeTest::Id);
        assert_eq!(traceable_token.token.text, "a");
        assert_eq!(traceable_token.location.column, 1);
        assert_eq!(traceable_token.location.line, 2);
    }

    #[test]
    fn next_token_returns_valid_token_after_next_token_call() {
        let lex_spec: LexSpecTest = LexSpecTest::new();
        let mut lexical_analyzer: LexicalAnalyzer<TokenTypeTest> = LexicalAnalyzer::new(&lex_spec);

        lexical_analyzer.load_source("a b");

        lexical_analyzer.next_token();
        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analyzer.next_token();

        let traceable_token: TraceableToken<TokenTypeTest> = traceable_token_option.unwrap();

        assert_eq!(traceable_token.token.t_type, TokenTypeTest::Id);
        assert_eq!(traceable_token.token.text, "b");
        assert_eq!(traceable_token.location.column, 3);
        assert_eq!(traceable_token.location.line, 1);
    }

    #[test]
    fn next_token_returns_eof_token_on_empty_source() {
        let lex_spec: LexSpecTest = LexSpecTest::new();
        let mut lexical_analyzer: LexicalAnalyzer<TokenTypeTest> = LexicalAnalyzer::new(&lex_spec);

        lexical_analyzer.load_source("");

        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analyzer.next_token();

        let traceable_token: TraceableToken<TokenTypeTest> = traceable_token_option.unwrap();

        assert_eq!(traceable_token.token.t_type, TokenTypeTest::Eof);
        assert_eq!(traceable_token.token.text, "");
        assert_eq!(traceable_token.location.column, 1);
        assert_eq!(traceable_token.location.line, 1);
    }

    #[test]
    fn next_token_returns_no_token() {
        let lex_spec: LexSpecTest = LexSpecTest::new();
        let mut lexical_analyzer: LexicalAnalyzer<TokenTypeTest> = LexicalAnalyzer::new(&lex_spec);

        lexical_analyzer.load_source("()");

        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analyzer.next_token();

        assert!(traceable_token_option.is_none());
    }

    #[test]
    fn next_token_returns_no_token_after_skipped_token() {
        let lex_spec: LexSpecTest = LexSpecTest::new();
        let mut lexical_analyzer: LexicalAnalyzer<TokenTypeTest> = LexicalAnalyzer::new(&lex_spec);

        lexical_analyzer.load_source("  ()");

        let traceable_token_option: Option<TraceableToken<TokenTypeTest>> = lexical_analyzer.next_token();

        assert!(traceable_token_option.is_none());
    }
}
