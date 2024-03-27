#![deny(missing_docs)]
use std::borrow::Cow;

use super::StyledGrapheme;
use crate::{prelude::*, widgets::Widget};

/// A line of text, consisting of one or more [`Span`]s.
///
/// [`Line`]s are used wherever text is displayed in the terminal and represent a single line of
/// text. When a [`Line`] is rendered, it is rendered as a single line of text, with each [`Span`]
/// being rendered in order (left to right).
///
/// [`Line`]s can be created from [`Span`]s, [`String`]s, and [`&str`]s. They can be styled with a
/// [`Style`], and have an [`Alignment`].
///
/// The line's [`Alignment`] is used by the rendering widget to determine how to align the line
/// within the available space. If the line is longer than the available space, the alignment is
/// ignored and the line is truncated.
///
/// The line's [`Style`] is used by the rendering widget to determine how to style the line. If the
/// line is longer than the available space, the style is applied to the entire line, and the line
/// is truncated. Each [`Span`] in the line will be styled with the [`Style`] of the line, and then
/// with its own [`Style`].
///
/// `Line` implements the [`Widget`] trait, which means it can be rendered to a [`Buffer`]. Usually
/// apps will use the [`Paragraph`] widget instead of rendering a [`Line`] directly as it provides
/// more functionality.
///
/// # Constructor Methods
///
/// - [`Line::default`] creates a line with empty content and the default style.
/// - [`Line::raw`] creates a line with the given content and the default style.
/// - [`Line::styled`] creates a line with the given content and style.
///
/// # Setter Methods
///
/// These methods are fluent setters. They return a `Line` with the property set.
///
/// - [`Line::spans`] sets the content of the line.
/// - [`Line::style`] sets the style of the line.
/// - [`Line::alignment`] sets the alignment of the line.
///
/// # Other Methods
///
/// - [`Line::patch_style`] patches the style of the line, adding modifiers from the given style.
/// - [`Line::reset_style`] resets the style of the line.
/// - [`Line::width`] returns the unicode width of the content held by this line.
/// - [`Line::styled_graphemes`] returns an iterator over the graphemes held by this line.
///
/// # Compatibility Notes
///
/// Before v0.26.0, [`Line`] did not have a `style` field and instead relied on only the styles that
/// were set on each [`Span`] contained in the `spans` field. The [`Line::patch_style`] method was
/// the only way to set the overall style for individual lines. For this reason, this field may not
/// be supported yet by all widgets (outside of the `ratatui` crate itself).
///
/// # Examples
///
/// ```rust
/// use ratatui::prelude::*;
///
/// Line::raw("unstyled");
/// Line::styled("yellow text", Style::new().yellow());
/// Line::from("red text").style(Style::new().red());
/// Line::from(String::from("unstyled"));
/// Line::from(vec![
///     Span::styled("Hello", Style::new().blue()),
///     Span::raw(" world!"),
/// ]);
/// ```
///
/// [`Paragraph`]: crate::widgets::Paragraph
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Line<'a> {
    /// The spans that make up this line of text.
    pub spans: Vec<Span<'a>>,

    /// The style of this line of text.
    pub style: Style,

    /// The alignment of this line of text.
    pub alignment: Option<Alignment>,
}

