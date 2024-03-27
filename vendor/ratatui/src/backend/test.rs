//! This module provides the `TestBackend` implementation for the [`Backend`] trait.
//! It is used in the integration tests to verify the correctness of the library.

use std::{
    fmt::{Display, Write},
    io,
};

use unicode_width::UnicodeWidthStr;

use crate::{
    backend::{Backend, ClearType, WindowSize},
    buffer::{Buffer, Cell},
    layout::{Rect, Size},
};

/// A [`Backend`] implementation used for integration testing that that renders to an in memory
/// buffer.
///
/// Note: that although many of the integration and unit tests in ratatui are written using this
/// backend, it is preferable to write unit tests for widgets directly against the buffer rather
/// than using this backend. This backend is intended for integration tests that test the entire
/// terminal UI.
///
/// # Example
///
/// ```rust
/// use ratatui::{backend::TestBackend, prelude::*};
///
/// let mut backend = TestBackend::new(10, 2);
/// backend.clear()?;
/// backend.assert_buffer(&Buffer::with_lines(vec!["          "; 2]));
/// # std::io::Result::Ok(())
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TestBackend {
    width: u16,
    buffer: Buffer,
    height: u16,
    cursor: bool,
    pos: (u16, u16),
}

/// Returns a string representation of the given buffer for debugging purpose.
///
/// This function is used to visualize the buffer content in a human-readable format.
/// It iterates through the buffer content and appends each cell's symbol to the view string.
/// If a cell is hidden by a multi-width symbol, it is added to the overwritten vector and
/// displayed at the end of the line.
fn buffer_view(buffer: &Buffer) -> String {
    let mut view = String::with_capacity(buffer.content.len() + buffer.area.height as usize * 3);
    for cells in buffer.content.chunks(buffer.area.width as usize) {
        let mut overwritten = vec![];
        let mut skip: usize = 0;
        view.push('"');
        for (x, c) in cells.iter().enumerate() {
            if skip == 0 {
                view.push_str(c.symbol());
            } else {
                overwritten.push((x, c.symbol()));
            }
            skip = std::cmp::max(skip, c.symbol().width()).saturating_sub(1);
        }
        view.push('"');
        if !overwritten.is_empty() {
            write!(&mut view, " Hidden by multi-width symbols: {overwritten:?}").unwrap();
        }
        view.push('\n');
    }
    view
}

impl TestBackend {
    /// Creates a new TestBackend with the specified width and height.
    pub fn new(width: u16, height: u16) -> TestBackend {
        TestBackend {
            width,
            height,
            buffer: Buffer::empty(Rect::new(0, 0, width, height)),
            cursor: false,
            pos: (0, 0),
        }
    }

    /// Returns a reference to the internal buffer of the TestBackend.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Resizes the TestBackend to the specified width and height.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.buffer.resize(Rect::new(0, 0, width, height));
        self.width = width;
        self.height = height;
    }

    /// Asserts that the TestBackend's buffer is equal to the expected buffer.
    /// If the buffers are not equal, a panic occurs with a detailed error message
    /// showing the differences between the expected and actual buffers.
    #[track_caller]
    pub fn assert_buffer(&self, expected: &Buffer) {
        assert_eq!(expected.area, self.buffer.area);
        let diff = expected.diff(&self.buffer);
        if diff.is_empty() {
            return;
        }

        let mut debug_info = String::from("Buffers are not equal");
        debug_info.push('\n');
        debug_info.push_str("Expected:");
        debug_info.push('\n');
        let expected_view = buffer_view(expected);
        debug_info.push_str(&expected_view);
        debug_info.push('\n');
        debug_info.push_str("Got:");
        debug_info.push('\n');
        let view = buffer_view(&self.buffer);
        debug_info.push_str(&view);
        debug_info.push('\n');

        debug_info.push_str("Diff:");
        debug_info.push('\n');
        let nice_diff = diff
            .iter()
            .enumerate()
            .map(|(i, (x, y, cell))| {
                let expected_cell = expected.get(*x, *y);
                format!("{i}: at ({x}, {y}) expected {expected_cell:?} got {cell:?}")
            })
            .collect::<Vec<String>>()
            .join("\n");
        debug_info.push_str(&nice_diff);
        panic!("{debug_info}");
    }
}

impl Display for TestBackend {
    /// Formats the TestBackend for display by calling the buffer_view function
    /// on its internal buffer.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", buffer_view(&self.buffer))
    }
}

