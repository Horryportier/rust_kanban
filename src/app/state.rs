use std::{fmt, str::FromStr, vec};

use log::error;
use serde::{Deserialize, Serialize};

use super::actions::Action;
use crate::inputs::key::Key;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Copy, Default)]
pub enum UiMode {
    #[default]
    Zen,
    TitleBody,
    BodyHelp,
    BodyLog,
    TitleBodyHelp,
    TitleBodyLog,
    TitleBodyHelpLog,
    BodyHelpLog,
    ConfigMenu,
    EditKeybindings,
    MainMenu,
    HelpMenu,
    LogsOnly,
    NewBoard,
    NewCard,
    LoadSave,
    CreateTheme,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum AppStatus {
    #[default]
    Init,
    Initialized,
    UserInput,
    KeyBindMode,
}

#[derive(Clone, PartialEq, Debug, Copy, Default)]
pub enum Focus {
    Title,
    Body,
    Help,
    Log,
    ConfigTable,
    ConfigHelp,
    MainMenu,
    MainMenuHelp,
    NewBoardName,
    NewBoardDescription,
    CardName,
    CardDescription,
    CardDueDate,
    SubmitButton,
    EditKeybindingsTable,
    CloseButton,
    CommandPaletteCommand,
    CommandPaletteCard,
    CommandPaletteBoard,
    LoadSave,
    SelectDefaultView,
    ChangeUiModePopup,
    ChangeCardStatusPopup,
    EditGeneralConfigPopup,
    EditSpecificKeyBindingPopup,
    ThemeSelector,
    ThemeEditor,
    StyleEditorFG,
    StyleEditorBG,
    StyleEditorModifier,
    TextInput,
    CardPriority,
    CardStatus,
    CardTags,
    CardComments,
    ChangeCardPriorityPopup,
    ChangeDateFormatPopup,
    FilterByTagPopup,
    #[default]
    NoFocus,
    ExtraFocus, // Used in cases where defining a new focus is not necessary
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyBindings {
    pub quit: Vec<Key>,
    pub open_config_menu: Vec<Key>,
    pub up: Vec<Key>,
    pub down: Vec<Key>,
    pub right: Vec<Key>,
    pub left: Vec<Key>,
    pub next_focus: Vec<Key>,
    pub prev_focus: Vec<Key>,
    pub take_user_input: Vec<Key>,
    pub stop_user_input: Vec<Key>,
    pub hide_ui_element: Vec<Key>,
    pub save_state: Vec<Key>,
    pub new_board: Vec<Key>,
    pub new_card: Vec<Key>,
    pub delete_board: Vec<Key>,
    pub delete_card: Vec<Key>,
    pub change_card_status_to_completed: Vec<Key>,
    pub change_card_status_to_active: Vec<Key>,
    pub change_card_status_to_stale: Vec<Key>,
    pub reset_ui: Vec<Key>,
    pub go_to_main_menu: Vec<Key>,
    pub toggle_command_palette: Vec<Key>,
    pub clear_all_toasts: Vec<Key>,
    pub undo: Vec<Key>,
    pub redo: Vec<Key>,
}

impl UiMode {
    pub fn from_string(s: &str) -> Option<UiMode> {
        match s {
            "Zen" => Some(UiMode::Zen),
            "Title and Body" => Some(UiMode::TitleBody),
            "Body and Help" => Some(UiMode::BodyHelp),
            "Body and Log" => Some(UiMode::BodyLog),
            "Title, Body and Help" => Some(UiMode::TitleBodyHelp),
            "Title, Body and Log" => Some(UiMode::TitleBodyLog),
            "Body, Help and Log" => Some(UiMode::BodyHelpLog),
            "Title, Body, Help and Log" => Some(UiMode::TitleBodyHelpLog),
            "Config" => Some(UiMode::ConfigMenu),
            "Edit Keybindings" => Some(UiMode::EditKeybindings),
            "Main Menu" => Some(UiMode::MainMenu),
            "Help Menu" => Some(UiMode::HelpMenu),
            "Logs Only" => Some(UiMode::LogsOnly),
            "New Board" => Some(UiMode::NewBoard),
            "New Card" => Some(UiMode::NewCard),
            "Load a Save" => Some(UiMode::LoadSave),
            "Create Theme" => Some(UiMode::CreateTheme),
            _ => None,
        }
    }

