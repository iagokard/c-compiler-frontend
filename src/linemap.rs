pub struct LineMap {
    line_starts: Vec<usize>,
    char_starts: Vec<Vec<usize>>,
}

impl LineMap {
    pub fn new(src: &str) -> Self {
        let mut line_starts = vec![0];
        let mut char_starts: Vec<Vec<usize>> = vec![Vec::new()];

        let mut byte = 0usize;
        for ch in src.chars() {
            if ch == '\n' {
                line_starts.push(byte + 1);
                char_starts.push(Vec::new());
            } else {
                char_starts.last_mut().unwrap().push(byte);
            }
            byte += ch.len_utf8();
        }

        Self {
            line_starts,
            char_starts,
        }
    }

    pub fn location(&self, offset: usize) -> (usize, usize) {
        let line = self.line_starts.partition_point(|&s| s <= offset);
        let chars = &self.char_starts[line - 1];
        let col = chars.partition_point(|&b| b < offset) + 1;
        (line, col)
    }

    pub fn span_location(&self, span: crate::token::Span) -> (usize, usize) {
        self.location(span.start)
    }
}