impl Backend for TestBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, c) in content {
            let cell = self.buffer.get_mut(x, y);
            *cell = c.clone();
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), io::Error> {
        self.cursor = false;
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), io::Error> {
        self.cursor = true;
        Ok(())
    }

    fn get_cursor(&mut self) -> Result<(u16, u16), io::Error> {
        Ok(self.pos)
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), io::Error> {
        self.pos = (x, y);
        Ok(())
    }

    fn clear(&mut self) -> Result<(), io::Error> {
        self.buffer.reset();
        Ok(())
    }

    fn clear_region(&mut self, clear_type: super::ClearType) -> io::Result<()> {
        match clear_type {
            ClearType::All => self.clear()?,
            ClearType::AfterCursor => {
                let index = self.buffer.index_of(self.pos.0, self.pos.1) + 1;
                self.buffer.content[index..].fill(Cell::default());
            }
            ClearType::BeforeCursor => {
                let index = self.buffer.index_of(self.pos.0, self.pos.1);
                self.buffer.content[..index].fill(Cell::default());
            }
            ClearType::CurrentLine => {
                let line_start_index = self.buffer.index_of(0, self.pos.1);
                let line_end_index = self.buffer.index_of(self.width - 1, self.pos.1);
                self.buffer.content[line_start_index..=line_end_index].fill(Cell::default());
            }
            ClearType::UntilNewLine => {
                let index = self.buffer.index_of(self.pos.0, self.pos.1);
                let line_end_index = self.buffer.index_of(self.width - 1, self.pos.1);
                self.buffer.content[index..=line_end_index].fill(Cell::default());
            }
        }
        Ok(())
    }

    /// Inserts n line breaks at the current cursor position.
    ///
    /// After the insertion, the cursor x position will be incremented by 1 (unless it's already
    /// at the end of line). This is a common behaviour of terminals in raw mode.
    ///
    /// If the number of lines to append is fewer than the number of lines in the buffer after the
    /// cursor y position then the cursor is moved down by n rows.
    ///
    /// If the number of lines to append is greater than the number of lines in the buffer after
    /// the cursor y position then that number of empty lines (at most the buffer's height in this
    /// case but this limit is instead replaced with scrolling in most backend implementations) will
    /// be added after the current position and the cursor will be moved to the last row.
    fn append_lines(&mut self, n: u16) -> io::Result<()> {
        let (cur_x, cur_y) = self.get_cursor()?;

        // the next column ensuring that we don't go past the last column
        let new_cursor_x = cur_x.saturating_add(1).min(self.width.saturating_sub(1));

        let max_y = self.height.saturating_sub(1);
        let lines_after_cursor = max_y.saturating_sub(cur_y);
        if n > lines_after_cursor {
            let rotate_by = n.saturating_sub(lines_after_cursor).min(max_y);

            if rotate_by == self.height - 1 {
                self.clear()?;
            }

            self.set_cursor(0, rotate_by)?;
            self.clear_region(ClearType::BeforeCursor)?;
            self.buffer
                .content
                .rotate_left((self.width * rotate_by).into());
        }

        let new_cursor_y = cur_y.saturating_add(n).min(max_y);
        self.set_cursor(new_cursor_x, new_cursor_y)?;

        Ok(())
    }

    fn size(&self) -> Result<Rect, io::Error> {
        Ok(Rect::new(0, 0, self.width, self.height))
    }

    fn window_size(&mut self) -> Result<WindowSize, io::Error> {
        // Some arbitrary window pixel size, probably doesn't need much testing.
        static WINDOW_PIXEL_SIZE: Size = Size {
            width: 640,
            height: 480,
        };
        Ok(WindowSize {
            columns_rows: (self.width, self.height).into(),
            pixels: WINDOW_PIXEL_SIZE,
        })
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            TestBackend::new(10, 2),
            TestBackend {
                width: 10,
                height: 2,
                buffer: Buffer::with_lines(vec!["          "; 2]),
                cursor: false,
                pos: (0, 0),
            }
        );
    }
    #[test]
    fn test_buffer_view() {
        let buffer = Buffer::with_lines(vec!["aaaa"; 2]);
        assert_eq!(buffer_view(&buffer), "\"aaaa\"\n\"aaaa\"\n");
    }

    #[test]
    fn buffer_view_with_overwrites() {
        let multi_byte_char = "👨‍👩‍👧‍👦"; // renders 8 wide
        let buffer = Buffer::with_lines(vec![multi_byte_char]);
        assert_eq!(
            buffer_view(&buffer),
            format!(
                r#""{multi_byte_char}" Hidden by multi-width symbols: [(1, " "), (2, " "), (3, " "), (4, " "), (5, " "), (6, " "), (7, " ")]
"#,
                multi_byte_char = multi_byte_char
            )
        );
    }

    #[test]
    fn buffer() {
        let backend = TestBackend::new(10, 2);
        assert_eq!(backend.buffer(), &Buffer::with_lines(vec!["          "; 2]));
    }

    #[test]
    fn resize() {
        let mut backend = TestBackend::new(10, 2);
        backend.resize(5, 5);
        assert_eq!(backend.buffer(), &Buffer::with_lines(vec!["     "; 5]));
    }

    #[test]
    fn assert_buffer() {
        let backend = TestBackend::new(10, 2);
        let buffer = Buffer::with_lines(vec!["          "; 2]);
        backend.assert_buffer(&buffer);
    }

    #[test]
    #[should_panic]
    fn assert_buffer_panics() {
        let backend = TestBackend::new(10, 2);
        let buffer = Buffer::with_lines(vec!["aaaaaaaaaa"; 2]);
        backend.assert_buffer(&buffer);
    }

    #[test]
    fn display() {
        let backend = TestBackend::new(10, 2);
        assert_eq!(format!("{}", backend), "\"          \"\n\"          \"\n");
    }

    #[test]
    fn draw() {
        let mut backend = TestBackend::new(10, 2);
        let mut cell = Cell::default();
        cell.set_symbol("a");
        backend.draw([(0, 0, &cell)].into_iter()).unwrap();
        backend.draw([(0, 1, &cell)].into_iter()).unwrap();
        backend.assert_buffer(&Buffer::with_lines(vec!["a         "; 2]));
    }

    #[test]
    fn hide_cursor() {
        let mut backend = TestBackend::new(10, 2);
        backend.hide_cursor().unwrap();
        assert!(!backend.cursor);
    }

    #[test]
    fn show_cursor() {
        let mut backend = TestBackend::new(10, 2);
        backend.show_cursor().unwrap();
        assert!(backend.cursor);
    }

    #[test]
    fn get_cursor() {
        let mut backend = TestBackend::new(10, 2);
        assert_eq!(backend.get_cursor().unwrap(), (0, 0));
    }

    #[test]
    fn set_cursor() {
        let mut backend = TestBackend::new(10, 10);
        backend.set_cursor(5, 5).unwrap();
        assert_eq!(backend.pos, (5, 5));
    }

    #[test]
    fn clear() {
        let mut backend = TestBackend::new(10, 4);
        let mut cell = Cell::default();
        cell.set_symbol("a");
        backend.draw([(0, 0, &cell)].into_iter()).unwrap();
        backend.draw([(0, 1, &cell)].into_iter()).unwrap();
        backend.clear().unwrap();
        backend.assert_buffer(&Buffer::with_lines(vec![
            "          ",
            "          ",
            "          ",
            "          ",
        ]));
    }

    #[test]
    fn clear_region_all() {
        let mut backend = TestBackend::new(10, 5);
        backend.buffer = Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]);

        backend.clear_region(ClearType::All).unwrap();
        backend.assert_buffer(&Buffer::with_lines(vec![
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
        ]));
    }

    #[test]
    fn clear_region_after_cursor() {
        let mut backend = TestBackend::new(10, 5);
        backend.buffer = Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]);

        backend.set_cursor(3, 2).unwrap();
        backend.clear_region(ClearType::AfterCursor).unwrap();
        backend.assert_buffer(&Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaa      ",
            "          ",
            "          ",
        ]));
    }

    #[test]
    fn clear_region_before_cursor() {
        let mut backend = TestBackend::new(10, 5);
        backend.buffer = Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]);

        backend.set_cursor(5, 3).unwrap();
        backend.clear_region(ClearType::BeforeCursor).unwrap();
        backend.assert_buffer(&Buffer::with_lines(vec![
            "          ",
            "          ",
            "          ",
            "     aaaaa",
            "aaaaaaaaaa",
        ]));
    }

    #[test]
    fn clear_region_current_line() {
        let mut backend = TestBackend::new(10, 5);
        backend.buffer = Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]);

        backend.set_cursor(3, 1).unwrap();
        backend.clear_region(ClearType::CurrentLine).unwrap();
        backend.assert_buffer(&Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "          ",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]));
    }

    #[test]
    fn clear_region_until_new_line() {
        let mut backend = TestBackend::new(10, 5);
        backend.buffer = Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]);

        backend.set_cursor(3, 0).unwrap();
        backend.clear_region(ClearType::UntilNewLine).unwrap();
        backend.assert_buffer(&Buffer::with_lines(vec![
            "aaa       ",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
            "aaaaaaaaaa",
        ]));
    }

    #[test]
    fn append_lines_not_at_last_line() {
        let mut backend = TestBackend::new(10, 5);
        backend.buffer = Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);

        backend.set_cursor(0, 0).unwrap();

        // If the cursor is not at the last line in the terminal the addition of a
        // newline simply moves the cursor down and to the right

        backend.append_lines(1).unwrap();
        assert_eq!(backend.get_cursor().unwrap(), (1, 1));

        backend.append_lines(1).unwrap();
        assert_eq!(backend.get_cursor().unwrap(), (2, 2));

        backend.append_lines(1).unwrap();
        assert_eq!(backend.get_cursor().unwrap(), (3, 3));

        backend.append_lines(1).unwrap();
        assert_eq!(backend.get_cursor().unwrap(), (4, 4));

        // As such the buffer should remain unchanged
        backend.assert_buffer(&Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]));
    }

    #[test]
    fn append_lines_at_last_line() {
        let mut backend = TestBackend::new(10, 5);
        backend.buffer = Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);

        // If the cursor is at the last line in the terminal the addition of a
        // newline will scroll the contents of the buffer
        backend.set_cursor(0, 4).unwrap();

        backend.append_lines(1).unwrap();

        backend.buffer = Buffer::with_lines(vec![
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
            "          ",
        ]);

        // It also moves the cursor to the right, as is common of the behaviour of
        // terminals in raw-mode
        assert_eq!(backend.get_cursor().unwrap(), (1, 4));
    }

    #[test]
    fn append_multiple_lines_not_at_last_line() {
        let mut backend = TestBackend::new(10, 5);
        backend.buffer = Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);

        backend.set_cursor(0, 0).unwrap();

        // If the cursor is not at the last line in the terminal the addition of multiple
        // newlines simply moves the cursor n lines down and to the right by 1

        backend.append_lines(4).unwrap();
        assert_eq!(backend.get_cursor().unwrap(), (1, 4));

        // As such the buffer should remain unchanged
        backend.assert_buffer(&Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]));
    }

    #[test]
    fn append_multiple_lines_past_last_line() {
        let mut backend = TestBackend::new(10, 5);
        backend.buffer = Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);

        backend.set_cursor(0, 3).unwrap();

        backend.append_lines(3).unwrap();
        assert_eq!(backend.get_cursor().unwrap(), (1, 4));

        backend.assert_buffer(&Buffer::with_lines(vec![
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
            "          ",
            "          ",
        ]));
    }

    #[test]
    fn append_multiple_lines_where_cursor_at_end_appends_height_lines() {
        let mut backend = TestBackend::new(10, 5);
        backend.buffer = Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);

        backend.set_cursor(0, 4).unwrap();

        backend.append_lines(5).unwrap();
        assert_eq!(backend.get_cursor().unwrap(), (1, 4));

        backend.assert_buffer(&Buffer::with_lines(vec![
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
        ]));
    }

    #[test]
    fn append_multiple_lines_where_cursor_appends_height_lines() {
        let mut backend = TestBackend::new(10, 5);
        backend.buffer = Buffer::with_lines(vec![
            "aaaaaaaaaa",
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
        ]);

        backend.set_cursor(0, 0).unwrap();

        backend.append_lines(5).unwrap();
        assert_eq!(backend.get_cursor().unwrap(), (1, 4));

        backend.assert_buffer(&Buffer::with_lines(vec![
            "bbbbbbbbbb",
            "cccccccccc",
            "dddddddddd",
            "eeeeeeeeee",
            "          ",
        ]));
    }

    #[test]
    fn size() {
        let backend = TestBackend::new(10, 2);
        assert_eq!(backend.size().unwrap(), Rect::new(0, 0, 10, 2));
    }

    #[test]
    fn flush() {
        let mut backend = TestBackend::new(10, 2);
        backend.flush().unwrap();
    }
}
