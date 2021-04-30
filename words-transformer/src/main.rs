extern crate iced;

extern crate word_dictionary;

#[macro_use]
extern crate lazy_static_include;

extern crate copypasta;

mod gui;
mod logo;

use iced::{
    button, canvas, text_input, window, Align, Column, Container, Element, HorizontalAlignment,
    Length, Row, Sandbox, Settings, Text, VerticalAlignment,
};

use copypasta::{ClipboardContext, ClipboardProvider};

use word_dictionary::*;

use gui::*;

const WINDOW_WIDTH: u32 = 450;
const WINDOW_HEIGHT: u32 = 420;
const WINDOW_PADDING: u16 = 20;
const FONT_SIZE: u16 = 24;
const CONTROL_SPACING: u16 = 10;
const INPUT_PADDING: u16 = 8;
const BUTTON_WIDTH: u16 = 100;
const BUTTON_HEIGHT: u16 = 36;

lazy_static_include_bytes! {
    /// Source Han Sans HW TC Regular
    FONT => "assets/fonts/SourceHanSansHWTC-Regular.ttf",
}

// static FONT: &[u8] = include_bytes!("../assets/fonts/SourceHanSansHWTC-Regular2.ttf");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum FoundState {
    Default,
    NotFound,
    Found,
}

#[derive(Debug, Default)]
struct UIStates {
    keyword: String,
    keyword_state: text_input::State,
    result: String,
    result_state: text_input::State,
    evolution: String,
    evolution_state: text_input::State,
    key: String,
    key_state: text_input::State,
    value: String,
    value_state: text_input::State,
    paste_search_state: button::State,
    search_state: button::State,
    copy_state: button::State,
    next_state: button::State,
    delete_state: button::State,
    add_state: button::State,
}

#[derive(Debug)]
struct WordsTransformer {
    dictionary: Dictionary,
    find_index: usize,
    find_left: bool,
    found_state: FoundState,
    ui_states: UIStates,
}