impl<'a> Line<'a> {
    /// Create a line with the default style.
    ///
    /// `content` can be any type that is convertible to [`Cow<str>`] (e.g. [`&str`], [`String`],
    /// [`Cow<str>`], or your own type that implements [`Into<Cow<str>>`]).
    ///
    /// A [`Line`] can specify a [`Style`], which will be applied before the style of each [`Span`]
    /// in the line.
    ///
    /// Any newlines in the content are removed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # use std::borrow::Cow;
    /// Line::raw("test content");
    /// Line::raw(String::from("test content"));
    /// Line::raw(Cow::from("test content"));
    /// ```
    pub fn raw<T>(content: T) -> Line<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Line {
            spans: content
                .into()
                .lines()
                .map(|v| Span::raw(v.to_string()))
                .collect(),
            ..Default::default()
        }
    }

    /// Create a line with the given style.
    // `content` can be any type that is convertible to [`Cow<str>`] (e.g. [`&str`], [`String`],
    /// [`Cow<str>`], or your own type that implements [`Into<Cow<str>>`]).
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    ///
    /// Any newlines in the content are removed.
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # use std::borrow::Cow;
    /// let style = Style::new().yellow().italic();
    /// Line::styled("My text", style);
    /// Line::styled(String::from("My text"), style);
    /// Line::styled(Cow::from("test content"), style);
    /// ```
    pub fn styled<T, S>(content: T, style: S) -> Line<'a>
    where
        T: Into<Cow<'a, str>>,
        S: Into<Style>,
    {
        Line {
            spans: content
                .into()
                .lines()
                .map(|v| Span::raw(v.to_string()))
                .collect(),
            style: style.into(),
            ..Default::default()
        }
    }

    /// Sets the spans of this line of text.
    ///
    /// `spans` accepts any iterator that yields items that are convertible to [`Span`] (e.g.
    /// [`&str`], [`String`], [`Span`], or your own type that implements [`Into<Span>`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let line = Line::default().spans(vec!["Hello".blue(), " world!".green()]);
    /// let line = Line::default().spans([1, 2, 3].iter().map(|i| format!("Item {}", i)));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn spans<I>(mut self, spans: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Span<'a>>,
    {
        self.spans = spans.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the style of this line of text.
    ///
    /// Defaults to [`Style::default()`].
    ///
    /// Note: This field was added in v0.26.0. Prior to that, the style of a line was determined
    /// only by the style of each [`Span`] contained in the line. For this reason, this field may
    /// not be supported by all widgets (outside of the `ratatui` crate itself).
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let mut line = Line::from("foo").style(Style::new().red());
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the target alignment for this line of text.
    ///
    /// Defaults to: [`None`], meaning the alignment is determined by the rendering widget.
    /// Setting the alignment of a Line generally overrides the alignment of its
    /// parent Text or Widget.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let mut line = Line::from("Hi, what's up?");
    /// assert_eq!(None, line.alignment);
    /// assert_eq!(
    ///     Some(Alignment::Right),
    ///     line.alignment(Alignment::Right).alignment
    /// )
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn alignment(self, alignment: Alignment) -> Self {
        Self {
            alignment: Some(alignment),
            ..self
        }
    }

    /// Left-aligns this line of text.
    ///
    /// Convenience shortcut for `Line::alignment(Alignment::Left)`.
    /// Setting the alignment of a Line generally overrides the alignment of its
    /// parent Text or Widget, with the default alignment being inherited from the parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let line = Line::from("Hi, what's up?").left_aligned();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn left_aligned(self) -> Self {
        self.alignment(Alignment::Left)
    }

    /// Center-aligns this line of text.
    ///
    /// Convenience shortcut for `Line::alignment(Alignment::Center)`.
    /// Setting the alignment of a Line generally overrides the alignment of its
    /// parent Text or Widget, with the default alignment being inherited from the parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let line = Line::from("Hi, what's up?").centered();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn centered(self) -> Self {
        self.alignment(Alignment::Center)
    }

    /// Right-aligns this line of text.
    ///
    /// Convenience shortcut for `Line::alignment(Alignment::Right)`.
    /// Setting the alignment of a Line generally overrides the alignment of its
    /// parent Text or Widget, with the default alignment being inherited from the parent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let line = Line::from("Hi, what's up?").right_aligned();
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn right_aligned(self) -> Self {
        self.alignment(Alignment::Right)
    }

    /// Returns the width of the underlying string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let line = Line::from(vec!["Hello".blue(), " world!".green()]);
    /// assert_eq!(12, line.width());
    /// ```
    pub fn width(&self) -> usize {
        self.spans.iter().map(Span::width).sum()
    }

    /// Returns an iterator over the graphemes held by this line.
    ///
    /// `base_style` is the [`Style`] that will be patched with each grapheme [`Style`] to get
    /// the resulting [`Style`].
    ///
    /// `base_style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`],
    /// or your own type that implements [`Into<Style>`]).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::iter::Iterator;
    ///
    /// use ratatui::{prelude::*, text::StyledGrapheme};
    ///
    /// let line = Line::styled("Text", Style::default().fg(Color::Yellow));
    /// let style = Style::default().fg(Color::Green).bg(Color::Black);
    /// assert_eq!(
    ///     line.styled_graphemes(style)
    ///         .collect::<Vec<StyledGrapheme>>(),
    ///     vec![
    ///         StyledGrapheme::new("T", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///         StyledGrapheme::new("e", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///         StyledGrapheme::new("x", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///         StyledGrapheme::new("t", Style::default().fg(Color::Yellow).bg(Color::Black)),
    ///     ]
    /// );
    /// ```
    pub fn styled_graphemes<S: Into<Style>>(
        &'a self,
        base_style: S,
    ) -> impl Iterator<Item = StyledGrapheme<'a>> {
        let style = base_style.into().patch(self.style);
        self.spans
            .iter()
            .flat_map(move |span| span.styled_graphemes(style))
    }

    /// Patches the style of this Line, adding modifiers from the given style.
    ///
    /// This is useful for when you want to apply a style to a line that already has some styling.
    /// In contrast to [`Line::style`], this method will not overwrite the existing style, but
    /// instead will add the given style's modifiers to this Line's style.
    ///
    /// `style` accepts any type that is convertible to [`Style`] (e.g. [`Style`], [`Color`], or
    /// your own type that implements [`Into<Style>`]).
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// let line = Line::styled("My text", Modifier::ITALIC);
    ///
    /// let styled_line = Line::styled("My text", (Color::Yellow, Modifier::ITALIC));
    ///
    /// assert_eq!(styled_line, line.patch_style(Color::Yellow));
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn patch_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.style = self.style.patch(style);
        self
    }

    /// Resets the style of this Line.
    ///
    /// Equivalent to calling `patch_style(Style::reset())`.
    ///
    /// This is a fluent setter method which must be chained or used as it consumes self
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use ratatui::prelude::*;
    /// # let style = Style::default().yellow();
    /// let line = Line::styled("My text", style);
    ///
    /// assert_eq!(Style::reset(), line.reset_style().style);
    /// ```
    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn reset_style(self) -> Self {
        self.patch_style(Style::reset())
    }

    /// Returns an iterator over the spans of this line.
    pub fn iter(&self) -> std::slice::Iter<Span<'a>> {
        self.spans.iter()
    }

    /// Returns a mutable iterator over the spans of this line.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Span<'a>> {
        self.spans.iter_mut()
    }
}

