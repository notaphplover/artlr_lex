pub struct TextLocation {
    pub column: u64,
    pub line: u64,
}

impl TextLocation {
    pub fn new(column: u64, line: u64) -> Self {
        TextLocation { column, line }
    }
}

impl Copy for TextLocation { }

impl Clone for TextLocation {
    fn clone(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod test {
    use crate::text_location::TextLocation;

    #[test]
    fn new_returns_instance() {
        const COLUMN: u64 = 2;
        const LINE: u64 = 5;

        let text_location: TextLocation = TextLocation::new(COLUMN, LINE);

        assert_eq!(text_location.column, COLUMN);
        assert_eq!(text_location.line, LINE);
    }
}