impl WordsTransformer {
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
                                    }
                                }
                            }
                        }
                    }
                }
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
                    None => {
                        match self.dictionary.find_right(s, find_index) {
                            Some(find_index) => (find_index, false),
                            None => (next_index, true),
                        }
                    }
                }
            } else {
                (next_index, true)
            }
        } else {
            let next_index = self.dictionary.find_right(s, find_index).unwrap();

            if next_index == self.find_index {
                match self.dictionary.find_left_strictly(s, find_index) {
                    Some(find_index) => (find_index, true),
                    None => {
                        match self.dictionary.find_left(s, find_index) {
                            Some(find_index) => (find_index, true),
                            None => (next_index, false),
                        }
                    }
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

    fn delete(&mut self) {
        // TODO: Should show up a confirm box to ask the user whether to delete.
    }

    fn add(&mut self) {
        let left = self.ui_states.key.trim().to_string();
        let right = self.ui_states.value.trim().to_string();

        if self.dictionary.find_left_strictly(&left, 0).is_some() {
            // TODO: Should show up a confirm box to ask the user whether to edit.
        }

        #[allow(clippy::single_match)]
        match self.dictionary.add_edit(&left, &right) {
            Ok(_add) => {
                self.ui_states.key.clear();
                self.ui_states.value.clear();

                self.search();
            }
            Err(_) => {
                // TODO: Should show up a message box to tell the user it failed. But currently, no dialog supports.
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    KeywordDataChanged(String),
    ResultDataChanged(String),
    EvolutionDataChanged(String),
    KeyDataChanged(String),
    ValueDataChanged(String),
    PasteSearch,
    Search,
    Copy,
    Next,
    Delete,
    Add,
}

impl Sandbox for WordsTransformer {
    type Message = Message;

    #[inline]
    fn new() -> Self {
        let mut dictionary = Dictionary::new("WordsData");

        dictionary.read_data().unwrap();

        WordsTransformer {
            dictionary,
            find_index: 0,
            find_left: false,
            found_state: FoundState::Default,
            ui_states: UIStates::default(),
        }
    }

    #[inline]
    fn title(&self) -> String {
        String::from("Words Transformer")
    }

    #[inline]
    fn update(&mut self, message: Message) {
        match message {
            Message::KeywordDataChanged(data) => {
                self.ui_states.keyword = data;

                self.no_search();
            }
            Message::ResultDataChanged(_) => {
                // read only
            }
            Message::EvolutionDataChanged(_) => {
                // read only
            }
            Message::KeyDataChanged(data) => {
                self.ui_states.key = data;
            }
            Message::ValueDataChanged(data) => {
                self.ui_states.value = data;
            }
            Message::PasteSearch => {
                self.paste_search();
            }
            Message::Search => {
                self.search();
            }
            Message::Copy => {
                self.copy_result();
            }
            Message::Next => {
                self.search_next();
            }
            Message::Delete => {
                self.delete();
            }
            Message::Add => {
                self.add();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let btn_paste = button::Button::new(
            &mut self.ui_states.paste_search_state,
            Text::new("Paste & Search")
                .width(Length::Fill)
                .height(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .width(Length::Fill)
        .height(Length::FillPortion(1))
        .on_press(Message::PasteSearch);

        let ti_keyword = text_input::TextInput::new(
            &mut self.ui_states.keyword_state,
            "Input a keyword here.",
            &self.ui_states.keyword,
            Message::KeywordDataChanged,
        )
        .width(Length::FillPortion(1))
        .padding(INPUT_PADDING)
        .on_submit(Message::Search);

        let btn_search = button::Button::new(
            &mut self.ui_states.search_state,
            Text::new("Search")
                .width(Length::Fill)
                .height(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .width(Length::Units(BUTTON_WIDTH))
        .height(Length::Units(BUTTON_HEIGHT))
        .on_press(Message::Search);

        let row_keyword = Row::new()
            .width(Length::Fill)
            .height(Length::Shrink)
            .spacing(CONTROL_SPACING)
            .align_items(Align::Center)
            .push(ti_keyword)
            .push(btn_search);

        let ti_result = text_input::TextInput::new(
            &mut self.ui_states.result_state,
            "Please search a word first.",
            &self.ui_states.result,
            Message::ResultDataChanged,
        )
        .width(Length::FillPortion(1))
        .padding(INPUT_PADDING);

        let btn_copy = button::Button::new(
            &mut self.ui_states.copy_state,
            Text::new("Copy")
                .width(Length::Fill)
                .height(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .width(Length::Units(BUTTON_WIDTH))
        .height(Length::Units(BUTTON_HEIGHT));

        let btn_copy = if self.found_state == FoundState::Found {
            btn_copy.on_press(Message::Copy)
        } else {
            btn_copy
        };

        let row_result = Row::new()
            .width(Length::Fill)
            .height(Length::Shrink)
            .spacing(CONTROL_SPACING)
            .align_items(Align::Center)
            .push(ti_result)
            .push(btn_copy);

        let ti_evolution = text_input::TextInput::new(
            &mut self.ui_states.evolution_state,
            "Please search a word first.",
            &self.ui_states.evolution,
            Message::EvolutionDataChanged,
        )
        .width(Length::FillPortion(1))
        .padding(INPUT_PADDING);

        let btn_next = button::Button::new(
            &mut self.ui_states.next_state,
            Text::new("Next")
                .width(Length::Fill)
                .height(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Units(BUTTON_HEIGHT));

        let btn_next = if self.found_state == FoundState::Found {
            btn_next.on_press(Message::Next)
        } else {
            btn_next
        };

        let btn_delete = button::Button::new(
            &mut self.ui_states.delete_state,
            Text::new("Delete")
                .width(Length::Fill)
                .height(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Units(BUTTON_HEIGHT));

        let btn_delete = if self.found_state == FoundState::Found {
            btn_delete.on_press(Message::Delete)
        } else {
            btn_delete
        };

        let separator = canvas::Canvas::new(Separator).width(Length::Fill).height(Length::Units(3));

        let ti_key = text_input::TextInput::new(
            &mut self.ui_states.key_state,
            "key",
            &self.ui_states.key,
            Message::KeyDataChanged,
        )
        .width(Length::FillPortion(1))
        .padding(INPUT_PADDING);

        let t_equal = Text::new("=");

        let ti_value = text_input::TextInput::new(
            &mut self.ui_states.value_state,
            "value",
            &self.ui_states.value,
            Message::ValueDataChanged,
        )
        .width(Length::FillPortion(1))
        .padding(INPUT_PADDING);

        let btn_add = button::Button::new(
            &mut self.ui_states.add_state,
            Text::new("Add/Edit")
                .width(Length::Fill)
                .height(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        )
        .width(Length::Units(BUTTON_WIDTH))
        .height(Length::Units(BUTTON_HEIGHT));

        let btn_add =
            if self.ui_states.key.trim().is_empty() || self.ui_states.value.trim().is_empty() {
                btn_add
            } else {
                btn_add.on_press(Message::Add)
            };

        let row_add = Row::new()
            .width(Length::Fill)
            .height(Length::Shrink)
            .spacing(CONTROL_SPACING)
            .align_items(Align::Center)
            .push(ti_key)
            .push(t_equal)
            .push(ti_value)
            .push(btn_add);

        let t_count = Text::new(format!("{}", self.dictionary.count()));

        let row_bottom = Row::new()
            .width(Length::Fill)
            .height(Length::Shrink)
            .spacing(CONTROL_SPACING)
            .align_items(Align::Start)
            .push(t_count);

        let content = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(CONTROL_SPACING)
            .align_items(Align::Center)
            .push(btn_paste)
            .push(row_keyword)
            .push(row_result)
            .push(ti_evolution)
            .push(btn_next)
            .push(btn_delete)
            .push(separator)
            .push(row_add)
            .push(row_bottom);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(WINDOW_PADDING)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    WordsTransformer::run(Settings {
        window: window::Settings {
            size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            always_on_top: true,
            icon: Some(
                window::Icon::from_rgba(logo::DATA.to_vec(), logo::WIDTH, logo::HEIGHT)
                    .map_err(|err| iced::Error::WindowCreationFailed(Box::new(err)))?,
            ),
            ..window::Settings::default()
        },
        default_font: Some(&*FONT),
        default_text_size: FONT_SIZE,
        ..Settings::default()
    })
}
