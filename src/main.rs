// CS-3210-01 Programming Language Survey 2025FA
// Authored by: Jaden Butler and Isaac Lloyd
// Last Updated: Sun, Dec 7th 2025

// L6: Rust Bible Verse Lookup Program
// Input bible reference and will look it up in Bible.txt
// and return the correct verse with pretty print. 
// If the verse does not exist, it throws the correct error.
// Will fail if Bible.txt or Bible_Abbreviations.csv do not exist

use std::collections::HashMap;
use std::fs::{OpenOptions};
use std::io::{stdin, stdout, Write};
use std::fs::read_to_string;

// define Bible as a big hashmap of maps (book: chapter: verse)
type Bible = HashMap<String, HashMap<String, HashMap<String, String>>>;

fn main() {
    // load the abbreviations file
    let abbr = load_abbreviations("Bible_Abbreviations.csv");

    // load the whole Bible into memory
    let bible = load_bible("Bible.txt");

    // main loop - keep requesting verses until user quits
    loop {
        println!("Please enter the reference of the verse you would like to find");

        // book name
        print!(" Book: ");
        stdout().flush().unwrap();

        let mut book = String::new();
        stdin().read_line(&mut book).unwrap();
        let book = book.trim().to_uppercase();

        // get the chapter number
        print!(" Chapter: ");
        stdout().flush().unwrap();

        let mut chapter = String::new();
        stdin().read_line(&mut chapter).unwrap();
        let chapter = chapter.trim().to_string();

        // get the verse number
        print!(" Verse: ");
        stdout().flush().unwrap();

        let mut verse = String::new();
        stdin().read_line(&mut verse).unwrap();
        let verse = verse.trim().to_string();

        // check for abbvr
        let full_book = if abbr.contains_key(&book) {
            abbr.get(&book).unwrap().clone()
        } else {
            book.clone()
        };

        // Try to find the verse
        let result = find_verse(&bible, &full_book, &chapter, &verse);

        if result.0 {
            //found

            let verse_text = result.1;
            let reference = format!("{} {}:{}", full_book, chapter, verse);
            println!("The verse you requested is:");
            let verse_text_pretty = pretty_print(&verse_text);
            println!("\n{}:\n{}\n", reference, verse_text_pretty);

            // add to verses.txt
            let mut out_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("verses.txt")
                .unwrap();
            writeln!(out_file, "{} {}", reference, verse_text).unwrap();

        } else {
            // didn't find it
            if result.1 == "book" {
                println!("The Bible does not contain the book \"{}\".", full_book);
            } else if result.1 == "chapter" {
                println!("The book of {} does not have chapter {}.", full_book, chapter);
            } else if result.1 == "verse" {
                println!("Chapter {} of {} does not have verse {}.", chapter, full_book, verse);
            }
        }

        print!("Would you like to look up another verse (Y/N)? ");
        stdout().flush().unwrap();

        let mut response = String::new();
        stdin().read_line(&mut response).unwrap();
        if response.trim().to_uppercase() != "Y" {
            // exit the loop if the user does not answer y
            break;
        }
    }
}

fn load_abbreviations(filename: &str) -> HashMap<String, String> {
    let mut abbr = HashMap::new();

    for line in read_to_string(filename).unwrap().lines() {
        // split into abbr and full
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() == 2 {
            let abbrev = parts[0].trim().to_uppercase();
            let full_name = parts[1].trim().to_uppercase();

            // add the abbreviation to map
            abbr.insert(abbrev, full_name);
        }
    }

    abbr
}

fn load_bible(filename: &str) -> Bible {
    let mut bible:Bible = HashMap::new();

    let mut curr_book = String::new();
    let mut curr_chap = String::new();

    // go through every line in the Bible and parse it into our struct
    for line in read_to_string(filename).unwrap().lines() {
        let line = line.to_string();

        if line.trim().is_empty() {
            continue;
        }

        if line.starts_with("THE BOOK OF ") {
            // found a new book
            curr_book = line[12..].trim().to_uppercase();
            curr_chap = String::new();

            // add the book to our hashmap if it doesn't exist
            if !bible.contains_key(&curr_book) {
                bible.insert(curr_book.clone(), HashMap::new());
            }

        } else if line.starts_with("CHAPTER ") {
            // found new chapter
            curr_chap = line[8..].trim().to_string();
            // add the chapter to the book
            if !curr_book.is_empty() && !curr_chap.is_empty() {
                let book_map = bible.get_mut(&curr_book).unwrap();
                if !book_map.contains_key(&curr_chap) {
                    book_map.insert(curr_chap.clone(), HashMap::new());
                }
            }

        } else if line.starts_with("PSALM ") {
            // for the different format in psalms
            curr_chap = line[6..].trim().to_string();
            // add the chapter to the book
            if !curr_book.is_empty() && !curr_chap.is_empty() {
                let book_map = bible.get_mut(&curr_book).unwrap();
                if !book_map.contains_key(&curr_chap) {
                    book_map.insert(curr_chap.clone(), HashMap::new());
                }
            }

        } else if !curr_book.is_empty() && !curr_chap.is_empty()
                && line.chars().next().unwrap().is_digit(10) {

            // this is a verse line
            let parts: Vec<&str> = line.trim().splitn(2, ' ').collect();
            if parts.len() == 2 {
                let verse_num = parts[0].to_string();
                let verse_text = parts[1].to_string();

                // add the verse to the chapter
                let book_map = bible.get_mut(&curr_book).unwrap();
                let chap_map = book_map.get_mut(&curr_chap).unwrap();
                chap_map.insert(verse_num, verse_text);
            }
        }
    }

    bible
}

fn find_verse(bible: &Bible, book: &str, chapter: &str, verse: &str) -> (bool, String) {
    // check for the book
    if !bible.contains_key(book) {
        return (false, "book".to_string());
    }
    
    // check for the chapter if book exists
    if !bible[book].contains_key(chapter) {
        return (false, "chapter".to_string());
    }
    
    // check for the verse if book and chapter exist
    if !bible[book][chapter].contains_key(verse) {
        return (false, "verse".to_string());
    }
    
    // return the verse if book, chapter, and verse exist
    (true, bible[book][chapter][verse].clone())
}

fn pretty_print(text: &str) -> String {
    let mut result = String::new();
    let mut text = text.to_string();
    
    // while there are still more than 80 chars left to wrap
    while text.len() > 80 {
        // cut point is at 80 in case no spaces are found
        let mut cut_point = 80;
        
        // find the last space before 81
        for (i, ch) in text.chars().enumerate() {
            if i >= 80 {
                break;
            }
            if ch == ' ' {
                cut_point = i;
            }
        }
        
        result.push_str(&text[..cut_point]);
        result.push('\n');
        
        text = text[cut_point..].trim_start().to_string();
    }
    
    result.push_str(&text);

    result
}