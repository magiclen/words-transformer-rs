mod gui;

#[rustfmt::skip]
mod logo;

use std::borrow::Cow;

use copypasta::{ClipboardContext, ClipboardProvider};
use gui::*;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, canvas, column, container, row, stack, text, text_input},
    window, Background, Border, Color, Element, Font, Length, Pixels, Settings, Size, Task, Theme,
};
use lazy_static_include::*;
use word_dictionary::*;

const WINDOW_WIDTH: f32 = 450f32;
const WINDOW_HEIGHT: f32 = 420f32;
const WINDOW_PADDING: f32 = 20f32;
const FONT_SIZE: f32 = 24f32;
const CONTROL_SPACING: f32 = 10f32;
const INPUT_PADDING: f32 = 8f32;
const BUTTON_WIDTH: f32 = 100f32;
const BUTTON_HEIGHT: f32 = 36f32;
const CARD_WIDTH: f32 = 320f32;

// The family name embedded in the bundled font, needed to select it as the default font.
const FONT_NAME: &str = "Source Han Sans HW TC";

lazy_static_include_bytes! {
    /// Source Han Sans HW TC Regular
    FONT => "assets/fonts/SourceHanSansHWTC-Regular.ttf",
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum FoundState {
    Default,
    NotFound,
    Found,
}

// The modal dialog currently shown on top of the main view, if any.
#[derive(Debug)]
enum Modal {
    None,
    ConfirmDelete,
    ConfirmEdit { left: String, right: String },
    Message(String),
}

#[derive(Debug, Default)]
struct UIStates {
    keyword:   String,
    result:    String,
    evolution: String,
    key:       String,
    value:     String,
}

#[derive(Debug)]
struct WordsTransformer {
    dictionary:  Dictionary,
    find_index:  usize,
    find_left:   bool,
    found_state: FoundState,
    ui_states:   UIStates,
    modal:       Modal,
}

impl WordsTransformer {
    fn new() -> Self {
        let mut dictionary = Dictionary::new("WordsData");

        dictionary.read_data().unwrap();

        WordsTransformer {
            dictionary,
            find_index: 0,
            find_left: false,
            found_state: FoundState::Default,
            ui_states: UIStates::default(),
            modal: Modal::None,
        }
    }

    fn paste_search(&mut self) {
        let mut ctx = ClipboardContext::new().unwrap();

        if let Ok(t) = ctx.get_contents() {
            self.ui_states.keyword = t;

            self.search();
        }
    }

    fn no_search(&mut self) {
        self.ui_states.result.clear();
        self.ui_states.evolution.clear();
        self.found_state = FoundState::Default;
    }

    fn search(&mut self) {
        let s = self.ui_states.keyword.trim();

        if s.is_empty() {
            return;
        }

        let (find_index, find_left) = match self.dictionary.find_left_strictly(s, 0) {
            Some(find_index) => (find_index, true),
            None => {
                match self.dictionary.find_left(s, 0) {
                    Some(find_index) => (find_index, true),
                    None => {
                        match self.dictionary.find_right_strictly(s, 0) {
                            Some(find_index) => (find_index, false),
                            None => {
                                match self.dictionary.find_right(s, 0) {
                                    Some(find_index) => (find_index, false),
                                    None => {
                                        // not found
                                        self.ui_states.result =
                                            String::from("---Cannot find the word!---");
                                        self.ui_states.evolution.clear();
                                        self.found_state = FoundState::NotFound;

                                        return;
                                    },
                                }
                            },
                        }
                    },
                }
            },
        };

        self.find_index = find_index;
        self.find_left = find_left;

        self.ui_states.result = if find_left {
            self.dictionary.get_left(find_index).unwrap()
        } else {
            self.dictionary.get_right(find_index).unwrap()
        }
        .to_string();

        self.ui_states.evolution = format!(
            "{} = {}",
            self.dictionary.get_left(find_index).unwrap(),
            self.dictionary.get_all_right_to_string(find_index).unwrap()
        );

        self.found_state = FoundState::Found;
    }

    fn copy_result(&mut self) {
        let mut ctx = ClipboardContext::new().unwrap();

        ctx.set_contents(self.ui_states.result.clone()).unwrap();
    }

    fn search_next(&mut self) {
        let s = self.ui_states.keyword.trim();

        let find_index = self.find_index + 1;

        let (find_index, find_left) = if self.find_left {
            let next_index = self.dictionary.find_left(s, find_index).unwrap();

            if next_index == self.find_index {
                match self.dictionary.find_right_strictly(s, find_index) {
                    Some(find_index) => (find_index, false),
                    None => match self.dictionary.find_right(s, find_index) {
                        Some(find_index) => (find_index, false),
                        None => (next_index, true),
                    },
                }
            } else {
                (next_index, true)
            }
        } else {
            let next_index = self.dictionary.find_right(s, find_index).unwrap();

            if next_index == self.find_index {
                match self.dictionary.find_left_strictly(s, find_index) {
                    Some(find_index) => (find_index, true),
                    None => match self.dictionary.find_left(s, find_index) {
                        Some(find_index) => (find_index, true),
                        None => (next_index, false),
                    },
                }
            } else {
                (next_index, false)
            }
        };

        self.find_index = find_index;
        self.find_left = find_left;

        self.ui_states.result = if find_left {
            self.dictionary.get_left(find_index).unwrap()
        } else {
            self.dictionary.get_right(find_index).unwrap()
        }
        .to_string();

        self.ui_states.evolution = format!(
            "{} = {}",
            self.dictionary.get_left(find_index).unwrap(),
            self.dictionary.get_all_right_to_string(find_index).unwrap()
        );
    }

    // Run the actual add/edit and either refresh the view or surface the error in a message dialog.
    fn do_add_edit(&mut self, left: &str, right: &str) {
        match self.dictionary.add_edit(left, right) {
            Ok(_add) => {
                self.ui_states.key.clear();
                self.ui_states.value.clear();

                self.search();
            },
            Err(err) => {
                self.modal = Modal::Message(err.to_string());
            },
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::KeywordDataChanged(data) => {
                self.ui_states.keyword = data;

                self.no_search();
            },
            Message::KeyDataChanged(data) => {
                self.ui_states.key = data;
            },
            Message::ValueDataChanged(data) => {
                self.ui_states.value = data;
            },
            Message::PasteSearch => {
                self.paste_search();
            },
            Message::Search => {
                self.search();
            },
            Message::Copy => {
                self.copy_result();
            },
            Message::Next => {
                self.search_next();
            },
            Message::Delete => {
                // Only offer deletion when a word is currently shown.
                if self.found_state == FoundState::Found {
                    self.modal = Modal::ConfirmDelete;
                }
            },
            Message::Add => {
                let left = self.ui_states.key.trim().to_string();
                let right = self.ui_states.value.trim().to_string();

                // Ask before touching an existing entry, otherwise add directly.
                if self.dictionary.find_left_strictly(&left, 0).is_some() {
                    self.modal = Modal::ConfirmEdit {
                        left,
                        right,
                    };
                } else {
                    self.do_add_edit(&left, &right);
                }
            },
            Message::ModalConfirm => match std::mem::replace(&mut self.modal, Modal::None) {
                Modal::ConfirmDelete => {
                    if let Err(err) = self.dictionary.delete(self.find_index) {
                        self.modal = Modal::Message(err.to_string());
                    } else {
                        self.no_search();
                        self.search();
                    }
                },
                Modal::ConfirmEdit {
                    left,
                    right,
                } => {
                    self.do_add_edit(&left, &right);
                },
                Modal::Message(_) | Modal::None => (),
            },
            Message::ModalCancel => {
                self.modal = Modal::None;
            },
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let btn_paste = button(fill_label("Paste & Search"))
            .width(Length::Fill)
            .height(Length::FillPortion(1))
            .on_press(Message::PasteSearch);

        let ti_keyword = text_input("Input a keyword here.", &self.ui_states.keyword)
            .width(Length::FillPortion(1))
            .padding(INPUT_PADDING)
            .on_input(Message::KeywordDataChanged)
            .on_submit(Message::Search);

        let btn_search = button(fill_label("Search"))
            .width(Length::Fixed(BUTTON_WIDTH))
            .height(Length::Fixed(BUTTON_HEIGHT))
            .on_press(Message::Search);

        let row_keyword =
            row![ti_keyword, btn_search].spacing(CONTROL_SPACING).align_y(Vertical::Center);

        // The result field is read-only, so it has no `on_input` handler.
        let ti_result = text_input("Please search a word first.", &self.ui_states.result)
            .width(Length::FillPortion(1))
            .padding(INPUT_PADDING);

        let btn_copy = {
            let btn = button(fill_label("Copy"))
                .width(Length::Fixed(BUTTON_WIDTH))
                .height(Length::Fixed(BUTTON_HEIGHT));

            if self.found_state == FoundState::Found {
                btn.on_press(Message::Copy)
            } else {
                btn
            }
        };

        let row_result =
            row![ti_result, btn_copy].spacing(CONTROL_SPACING).align_y(Vertical::Center);

        let ti_evolution = text_input("Please search a word first.", &self.ui_states.evolution)
            .width(Length::FillPortion(1))
            .padding(INPUT_PADDING);

        let btn_next = {
            let btn =
                button(fill_label("Next")).width(Length::Fill).height(Length::Fixed(BUTTON_HEIGHT));

            if self.found_state == FoundState::Found {
                btn.on_press(Message::Next)
            } else {
                btn
            }
        };

        let btn_delete = {
            let btn = button(fill_label("Delete"))
                .width(Length::Fill)
                .height(Length::Fixed(BUTTON_HEIGHT));

            if self.found_state == FoundState::Found {
                btn.on_press(Message::Delete)
            } else {
                btn
            }
        };

        let separator = canvas(Separator).width(Length::Fill).height(Length::Fixed(3f32));

        let ti_key = text_input("key", &self.ui_states.key)
            .width(Length::FillPortion(1))
            .padding(INPUT_PADDING)
            .on_input(Message::KeyDataChanged);

        let t_equal = text("=");

        let ti_value = text_input("value", &self.ui_states.value)
            .width(Length::FillPortion(1))
            .padding(INPUT_PADDING)
            .on_input(Message::ValueDataChanged);

        let btn_add = {
            let btn = button(fill_label("Add/Edit"))
                .width(Length::Fixed(BUTTON_WIDTH))
                .height(Length::Fixed(BUTTON_HEIGHT));

            if self.ui_states.key.trim().is_empty() || self.ui_states.value.trim().is_empty() {
                btn
            } else {
                btn.on_press(Message::Add)
            }
        };

        let row_add = row![ti_key, t_equal, ti_value, btn_add]
            .spacing(CONTROL_SPACING)
            .align_y(Vertical::Center);

        let t_count = text(self.dictionary.count().to_string());

        let row_bottom = row![t_count].spacing(CONTROL_SPACING).align_y(Vertical::Top);

        let content = column![
            btn_paste,
            row_keyword,
            row_result,
            ti_evolution,
            btn_next,
            btn_delete,
            separator,
            row_add,
            row_bottom,
        ]
        .spacing(CONTROL_SPACING)
        .align_x(Horizontal::Center);

        let base = container(content)
            .padding(WINDOW_PADDING)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        match &self.modal {
            Modal::None => base.into(),
            Modal::ConfirmDelete => {
                modal_stack(base.into(), dialog_card("Delete this word?".to_string(), true))
            },
            Modal::ConfirmEdit {
                left, ..
            } => modal_stack(
                base.into(),
                dialog_card(format!("\"{left}\" already exists. Add this translation?"), true),
            ),
            Modal::Message(message) => {
                modal_stack(base.into(), dialog_card(message.clone(), false))
            },
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    KeywordDataChanged(String),
    KeyDataChanged(String),
    ValueDataChanged(String),
    PasteSearch,
    Search,
    Copy,
    Next,
    Delete,
    Add,
    ModalConfirm,
    ModalCancel,
}

// A text label that fills its parent and is centered on both axes, used inside buttons.
fn fill_label(label: &str) -> Element<'_, Message> {
    text(label).width(Length::Fill).height(Length::Fill).center().into()
}

// The dimmed background of a modal, styled to intercept clicks meant for the base view.
fn overlay_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(0f32, 0f32, 0f32, 0.5f32))),
        ..container::Style::default()
    }
}

