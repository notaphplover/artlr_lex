use regex::Regex;

pub struct TokenRegex<TType> {
    pub regex: Regex,
    pub t_type: TType,
}

impl<TType> TokenRegex<TType> {
    pub fn new(regex: Regex, t_type: TType) -> TokenRegex<TType> {
        TokenRegex { regex, t_type }
    }
}
