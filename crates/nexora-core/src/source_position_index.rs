use crate::{NexoraError, SourcePosition};

/// Immutable positions for one source; columns count Unicode scalar values, lines LF.
/// Continuation-byte ranks avoid rescanning even a long, single-line UTF-8 prefix.
pub struct SourcePositionIndex<'a> {
    source: &'a str,
    line_starts: Vec<usize>,
    continuations: Vec<usize>,
}
impl<'a> SourcePositionIndex<'a> {
    pub fn new(source: &'a str) -> Result<Self, NexoraError> {
        let mut line_starts = vec![0];
        let mut continuations = Vec::new();
        for (offset, byte) in source.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push(offset + 1);
            } else if byte & 0xc0 == 0x80 {
                continuations.push(offset);
            }
        }
        Ok(Self {
            source,
            line_starts,
            continuations,
        })
    }
    pub fn source(&self) -> &'a str {
        self.source
    }
    pub fn position(&self, offset: usize) -> Result<SourcePosition, NexoraError> {
        if !self.source.is_char_boundary(offset) {
            return Err(NexoraError::Configuration(
                "source position must be in bounds and on a UTF-8 boundary".into(),
            ));
        }
        let line = self.line_starts.partition_point(|start| *start <= offset);
        let start = self.line_starts[line - 1];
        let continuations = self.continuations.partition_point(|at| *at < offset)
            - self.continuations.partition_point(|at| *at < start);
        Ok(SourcePosition {
            offset,
            line,
            column: offset - start - continuations + 1,
        })
    }
}
