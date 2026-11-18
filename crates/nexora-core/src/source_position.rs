use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourcePosition {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}
impl Default for SourcePosition {
    fn default() -> Self {
        Self {
            offset: 0,
            line: 1,
            column: 1,
        }
    }
}
impl SourcePosition {
    pub fn advance(&mut self, text: &str) {
        self.offset += text.len();
        for ch in text.chars() {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }
}
