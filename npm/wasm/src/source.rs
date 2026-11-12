//! Translate source coordinates between UTF-8 bytes and UTF-16 code units.

use std::ops::Range;

use comfit_core::Error;

use crate::binding::{Point, Range as WebRange, SyntaxNode};

/// UTF-8/UTF-16 offsets and line starts for one unchanged source string.
pub struct Coordinates {
    offsets: Vec<(usize, usize)>,
    lines: Vec<usize>,
}

impl Coordinates {
    /// Build conversion tables from the source used by a parse.
    ///
    /// # Errors
    ///
    /// Returns a backend error if the UTF-8 source exceeds the runtime's 32-bit index limit.
    pub fn new(source: &str) -> Result<Self, Error> {
        if source.len() > u32::MAX as usize {
            return Err(Error::Backend(
                "source exceeds web-tree-sitter's index limit".into(),
            ));
        }

        let mut offsets = Vec::new();
        let mut lines = vec![0];
        let mut index = 0;

        for (byte, character) in source.char_indices() {
            index += character.len_utf16();

            if character == '\n' {
                lines.push(byte + 1);
            }

            if !character.is_ascii() {
                offsets.push((index, byte + character.len_utf8()));
            }
        }

        Ok(Self { offsets, lines })
    }

    fn byte(&self, index: usize, source: &str) -> Result<usize, Error> {
        let position = self.offsets.partition_point(|(unit, _)| *unit <= index);
        let delta = position
            .checked_sub(1)
            .map_or(0, |index| self.offsets[index].1 - self.offsets[index].0);
        let byte = index + delta;

        if !source.is_char_boundary(byte) {
            return Err(Error::InvalidRange(byte..byte));
        }

        Ok(byte)
    }

    fn index(&self, byte: usize) -> usize {
        let position = self.offsets.partition_point(|(_, offset)| *offset <= byte);
        let delta = position
            .checked_sub(1)
            .map_or(0, |index| self.offsets[index].1 - self.offsets[index].0);

        byte - delta
    }

    /// Convert a runtime node range to UTF-8 byte offsets in the same source.
    ///
    /// # Errors
    ///
    /// Returns an invalid-range error for coordinates that do not land on source boundaries.
    pub fn range(&self, node: &SyntaxNode, source: &str) -> Result<Range<usize>, Error> {
        Ok(self.byte(node.start_index() as usize, source)?
            ..self.byte(node.end_index() as usize, source)?)
    }

    /// Convert validated source byte endpoints to runtime UTF-16 indices and points.
    ///
    /// The range must refer to the source used to build these tables.
    pub fn web_range(&self, range: &Range<usize>) -> WebRange {
        let point = |byte| {
            let row = self.lines.partition_point(|start| *start <= byte) - 1;
            let start = self.lines[row];
            Point::new(row as u32, (self.index(byte) - self.index(start)) as u32)
        };

        WebRange::new(
            &point(range.start),
            &point(range.end),
            self.index(range.start) as u32,
            self.index(range.end) as u32,
        )
    }
}
