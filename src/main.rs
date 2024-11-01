mod load_verses;
use std::default;

use load_verses::{load_rst_verses, get_book, Verse, Book};
use iced::{alignment, keyboard, Element, Fill, Length, Renderer, Shrink, Subscription, Task, Theme};
use iced::widget::{button, center, column, container, pick_list, row, text, Row, Text, rich_text, span};
use iced::event::{self, Event};



#[derive(Default)]
struct AppState {
    all_verses: Vec<Verse>,
    current_book: Book,
    book_names: Vec<String>,
    current_chapter: usize,
    current_verse: usize,
    current_verse_text: String,
    user_input: String,
}

#[derive(Debug, Clone)]
enum Message {
    BookPiked(String),
    ChapterPicked(usize),
    VersePicked(usize),
    Event(Event),
}

impl AppState {

    fn new() -> (Self, Task<Message>) {
        let all_verses = load_rst_verses();
        let current_book =  get_book("Matthew".to_string(), all_verses.clone());
       (AppState {
        all_verses: all_verses,
        book_names: vec!["Matthew".to_string(),"Luke".to_string()],
        current_book: current_book.clone(),
        current_chapter: 1,
        current_verse: 1,
        current_verse_text: current_book.chapters.get(&1).unwrap()[0].text.clone(),
        ..Default::default()
    }, Task::none()) 
    }

    fn update(&mut self, message: Message) {

        match message {
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed { text: Some(text), ..}) => {
                    self.user_input.push_str(&text);
                    if !self.current_verse_text.starts_with(&self.user_input) {
                        self.user_input.pop().unwrap();
                    }
                    if self.user_input == self.current_verse_text {
                        self.user_input = String::new();
                        self.next_verse();
                    }
                    
                }

                _=> {}
            }

            Message::BookPiked(book_name) => {
                self.current_book = get_book(book_name, self.all_verses.clone());
                self.current_chapter = 1;
                self.update_text_verse(1);
            }

            Message::ChapterPicked(chapter) => {
                self.current_chapter = chapter;
                self.update_text_verse(1);
            }

            Message::VersePicked(verse) => {
                self.current_verse = verse;
                self.update_text_verse(verse);
            }
        }

    }

    fn update_text_verse(&mut self, verse: usize) {
        self.current_verse_text = self.current_book.chapters.get(&self.current_chapter).unwrap()[verse-1].text.clone();
        self.current_verse = verse;
        self.user_input = String::new();
    }

    fn next_verse(&mut self) {
        let verses_in_chapter = self.current_book.chapters.get(&self.current_chapter).unwrap().len();
        if self.current_verse < verses_in_chapter {
            self.current_verse += 1;
            self.update_text_verse(self.current_verse);
        } else {
            let chapters_in_book = self.current_book.chapters.keys().len();
            if self.current_chapter < chapters_in_book {
                self.current_chapter += 1;
                self.current_verse = 1;
                self.update_text_verse(self.current_verse);
            } else {
                self.current_chapter = 1;
                self.current_verse = 1;
                self.update_text_verse(self.current_verse);
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    fn view(&self) -> Element<Message> {
  
        let book_names = self.book_names.clone();
        let book_picker = pick_list(book_names, Some(self.current_book.book_name.clone()), Message::BookPiked);
        
        let mut book_chapters: Vec<usize> = self.current_book.chapters.clone().into_keys().collect();
        book_chapters.sort();
        let chapter_picker: pick_list::PickList<'_, usize, Vec<usize>, usize, Message> = pick_list(book_chapters, Some(self.current_chapter), Message::ChapterPicked);

        let mut chapter_verses: Vec<usize> = self.current_book.chapters.get(&self.current_chapter).unwrap().iter().map(|v| v.verse).collect();
        chapter_verses.sort();
        let verse_picker: pick_list::PickList<'_, usize, Vec<usize>, usize, Message> = pick_list(chapter_verses, Some(self.current_verse), Message::VersePicked);

        let input_text = self.user_input.clone();
        let verse_text = &self.current_verse_text[self.user_input.len()..];

        let all_text =rich_text![
            span(input_text),
            span(verse_text).color([0.0, 0.0, 0.0, 0.5]),
        ].size(33);

        let pickers = row![book_picker, chapter_picker, verse_picker, ];
        
        column![pickers, center(all_text)].spacing(10).into()
    }
}
fn main() {
   iced::application("TypeTheWord", AppState::update, AppState::view)
   .subscription(AppState::subscription)
   .run_with(AppState::new);
}