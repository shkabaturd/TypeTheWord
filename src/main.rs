mod load_verses;
use std::default;

use load_verses::{load_rst_verses, get_book, Verse, Book};
use iced::{Element, Task,};
use iced::widget::{pick_list, column, row, text,};


#[derive(Default)]
struct AppState {
    current_book: Book,
    book_names: Vec<String>,
    current_chapter: u32,
    current_verse: u32,
    current_verse_text: String,
    user_input: String,
}

#[derive(Debug, Clone)]
enum Message {
    BookPiked(String),
    ChapterPicked(u32),
    VersePicked(u32),
}

impl AppState {

    fn new() -> (Self, Task<Message>) {
        let all_verses = load_rst_verses();
        let current_book =  get_book("Matthew".to_string(), all_verses);
       (AppState {
        book_names: vec!["Matthew".to_string(),"Luke".to_string()],
        current_book: current_book.clone(),
        current_chapter: 1,
        current_verse: 1,
        current_verse_text: current_book.chapters.get(&1).unwrap()[0].text.clone(),
        ..Default::default()
    }, Task::none()) 
    }

    fn update(&mut self, message: Message) {

        // TODO: добавить subscription для клавиатуры

    }

    fn view(&self) -> Element<Message> {
        let text = text(&self.current_verse_text);
        println!("{}", self.current_verse_text);
        let select_book = pick_list(self.book_names.clone(), Some("Matthew".to_string()), Message::BookPiked);

        column![text, select_book].spacing(10).into()
    }
}
fn main() {
   iced::application("TypeTheWord", AppState::update, AppState::view).run_with(AppState::new);
}