// The card that holds the message and buttons of a modal.
fn card_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::WHITE)),
        border: Border {
            color:  Color::from_rgb(0.7f32, 0.7f32, 0.7f32),
            width:  1f32,
            radius: 6f32.into(),
        },
        ..container::Style::default()
    }
}

// Build a modal card with a message and either No/Yes (confirm) or a single OK button.
fn dialog_card<'a>(message: String, confirm: bool) -> Element<'a, Message> {
    let buttons = if confirm {
        row![
            button(fill_label("No"))
                .width(Length::Fixed(BUTTON_WIDTH))
                .height(Length::Fixed(BUTTON_HEIGHT))
                .on_press(Message::ModalCancel),
            button(fill_label("Yes"))
                .width(Length::Fixed(BUTTON_WIDTH))
                .height(Length::Fixed(BUTTON_HEIGHT))
                .on_press(Message::ModalConfirm),
        ]
    } else {
        row![button(fill_label("OK"))
            .width(Length::Fixed(BUTTON_WIDTH))
            .height(Length::Fixed(BUTTON_HEIGHT))
            .on_press(Message::ModalCancel)]
    }
    .spacing(CONTROL_SPACING);

    let body = column![text(message).width(Length::Fill), buttons]
        .spacing(CONTROL_SPACING)
        .align_x(Horizontal::Center);

    container(body)
        .width(Length::Fixed(CARD_WIDTH))
        .padding(WINDOW_PADDING)
        .style(card_style)
        .into()
}

// Layer a modal card, over a dimmed overlay, over the base view.
fn modal_stack<'a>(base: Element<'a, Message>, card: Element<'a, Message>) -> Element<'a, Message> {
    let overlay =
        container(card).center_x(Length::Fill).center_y(Length::Fill).style(overlay_style);

    stack![base, overlay].into()
}

fn main() -> iced::Result {
    let settings = Settings {
        fonts: vec![Cow::Borrowed(*FONT)],
        default_font: Font::with_name(FONT_NAME),
        default_text_size: Pixels(FONT_SIZE),
        ..Settings::default()
    };

    iced::application(WordsTransformer::new, WordsTransformer::update, WordsTransformer::view)
        .title("Words Transformer")
        .settings(settings)
        .window(window::Settings {
            size: Size::new(WINDOW_WIDTH, WINDOW_HEIGHT),
            level: window::Level::AlwaysOnTop,
            icon: window::icon::from_rgba(logo::DATA.to_vec(), logo::WIDTH, logo::HEIGHT).ok(),
            ..window::Settings::default()
        })
        .run()
}