    pub fn from_number(n: u8) -> UiMode {
        match n {
            1 => UiMode::Zen,
            2 => UiMode::TitleBody,
            3 => UiMode::BodyHelp,
            4 => UiMode::BodyLog,
            5 => UiMode::TitleBodyHelp,
            6 => UiMode::TitleBodyLog,
            7 => UiMode::BodyHelpLog,
            8 => UiMode::TitleBodyHelpLog,
            9 => UiMode::LogsOnly,
            _ => {
                error!("Invalid UiMode: {}", n);
                UiMode::TitleBody
            }
        }
    }

    pub fn get_available_targets(&self) -> Vec<Focus> {
        match self {
            UiMode::Zen => vec![Focus::Body],
            UiMode::TitleBody => vec![Focus::Title, Focus::Body],
            UiMode::BodyHelp => vec![Focus::Body, Focus::Help],
            UiMode::BodyLog => vec![Focus::Body, Focus::Log],
            UiMode::TitleBodyHelp => vec![Focus::Title, Focus::Body, Focus::Help],
            UiMode::TitleBodyLog => vec![Focus::Title, Focus::Body, Focus::Log],
            UiMode::BodyHelpLog => vec![Focus::Body, Focus::Help, Focus::Log],
            UiMode::TitleBodyHelpLog => vec![Focus::Title, Focus::Body, Focus::Help, Focus::Log],
            UiMode::ConfigMenu => vec![Focus::ConfigTable, Focus::SubmitButton, Focus::ExtraFocus],
            UiMode::EditKeybindings => vec![
                Focus::Title,
                Focus::EditKeybindingsTable,
                Focus::SubmitButton,
            ],
            UiMode::MainMenu => vec![Focus::MainMenu, Focus::MainMenuHelp, Focus::Log],
            UiMode::HelpMenu => vec![Focus::Help, Focus::Log],
            UiMode::LogsOnly => vec![Focus::Log],
            UiMode::NewBoard => vec![
                Focus::NewBoardName,
                Focus::NewBoardDescription,
                Focus::SubmitButton,
            ],
            UiMode::NewCard => vec![
                Focus::CardName,
                Focus::CardDescription,
                Focus::CardDueDate,
                Focus::SubmitButton,
            ],
            UiMode::LoadSave => vec![Focus::Body],
            UiMode::CreateTheme => vec![Focus::ThemeEditor, Focus::SubmitButton, Focus::ExtraFocus],
        }
    }

    pub fn all() -> Vec<String> {
        let mut s = vec![];
        for i in 1..10 {
            s.push(UiMode::from_number(i).to_string());
        }
        s
    }

    pub fn view_modes() -> Vec<UiMode> {
        vec![
            UiMode::Zen,
            UiMode::TitleBody,
            UiMode::BodyHelp,
            UiMode::BodyLog,
            UiMode::TitleBodyHelp,
            UiMode::TitleBodyLog,
            UiMode::BodyHelpLog,
            UiMode::TitleBodyHelpLog,
        ]
    }
}

impl fmt::Display for UiMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UiMode::Zen => write!(f, "Zen"),
            UiMode::TitleBody => write!(f, "Title and Body"),
            UiMode::BodyHelp => write!(f, "Body and Help"),
            UiMode::BodyLog => write!(f, "Body and Log"),
            UiMode::TitleBodyHelp => write!(f, "Title, Body and Help"),
            UiMode::TitleBodyLog => write!(f, "Title, Body and Log"),
            UiMode::BodyHelpLog => write!(f, "Body, Help and Log"),
            UiMode::TitleBodyHelpLog => write!(f, "Title, Body, Help and Log"),
            UiMode::ConfigMenu => write!(f, "Config"),
            UiMode::EditKeybindings => write!(f, "Edit Keybindings"),
            UiMode::MainMenu => write!(f, "Main Menu"),
            UiMode::HelpMenu => write!(f, "Help Menu"),
            UiMode::LogsOnly => write!(f, "Logs Only"),
            UiMode::NewBoard => write!(f, "New Board"),
            UiMode::NewCard => write!(f, "New Card"),
            UiMode::LoadSave => write!(f, "Load a Save"),
            UiMode::CreateTheme => write!(f, "Create Theme"),
        }
    }
}

