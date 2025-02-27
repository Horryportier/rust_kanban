use ratatui::style::{Color, Modifier, Style};

pub const FIELD_NOT_SET: &str = "Not Set";
pub const CONFIG_FILE_NAME: &str = "config.json";
pub const CONFIG_DIR_NAME: &str = "rust_kanban";
pub const SAVE_DIR_NAME: &str = "kanban_saves";
pub const SAVE_FILE_NAME: &str = "kanban";
pub const NO_OF_BOARDS_PER_PAGE: u16 = 3;
pub const MIN_NO_BOARDS_PER_PAGE: u16 = 1;
pub const MAX_NO_BOARDS_PER_PAGE: u16 = 5;
pub const NO_OF_CARDS_PER_BOARD: u16 = 2;
pub const MIN_NO_CARDS_PER_BOARD: u16 = 1;
pub const MAX_NO_CARDS_PER_BOARD: u16 = 4;
pub const DEFAULT_BOARD_TITLE_LENGTH: u16 = 20;
pub const DEFAULT_CARD_TITLE_LENGTH: u16 = 20;
pub const APP_TITLE: &str = "Rust 🦀 Kanban";
pub const MIN_TERM_WIDTH: u16 = 110;
pub const MIN_TERM_HEIGHT: u16 = 30;
pub const LIST_SELECTED_SYMBOL: &str = ">> ";
pub const VERTICAL_SCROLL_BAR_SYMBOL: &str = "█";
pub const DEFAULT_CARD_WARNING_DUE_DATE_DAYS: u16 = 3;
pub const MAX_TOASTS_TO_DISPLAY: usize = 5;
pub const SCREEN_TO_TOAST_WIDTH_RATIO: u16 = 3; // 1/3rd of the screen width
pub const TOAST_FADE_OUT_TIME: u64 = 400;
pub const TOAST_FADE_IN_TIME: u64 = 200;
pub const DEFAULT_TICKRATE: u64 = 50;
pub const DEFAULT_TOAST_DURATION: u64 = 5;
pub const IO_EVENT_WAIT_TIME: u64 = 5; // ms
pub const MOUSE_OUT_OF_BOUNDS_COORDINATES: (u16, u16) = (9999, 9999);
pub const NEW_CARD_FORM_DEFAULT_STATE: [&str; 3] = ["", "", ""];
pub const NEW_BOARD_FORM_DEFAULT_STATE: [&str; 2] = ["", ""];
pub const SAMPLE_TEXT: &str = "Sample Text";
pub const THEME_DIR_NAME: &str = "themes";
pub const THEME_FILE_NAME: &str = "kanban_theme";
pub const RANDOM_SEARCH_TERM: &str = "iibnigivirneiivure";

// Style
pub const GENERAL_STYLE: Style = Style {
    fg: Some(Color::White),
    bg: Some(Color::Reset),
    add_modifier: Modifier::empty(),
    sub_modifier: Modifier::empty(),
};
pub const LIST_SELECT_STYLE: Style = Style {
    fg: Some(Color::White),
    bg: Some(Color::LightMagenta),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const CARD_DUE_DATE_DEFAULT_STYLE: Style = Style {
    fg: Some(Color::LightGreen),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const CARD_DUE_DATE_WARNING_STYLE: Style = Style {
    fg: Some(Color::LightYellow),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const CARD_DUE_DATE_CRITICAL_STYLE: Style = Style {
    fg: Some(Color::LightRed),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const CARD_ACTIVE_STATUS_STYLE: Style = Style {
    fg: Some(Color::LightCyan),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const CARD_COMPLETED_STATUS_STYLE: Style = Style {
    fg: Some(Color::LightGreen),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const CARD_STALE_STATUS_STYLE: Style = Style {
    fg: Some(Color::DarkGray),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const KEYBOARD_FOCUS_STYLE: Style = Style {
    fg: Some(Color::LightCyan),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const MOUSE_HIGHLIGHT_STYLE: Style = Style {
    fg: Some(Color::Rgb(255, 165, 0)),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const HELP_KEY_STYLE: Style = Style {
    fg: Some(Color::LightCyan),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const LOG_ERROR_STYLE: Style = Style {
    fg: Some(Color::LightRed),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const LOG_DEBUG_STYLE: Style = Style {
    fg: Some(Color::LightGreen),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const LOG_WARN_STYLE: Style = Style {
    fg: Some(Color::LightYellow),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const LOG_TRACE_STYLE: Style = Style {
    fg: Some(Color::Gray),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const LOG_INFO_STYLE: Style = Style {
    fg: Some(Color::LightCyan),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const PROGRESS_BAR_STYLE: Style = Style {
    fg: Some(Color::LightGreen),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const ERROR_TEXT_STYLE: Style = Style {
    fg: Some(Color::LightRed),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const INACTIVE_TEXT_STYLE: Style = Style {
    fg: Some(Color::Rgb(40, 40, 40)),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const CARD_PRIORITY_LOW_STYLE: Style = Style {
    fg: Some(Color::LightGreen),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const CARD_PRIORITY_MEDIUM_STYLE: Style = Style {
    fg: Some(Color::LightYellow),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const CARD_PRIORITY_HIGH_STYLE: Style = Style {
    fg: Some(Color::LightRed),
    bg: Some(Color::Reset),
    add_modifier: Modifier::BOLD,
    sub_modifier: Modifier::empty(),
};
pub const SPINNER_FRAMES: [&str; 7] = [
    "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]",
];