impl<'a> IntoIterator for Line<'a> {
    type Item = Span<'a>;
    type IntoIter = std::vec::IntoIter<Span<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.spans.into_iter()
    }
}

impl<'a> IntoIterator for &'a Line<'a> {
    type Item = &'a Span<'a>;
    type IntoIter = std::slice::Iter<'a, Span<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Line<'a> {
    type Item = &'a mut Span<'a>;
    type IntoIter = std::slice::IterMut<'a, Span<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a> From<String> for Line<'a> {
    fn from(s: String) -> Self {
        Self::from(vec![Span::from(s)])
    }
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(s: &'a str) -> Self {
        Self::from(vec![Span::from(s)])
    }
}

impl<'a> From<Vec<Span<'a>>> for Line<'a> {
    fn from(spans: Vec<Span<'a>>) -> Self {
        Self {
            spans,
            ..Default::default()
        }
    }
}

impl<'a> From<Span<'a>> for Line<'a> {
    fn from(span: Span<'a>) -> Self {
        Self::from(vec![span])
    }
}

impl<'a> From<Line<'a>> for String {
    fn from(line: Line<'a>) -> String {
        line.iter().fold(String::new(), |mut acc, s| {
            acc.push_str(s.content.as_ref());
            acc
        })
    }
}

impl Widget for Line<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl WidgetRef for Line<'_> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        buf.set_style(area, self.style);
        let width = self.width() as u16;
        let offset = match self.alignment {
            Some(Alignment::Left) => 0,
            Some(Alignment::Center) => (area.width.saturating_sub(width)) / 2,
            Some(Alignment::Right) => area.width.saturating_sub(width),
            None => 0,
        };
        let mut x = area.left().saturating_add(offset);
        for span in self.spans.iter() {
            let span_width = span.width() as u16;
            let span_area = Rect {
                x,
                width: span_width.min(area.right() - x),
                ..area
            };
            span.render(span_area, buf);
            x = x.saturating_add(span_width);
            if x >= area.right() {
                break;
            }
        }
    }
}