impl AppStatus {
    pub fn initialized() -> Self {
        Self::Initialized
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }
}

impl Focus {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Title => "Title",
            Self::Body => "Body",
            Self::Help => "Help",
            Self::Log => "Log",
            Self::ConfigTable => "Config",
            Self::ConfigHelp => "Config Help",
            Self::MainMenu => "Main Menu",
            Self::MainMenuHelp => "Main Menu Help",
            Self::NewBoardName => "New Board Name",
            Self::NewBoardDescription => "New Board Description",
            Self::CardName => "New Card Name",
            Self::CardDescription => "Card Description",
            Self::CardDueDate => "Card Due Date",
            Self::SubmitButton => "Submit Button",
            Self::EditKeybindingsTable => "Edit Keybindings Table",
            Self::CloseButton => "Close Button",
            Self::CommandPaletteCommand => "Command Palette Command",
            Self::CommandPaletteCard => "Command Palette Card",
            Self::CommandPaletteBoard => "Command Palette Board",
            Self::LoadSave => "Load Save",
            Self::SelectDefaultView => "Select Default View",
            Self::ChangeUiModePopup => "Change Ui Mode Popup",
            Self::ChangeCardStatusPopup => "Change Card Status Popup",
            Self::EditGeneralConfigPopup => "Edit General Config Popup",
            Self::EditSpecificKeyBindingPopup => "Edit Specific Key Binding Popup",
            Self::ThemeSelector => "Theme Selector",
            Self::ThemeEditor => "Theme Editor",
            Self::StyleEditorFG => "Theme Editor FG",
            Self::StyleEditorBG => "Theme Editor BG",
            Self::StyleEditorModifier => "Theme Editor Modifier",
            Self::TextInput => "Text Input",
            Self::CardPriority => "Card Priority",
            Self::CardStatus => "Card Status",
            Self::CardTags => "Card Tags",
            Self::CardComments => "Card Comments",
            Self::ChangeCardPriorityPopup => "Change Card Priority Popup",
            Self::ChangeDateFormatPopup => "Change Date Format Popup",
            Self::FilterByTagPopup => "Filter By Tag Popup",
            Self::NoFocus => "No Focus",
            Self::ExtraFocus => "Extra Focus",
        }
    }
    pub fn next(&self, available_tabs: &Vec<Focus>) -> Self {
        // check if current_focus is in available_tabs if not set to first available tab other wise find next tab
        if available_tabs.contains(self) {
            let index = available_tabs.iter().position(|x| x == self).unwrap();
            if index == available_tabs.len() - 1 {
                available_tabs[0]
            } else {
                available_tabs[index + 1]
            }
        } else {
            available_tabs[0]
        }
    }
    pub fn prev(&self, available_tabs: &Vec<Focus>) -> Self {
        // check if current_focus is in available_tabs if not set to first available tab other wise find next tab
        if available_tabs.contains(self) {
            let index = available_tabs.iter().position(|x| x == self).unwrap();
            if index == 0 {
                available_tabs[available_tabs.len() - 1]
            } else {
                available_tabs[index - 1]
            }
        } else {
            available_tabs[0]
        }
    }
}

