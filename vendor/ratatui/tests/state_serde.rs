//! State like [`ListState`], [`TableState`] and [`ScrollbarState`] can be serialized and
//! deserialized through serde. This allows saving your entire state to disk when the user exits the
//! the app, and restore it again upon re-opening the app.
//! This way, they get right back to where they were, without having to re-seek to their previous
//! position, if that's applicable for the app at hand.
//!
//! **Note**: For this pattern to work easily, you need to have some toplevel struct which stores
//! _only_ state and not any draw commands.
//!
//! **Note**: For many applications, it might be beneficial to instead keep your own state and
//! instead construct the state for widgets on the fly instead, if that allows you to express you
//! the semantic meaning of your state better or only fetch part of a dataset.

// not too happy about the redundancy in these tests,
// but if that helps readability then it's ok i guess /shrug

use ratatui::{backend::TestBackend, prelude::*, widgets::*};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct AppState {
    list_state: ListState,
    table_state: TableState,
    scrollbar_state: ScrollbarState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            list_state: ListState::default(),
            table_state: TableState::default(),
            scrollbar_state: ScrollbarState::new(10),
        }
    }
}
impl AppState {
    fn select(&mut self, index: usize) {
        self.list_state.select(Some(index));
        self.table_state.select(Some(index));
        self.scrollbar_state = self.scrollbar_state.position(index);
    }
}

/// Renders the list to a TestBackend and asserts that the result matches the expected buffer.
#[track_caller]
fn assert_buffer(state: &mut AppState, expected: &Buffer) {
    let backend = TestBackend::new(21, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|f| {
            let items = vec![
                "awa", "banana", "Cats!!", "d20", "Echo", "Foxtrot", "Golf", "Hotel", "IwI",
                "Juliett",
            ];

            use Constraint::*;
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Length(10), Length(10), Length(1)])
                .split(f.size());
            let list = List::new(items.clone())
                .highlight_symbol(">>")
                .block(Block::default().borders(Borders::RIGHT));
            f.render_stateful_widget(list, layout[0], &mut state.list_state);

            let table = Table::new(
                items.iter().map(|i| Row::new(vec![*i])),
                [Constraint::Length(10); 1],
            )
            .highlight_symbol(">>");
            f.render_stateful_widget(table, layout[1], &mut state.table_state);

            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
            f.render_stateful_widget(scrollbar, layout[2], &mut state.scrollbar_state);
        })
        .unwrap();
    terminal.backend().assert_buffer(expected);
}

const DEFAULT_STATE_BUFFER: [&str; 5] = [
    "awa      │awa       ▲",
    "banana   │banana    █",
    "Cats!!   │Cats!!    ║",
    "d20      │d20       ║",
    "Echo     │Echo      ▼",
];

const DEFAULT_STATE_REPR: &str = r#"{
  "list_state": {
    "offset": 0,
    "selected": null
  },
  "table_state": {
    "offset": 0,
    "selected": null
  },
  "scrollbar_state": {
    "content_length": 10,
    "position": 0,
    "viewport_content_length": 0
  }
}"#;

#[test]
fn default_state_serialize() {
    let mut state = AppState::default();

    let expected = Buffer::with_lines(DEFAULT_STATE_BUFFER);
    assert_buffer(&mut state, &expected);

    let state = serde_json::to_string_pretty(&state).unwrap();
    assert_eq!(state, DEFAULT_STATE_REPR);
}

#[test]
fn default_state_deserialize() {
    let expected = Buffer::with_lines(DEFAULT_STATE_BUFFER);
    let mut state: AppState = serde_json::from_str(DEFAULT_STATE_REPR).unwrap();
    assert_buffer(&mut state, &expected);
}

const SELECTED_STATE_BUFFER: [&str; 5] = [
    "  awa    │  awa     ▲",
    ">>banana │>>banana  █",
    "  Cats!! │  Cats!!  ║",
    "  d20    │  d20     ║",
    "  Echo   │  Echo    ▼",
];
const SELECTED_STATE_REPR: &str = r#"{
  "list_state": {
    "offset": 0,
    "selected": 1
  },
  "table_state": {
    "offset": 0,
    "selected": 1
  },
  "scrollbar_state": {
    "content_length": 10,
    "position": 1,
    "viewport_content_length": 0
  }
}"#;

#[test]
fn selected_state_serialize() {
    let mut state = AppState::default();
    state.select(1);

    let expected = Buffer::with_lines(SELECTED_STATE_BUFFER);
    assert_buffer(&mut state, &expected);

    let state = serde_json::to_string_pretty(&state).unwrap();
    assert_eq!(state, SELECTED_STATE_REPR);
}

#[test]
fn selected_state_deserialize() {
    let expected = Buffer::with_lines(SELECTED_STATE_BUFFER);
    let mut state: AppState = serde_json::from_str(SELECTED_STATE_REPR).unwrap();
    assert_buffer(&mut state, &expected);
}

const SCROLLED_STATE_BUFFER: [&str; 5] = [
    "  Echo   │  Echo    ▲",
    "  Foxtrot│  Foxtrot ║",
    "  Golf   │  Golf    ║",
    "  Hotel  │  Hotel   █",
    ">>IwI    │>>IwI     ▼",
];

const SCROLLED_STATE_REPR: &str = r#"{
  "list_state": {
    "offset": 4,
    "selected": 8
  },
  "table_state": {
    "offset": 4,
    "selected": 8
  },
  "scrollbar_state": {
    "content_length": 10,
    "position": 8,
    "viewport_content_length": 0
  }
}"#;

#[test]
fn scrolled_state_serialize() {
    let mut state = AppState::default();
    state.select(8);

    let expected = Buffer::with_lines(SCROLLED_STATE_BUFFER);
    assert_buffer(&mut state, &expected);

    let state = serde_json::to_string_pretty(&state).unwrap();
    assert_eq!(state, SCROLLED_STATE_REPR);
}

#[test]
fn scrolled_state_deserialize() {
    let expected = Buffer::with_lines(SCROLLED_STATE_BUFFER);
    let mut state: AppState = serde_json::from_str(SCROLLED_STATE_REPR).unwrap();
    assert_buffer(&mut state, &expected);
}
