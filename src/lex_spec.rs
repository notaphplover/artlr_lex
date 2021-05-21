use crate::token_regex::TokenRegex;

pub trait LexSpec<TType> {
    fn get_eof_token(&self) -> TType;
    fn get_new_line_token_type(&self) -> TType;
    fn get_token_regex_collection(&self) -> &Vec<TokenRegex<TType>>;
    fn is_token_to_skip(&self, t_type: TType) -> bool;
}