impl FromStr for Focus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Title" => Ok(Self::Title),
            "Body" => Ok(Self::Body),
            "Help" => Ok(Self::Help),
            "Log" => Ok(Self::Log),
            "Config" => Ok(Self::ConfigTable),
            "Config Help" => Ok(Self::ConfigHelp),
            "Main Menu" => Ok(Self::MainMenu),
            "Main Menu Help" => Ok(Self::MainMenuHelp),
            "No Focus" => Ok(Self::NoFocus),
            "New Board Name" => Ok(Self::NewBoardName),
            "New Board Description" => Ok(Self::NewBoardDescription),
            "New Card Name" => Ok(Self::CardName),
            "Card Description" => Ok(Self::CardDescription),
            "Card Due Date" => Ok(Self::CardDueDate),
            "Edit Keybindings Table" => Ok(Self::EditKeybindingsTable),
            "Close Button" => Ok(Self::CloseButton),
            "Command Palette Command" => Ok(Self::CommandPaletteCommand),
            "Command Palette Card" => Ok(Self::CommandPaletteCard),
            "Command Palette Board" => Ok(Self::CommandPaletteBoard),
            "Load Save" => Ok(Self::LoadSave),
            "Select Default View" => Ok(Self::SelectDefaultView),
            "Change Ui Mode Popup" => Ok(Self::ChangeUiModePopup),
            "Change Card Status Popup" => Ok(Self::ChangeCardStatusPopup),
            "Edit General Config Popup" => Ok(Self::EditGeneralConfigPopup),
            "Edit Specific Key Binding Popup" => Ok(Self::EditSpecificKeyBindingPopup),
            "Theme Selector" => Ok(Self::ThemeSelector),
            "Theme Editor" => Ok(Self::ThemeEditor),
            "Theme Editor FG" => Ok(Self::StyleEditorFG),
            "Theme Editor BG" => Ok(Self::StyleEditorBG),
            "Theme Editor Modifier" => Ok(Self::StyleEditorModifier),
            "Text Input" => Ok(Self::TextInput),
            "Card Priority" => Ok(Self::CardPriority),
            "Card Status" => Ok(Self::CardStatus),
            "Card Tags" => Ok(Self::CardTags),
            "Card Comments" => Ok(Self::CardComments),
            "Change Card Priority Popup" => Ok(Self::ChangeCardPriorityPopup),
            "Filter By Tag Popup" => Ok(Self::FilterByTagPopup),
            "Submit Button" => Ok(Self::SubmitButton),
            "Extra Focus" => Ok(Self::ExtraFocus),
            _ => Ok(Self::NoFocus),
        }
    }
}

