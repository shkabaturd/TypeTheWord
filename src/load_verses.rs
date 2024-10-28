use serde::{Deserialize, Serialize,};

use std::{collections::HashMap, include_str};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Verse {
    pub book_id: String,
    pub book_name: String,
    pub chapter: u32,
    pub verse: u32,
    pub text: String,
}

#[derive(Debug, Default, Clone)]
pub struct Book {
    pub book_name: String,
    pub chapters: HashMap<u32, Vec<Verse>>,
}

const VERSES_TEXT_RST: &str = include_str!("../bible_translations/rst.json");

pub fn load_rst_verses() -> Vec<Verse> {
    let mut verses: Vec<Verse> = Vec::new();
    for line in VERSES_TEXT_RST.split('\n').filter(|s| !s.is_empty()) {
        //println!("{}", line);
        let v: Verse = serde_json::from_str(line).unwrap();
        verses.push(v);
       
    }
    return verses;
}

pub fn get_book(book_name: String, verses: Vec<Verse>) -> Book {
    let all_book_verses: Vec<Verse> = verses.into_iter().filter(|v| v.book_name == book_name).collect();
    let mut chapters: HashMap<u32, Vec<Verse>> = HashMap::new();

    for verse in all_book_verses {
        chapters.entry(verse.chapter).or_insert(Vec::new()).push(verse);
    }

    Book {
        book_name: book_name,
        chapters: chapters,
    }
}