impl std::fmt::Display for Line<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for span in &self.spans {
            write!(f, "{span}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[test]
    fn raw_str() {
        let line = Line::raw("test content");
        assert_eq!(line.spans, vec![Span::raw("test content")]);
        assert_eq!(line.alignment, None);

        let line = Line::raw("a\nb");
        assert_eq!(line.spans, vec![Span::raw("a"), Span::raw("b")]);
        assert_eq!(line.alignment, None);
    }

    #[test]
    fn styled_str() {
        let style = Style::new().yellow();
        let content = "Hello, world!";
        let line = Line::styled(content, style);
        assert_eq!(line.spans, vec![Span::raw(content)]);
        assert_eq!(line.style, style);
    }

    #[test]
    fn styled_string() {
        let style = Style::new().yellow();
        let content = String::from("Hello, world!");
        let line = Line::styled(content.clone(), style);
        assert_eq!(line.spans, vec![Span::raw(content)]);
        assert_eq!(line.style, style);
    }

    #[test]
    fn styled_cow() {
        let style = Style::new().yellow();
        let content = Cow::from("Hello, world!");
        let line = Line::styled(content.clone(), style);
        assert_eq!(line.spans, vec![Span::raw(content)]);
        assert_eq!(line.style, style);
    }

    #[test]
    fn spans_vec() {
        let line = Line::default().spans(vec!["Hello".blue(), " world!".green()]);
        assert_eq!(
            line.spans,
            vec![
                Span::styled("Hello", Style::new().blue()),
                Span::styled(" world!", Style::new().green()),
            ]
        );
    }

    #[test]
    fn spans_iter() {
        let line = Line::default().spans([1, 2, 3].iter().map(|i| format!("Item {i}")));
        assert_eq!(
            line.spans,
            vec![
                Span::raw("Item 1"),
                Span::raw("Item 2"),
                Span::raw("Item 3"),
            ]
        );
    }

    #[test]
    fn style() {
        let line = Line::default().style(Style::new().red());
        assert_eq!(line.style, Style::new().red());
    }

    #[test]
    fn alignment() {
        let line = Line::from("This is left").alignment(Alignment::Left);
        assert_eq!(Some(Alignment::Left), line.alignment);

        let line = Line::from("This is default");
        assert_eq!(None, line.alignment);
    }

    #[test]
    fn width() {
        let line = Line::from(vec![
            Span::styled("My", Style::default().fg(Color::Yellow)),
            Span::raw(" text"),
        ]);
        assert_eq!(7, line.width());

        let empty_line = Line::default();
        assert_eq!(0, empty_line.width());
    }

    #[test]
    fn patch_style() {
        let raw_line = Line::styled("foobar", Color::Yellow);
        let styled_line = Line::styled("foobar", (Color::Yellow, Modifier::ITALIC));

        assert_ne!(raw_line, styled_line);

        let raw_line = raw_line.patch_style(Modifier::ITALIC);
        assert_eq!(raw_line, styled_line);
    }

    #[test]
    fn reset_style() {
        let line =
            Line::styled("foobar", Style::default().yellow().on_red().italic()).reset_style();

        assert_eq!(Style::reset(), line.style);
    }

    #[test]
    fn from_string() {
        let s = String::from("Hello, world!");
        let line = Line::from(s);
        assert_eq!(vec![Span::from("Hello, world!")], line.spans);
    }

    #[test]
    fn from_str() {
        let s = "Hello, world!";
        let line = Line::from(s);
        assert_eq!(vec![Span::from("Hello, world!")], line.spans);
    }

    #[test]
    fn from_vec() {
        let spans = vec![
            Span::styled("Hello,", Style::default().fg(Color::Red)),
            Span::styled(" world!", Style::default().fg(Color::Green)),
        ];
        let line = Line::from(spans.clone());
        assert_eq!(spans, line.spans);
    }

    #[test]
    fn from_span() {
        let span = Span::styled("Hello, world!", Style::default().fg(Color::Yellow));
        let line = Line::from(span.clone());
        assert_eq!(vec![span], line.spans);
    }

    #[test]
    fn into_string() {
        let line = Line::from(vec![
            Span::styled("Hello,", Style::default().fg(Color::Red)),
            Span::styled(" world!", Style::default().fg(Color::Green)),
        ]);
        let s: String = line.into();
        assert_eq!("Hello, world!", s);
    }

    #[test]
    fn styled_graphemes() {
        const RED: Style = Style::new().fg(Color::Red);
        const GREEN: Style = Style::new().fg(Color::Green);
        const BLUE: Style = Style::new().fg(Color::Blue);
        const RED_ON_WHITE: Style = Style::new().fg(Color::Red).bg(Color::White);
        const GREEN_ON_WHITE: Style = Style::new().fg(Color::Green).bg(Color::White);
        const BLUE_ON_WHITE: Style = Style::new().fg(Color::Blue).bg(Color::White);

        let line = Line::from(vec![
            Span::styled("He", RED),
            Span::styled("ll", GREEN),
            Span::styled("o!", BLUE),
        ]);
        let styled_graphemes = line
            .styled_graphemes(Style::new().bg(Color::White))
            .collect::<Vec<StyledGrapheme>>();
        assert_eq!(
            styled_graphemes,
            vec![
                StyledGrapheme::new("H", RED_ON_WHITE),
                StyledGrapheme::new("e", RED_ON_WHITE),
                StyledGrapheme::new("l", GREEN_ON_WHITE),
                StyledGrapheme::new("l", GREEN_ON_WHITE),
                StyledGrapheme::new("o", BLUE_ON_WHITE),
                StyledGrapheme::new("!", BLUE_ON_WHITE),
            ],
        );
    }

    #[test]
    fn display_line_from_vec() {
        let line_from_vec = Line::from(vec![Span::raw("Hello,"), Span::raw(" world!")]);

        assert_eq!(format!("{line_from_vec}"), "Hello, world!");
    }

    #[test]
    fn display_styled_line() {
        let styled_line = Line::styled("Hello, world!", Style::new().green().italic());

        assert_eq!(format!("{styled_line}"), "Hello, world!");
    }

    #[test]
    fn display_line_from_styled_span() {
        let styled_span = Span::styled("Hello, world!", Style::new().green().italic());
        let line_from_styled_span = Line::from(styled_span);

        assert_eq!(format!("{line_from_styled_span}"), "Hello, world!");
    }

    mod widget {
        use super::*;
        use crate::assert_buffer_eq;
        const BLUE: Style = Style::new().fg(Color::Blue);
        const GREEN: Style = Style::new().fg(Color::Green);
        const ITALIC: Style = Style::new().add_modifier(Modifier::ITALIC);

        fn hello_world() -> Line<'static> {
            Line::from(vec![
                Span::styled("Hello ", BLUE),
                Span::styled("world!", GREEN),
            ])
            .style(ITALIC)
        }

        #[test]
        fn render() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 1));
            hello_world().render(Rect::new(0, 0, 15, 1), &mut buf);
            let mut expected = Buffer::with_lines(vec!["Hello world!   "]);
            expected.set_style(Rect::new(0, 0, 15, 1), ITALIC);
            expected.set_style(Rect::new(0, 0, 6, 1), BLUE);
            expected.set_style(Rect::new(6, 0, 6, 1), GREEN);
            assert_buffer_eq!(buf, expected);
        }

        #[test]
        fn render_only_styles_line_area() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 20, 1));
            hello_world().render(Rect::new(0, 0, 15, 1), &mut buf);
            let mut expected = Buffer::with_lines(vec!["Hello world!        "]);
            expected.set_style(Rect::new(0, 0, 15, 1), ITALIC);
            expected.set_style(Rect::new(0, 0, 6, 1), BLUE);
            expected.set_style(Rect::new(6, 0, 6, 1), GREEN);
            assert_buffer_eq!(buf, expected);
        }

        #[test]
        fn render_truncates() {
            let mut buf = Buffer::empty(Rect::new(0, 0, 10, 1));
            Line::from("Hello world!").render(Rect::new(0, 0, 5, 1), &mut buf);
            let expected = Buffer::with_lines(vec!["Hello     "]);
            assert_buffer_eq!(buf, expected);
        }

        #[test]
        fn render_centered() {
            let line = hello_world().alignment(Alignment::Center);
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 1));
            line.render(Rect::new(0, 0, 15, 1), &mut buf);
            let mut expected = Buffer::with_lines(vec![" Hello world!  "]);
            expected.set_style(Rect::new(0, 0, 15, 1), ITALIC);
            expected.set_style(Rect::new(1, 0, 6, 1), BLUE);
            expected.set_style(Rect::new(7, 0, 6, 1), GREEN);
            assert_buffer_eq!(buf, expected);
        }

        #[test]
        fn render_right_aligned() {
            let line = hello_world().alignment(Alignment::Right);
            let mut buf = Buffer::empty(Rect::new(0, 0, 15, 1));
            line.render(Rect::new(0, 0, 15, 1), &mut buf);
            let mut expected = Buffer::with_lines(vec!["   Hello world!"]);
            expected.set_style(Rect::new(0, 0, 15, 1), ITALIC);
            expected.set_style(Rect::new(3, 0, 6, 1), BLUE);
            expected.set_style(Rect::new(9, 0, 6, 1), GREEN);
            assert_buffer_eq!(buf, expected);
        }
    }

    #[test]
    fn left_aligned() {
        let line = Line::from("Hello, world!").left_aligned();
        assert_eq!(line.alignment, Some(Alignment::Left));
    }

    #[test]
    fn centered() {
        let line = Line::from("Hello, world!").centered();
        assert_eq!(line.alignment, Some(Alignment::Center));
    }

    #[test]
    fn right_aligned() {
        let line = Line::from("Hello, world!").right_aligned();
        assert_eq!(line.alignment, Some(Alignment::Right));
    }

    mod iterators {
        use super::*;

        /// a fixture used in the tests below to avoid repeating the same setup
        #[fixture]
        fn hello_world() -> Line<'static> {
            Line::from(vec![
                Span::styled("Hello ", Color::Blue),
                Span::styled("world!", Color::Green),
            ])
        }

        #[rstest]
        fn iter(hello_world: Line<'_>) {
            let mut iter = hello_world.iter();
            assert_eq!(iter.next(), Some(&Span::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(&Span::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn iter_mut(mut hello_world: Line<'_>) {
            let mut iter = hello_world.iter_mut();
            assert_eq!(iter.next(), Some(&mut Span::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(&mut Span::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn into_iter(hello_world: Line<'_>) {
            let mut iter = hello_world.into_iter();
            assert_eq!(iter.next(), Some(Span::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(Span::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn into_iter_ref(hello_world: Line<'_>) {
            let mut iter = (&hello_world).into_iter();
            assert_eq!(iter.next(), Some(&Span::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(&Span::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn into_iter_mut_ref() {
            let mut hello_world = Line::from(vec![
                Span::styled("Hello ", Color::Blue),
                Span::styled("world!", Color::Green),
            ]);
            let mut iter = (&mut hello_world).into_iter();
            assert_eq!(iter.next(), Some(&mut Span::styled("Hello ", Color::Blue)));
            assert_eq!(iter.next(), Some(&mut Span::styled("world!", Color::Green)));
            assert_eq!(iter.next(), None);
        }

        #[rstest]
        fn for_loop_ref(hello_world: Line<'_>) {
            let mut result = String::new();
            for span in &hello_world {
                result.push_str(span.content.as_ref());
            }
            assert_eq!(result, "Hello world!");
        }

        #[rstest]
        fn for_loop_mut_ref() {
            let mut hello_world = Line::from(vec![
                Span::styled("Hello ", Color::Blue),
                Span::styled("world!", Color::Green),
            ]);
            let mut result = String::new();
            for span in &mut hello_world {
                result.push_str(span.content.as_ref());
            }
            assert_eq!(result, "Hello world!");
        }

        #[rstest]
        fn for_loop_into(hello_world: Line<'_>) {
            let mut result = String::new();
            for span in hello_world {
                result.push_str(span.content.as_ref());
            }
            assert_eq!(result, "Hello world!");
        }
    }
}