impl KeyBindings {
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Vec<Key>)> {
        vec![
            ("quit", &self.quit),
            ("next_focus", &self.next_focus),
            ("prev_focus", &self.prev_focus),
            ("open_config_menu", &self.open_config_menu),
            ("up", &self.up),
            ("down", &self.down),
            ("right", &self.right),
            ("left", &self.left),
            ("take_user_input", &self.take_user_input),
            ("stop_user_input", &self.stop_user_input),
            ("hide_ui_element", &self.hide_ui_element),
            ("save_state", &self.save_state),
            ("new_board", &self.new_board),
            ("new_card", &self.new_card),
            ("delete_card", &self.delete_card),
            ("delete_board", &self.delete_board),
            (
                "change_card_status_to_completed",
                &self.change_card_status_to_completed,
            ),
            (
                "change_card_status_to_active",
                &self.change_card_status_to_active,
            ),
            (
                "change_card_status_to_stale",
                &self.change_card_status_to_stale,
            ),
            ("reset_ui", &self.reset_ui),
            ("go_to_main_menu", &self.go_to_main_menu),
            ("toggle_command_palette", &self.toggle_command_palette),
            ("clear_all_toasts", &self.clear_all_toasts),
            ("undo", &self.undo),
            ("redo", &self.redo),
        ]
        .into_iter()
    }

    pub fn key_to_action(self, key: Key) -> Option<&'static Action> {
        for (action, keys) in self.iter() {
            if keys.contains(&key) {
                match action {
                    "quit" => return Some(&Action::Quit),
                    "next_focus" => return Some(&Action::NextFocus),
                    "prev_focus" => return Some(&Action::PrvFocus),
                    "open_config_menu" => return Some(&Action::OpenConfigMenu),
                    "up" => return Some(&Action::Up),
                    "down" => return Some(&Action::Down),
                    "right" => return Some(&Action::Right),
                    "left" => return Some(&Action::Left),
                    "take_user_input" => return Some(&Action::TakeUserInput),
                    "stop_user_input" => return Some(&Action::StopUserInput),
                    "hide_ui_element" => return Some(&Action::HideUiElement),
                    "save_state" => return Some(&Action::SaveState),
                    "new_board" => return Some(&Action::NewBoard),
                    "new_card" => return Some(&Action::NewCard),
                    "delete_card" => return Some(&Action::DeleteCard),
                    "delete_board" => return Some(&Action::DeleteBoard),
                    "change_card_status_to_completed" => {
                        return Some(&Action::ChangeCardStatusToCompleted)
                    }
                    "change_card_status_to_active" => {
                        return Some(&Action::ChangeCardStatusToActive)
                    }
                    "change_card_status_to_stale" => return Some(&Action::ChangeCardStatusToStale),
                    "reset_ui" => return Some(&Action::ResetUI),
                    "go_to_main_menu" => return Some(&Action::GoToMainMenu),
                    "toggle_command_palette" => return Some(&Action::ToggleCommandPalette),
                    "clear_all_toasts" => return Some(&Action::ClearAllToasts),
                    "undo" => return Some(&Action::Undo),
                    "redo" => return Some(&Action::Redo),
                    _ => return None,
                }
            }
        }
        None
    }

    pub fn str_to_action(self, action: &str) -> Option<&'static Action> {
        match action {
            "quit" => Some(&Action::Quit),
            "next_focus" => Some(&Action::NextFocus),
            "prev_focus" => Some(&Action::PrvFocus),
            "open_config_menu" => Some(&Action::OpenConfigMenu),
            "up" => Some(&Action::Up),
            "down" => Some(&Action::Down),
            "right" => Some(&Action::Right),
            "left" => Some(&Action::Left),
            "take_user_input" => Some(&Action::TakeUserInput),
            "stop_user_input" => Some(&Action::StopUserInput),
            "hide_ui_element" => Some(&Action::HideUiElement),
            "save_state" => Some(&Action::SaveState),
            "new_board" => Some(&Action::NewBoard),
            "new_card" => Some(&Action::NewCard),
            "delete_card" => Some(&Action::DeleteCard),
            "delete_board" => Some(&Action::DeleteBoard),
            "change_card_status_to_completed" => Some(&Action::ChangeCardStatusToCompleted),
            "change_card_status_to_active" => Some(&Action::ChangeCardStatusToActive),
            "change_card_status_to_stale" => Some(&Action::ChangeCardStatusToStale),
            "reset_ui" => Some(&Action::ResetUI),
            "go_to_main_menu" => Some(&Action::GoToMainMenu),
            "toggle_command_palette" => Some(&Action::ToggleCommandPalette),
            "clear_all_toasts" => Some(&Action::ClearAllToasts),
            "undo" => Some(&Action::Undo),
            "redo" => Some(&Action::Redo),
            _ => None,
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: vec![Key::Ctrl('c'), Key::Char('q')],
            next_focus: vec![Key::Tab],
            prev_focus: vec![Key::BackTab],
            open_config_menu: vec![Key::Char('c')],
            up: vec![Key::Up],
            down: vec![Key::Down],
            right: vec![Key::Right],
            left: vec![Key::Left],
            take_user_input: vec![Key::Char('i')],
            stop_user_input: vec![Key::Ins],
            hide_ui_element: vec![Key::Char('h')],
            save_state: vec![Key::Ctrl('s')],
            new_board: vec![Key::Char('b')],
            new_card: vec![Key::Char('n')],
            delete_card: vec![Key::Char('d')],
            delete_board: vec![Key::Char('D')],
            change_card_status_to_completed: vec![Key::Char('1')],
            change_card_status_to_active: vec![Key::Char('2')],
            change_card_status_to_stale: vec![Key::Char('3')],
            reset_ui: vec![Key::Char('r')],
            go_to_main_menu: vec![Key::Char('m')],
            toggle_command_palette: vec![Key::Ctrl('p')],
            clear_all_toasts: vec![Key::Char('t')],
            undo: vec![Key::Ctrl('z')],
            redo: vec![Key::Ctrl('y')],
        }
    }
}
