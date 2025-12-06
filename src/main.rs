// Rust Bible Verse Lookup Program
// Reads Bible.txt and returns the correct verse with pretty print. 
// If the verse does not exist, it throws the correct error.
// Authored by: Jaden Butler and Isaac Lloyd

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

// struct to hold all the bible data after we parse it
struct Bible {
    // book name -> chapter number -> verse number -> verse text
    books: HashMap<String, HashMap<i32, HashMap<i32, String>>>,
}

fn main() {
    // load the abbreviations file
    let abbr = load_abbreviations("Bible_Abbreviations.csv");
    // load the whole Bible into memory (parsed this time)
    let bible = load_bible("Bible.txt");

    // main loop - keep requesting verses until N
    loop {
        println!("Please enter the reference of the verse you would like to retrieve");

        // book name
        print!(" the book: ");
        io::stdout().flush().unwrap();
        let mut book = String::new();
        io::stdin().read_line(&mut book).unwrap();
        let book = book.trim().to_uppercase();
        // get the chapter number
        print!(" the chapter: ");
        io::stdout().flush().unwrap();
        let mut chapter = String::new();
        io::stdin().read_line(&mut chapter).unwrap();
        let chapter: i32 = chapter.trim().parse().unwrap();
        // get the verse number
        print!(" the verse: ");
        io::stdout().flush().unwrap();
        let mut verse = String::new();
        io::stdin().read_line(&mut verse).unwrap();
        let verse: i32 = verse.trim().parse().unwrap();

        // check for abbvr
        let full_book = if abbr.contains_key(&book) {
            abbr.get(&book).unwrap().clone()
        } else {
            book.clone()
        };

        // Actually try to find the verse
        let result = find_verse(&bible, &full_book, chapter, verse);

        if result.0 {
            //found
            let verse_text = &result.1;
            let reference = format!("{} {}:{}", full_book, chapter, verse);
            println!("The verse you requested is:");
            pretty_print(&reference, verse_text);

            // add to verses.txt
            let mut out_file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("verses.txt")
                .unwrap();
            writeln!(out_file, "{} {}", reference, verse_text).unwrap();
        } else {
            // didn't find it
            let error_type = &result.1;
            if error_type == "book" {
                println!("The Bible does not contain the book \"{}\".", full_book);
            } else if error_type == "chapter" {
                println!("The book of {} does not have chapter {}.", full_book, chapter);
            } else if error_type == "verse" {
                println!("Chapter {} of {} does not have verse {}.", chapter, full_book, verse);
            }
        }
        print!("Would you like to look up another verse (Y/N)? ");
        io::stdout().flush().unwrap();
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        if response.trim().to_uppercase() != "Y" {
            break;
        }
    }
}

fn load_abbreviations(filename: &str) -> HashMap<String, String> {
    let mut abbr = HashMap::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // read each line from the CSV
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let abbrev = parts[0].trim().to_uppercase();
            let full_name = parts[1].trim().to_uppercase();

            // store both the abbreviation and full name
            abbr.insert(abbrev, full_name.clone());
            abbr.insert(full_name.clone(), full_name);
        }
    }
    abbr
}

fn load_bible(filename: &str) -> Bible {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut bible = Bible {
        books: HashMap::new(),
    };
    let mut curr_book = String::new();
    let mut curr_chap = 0;

    // go through every line in the Bible and parse it into our struct
    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with("THE BOOK OF ") {
            // found a new book
            curr_book = line[12..].trim().to_uppercase();
            curr_chap = 0;
            // add the book to our hashmap if it doesn't exist
            if !bible.books.contains_key(&curr_book) {
                bible.books.insert(curr_book.clone(), HashMap::new());
            }
        } else if line.starts_with("CHAPTER ") {
            // found new chapter
            curr_chap = line[8..].trim().parse().unwrap_or(0);
            // add the chapter to the book
            if !curr_book.is_empty() && curr_chap > 0 {
                let book_map = bible.books.get_mut(&curr_book).unwrap();
                if !book_map.contains_key(&curr_chap) {
                    book_map.insert(curr_chap, HashMap::new());
                }
            }
        } else if line.starts_with("PSALM ") {
            // for the different format in psalms
            curr_chap = line[6..].trim().parse().unwrap_or(0);
            // add the chapter to the book
            if !curr_book.is_empty() && curr_chap > 0 {
                let book_map = bible.books.get_mut(&curr_book).unwrap();
                if !book_map.contains_key(&curr_chap) {
                    book_map.insert(curr_chap, HashMap::new());
                }
            }
        } else if !curr_book.is_empty() && curr_chap > 0 && !line.trim().is_empty()
                  && line.chars().next().unwrap().is_digit(10) {
            // this is a verse line
            let parts: Vec<&str> = line.trim().splitn(2, ' ').collect();
            if parts.len() == 2 {
                let verse_num: i32 = parts[0].parse().unwrap_or(0);
                let verse_text = parts[1].to_string();

                // add the verse to the chapter
                let book_map = bible.books.get_mut(&curr_book).unwrap();
                let chap_map = book_map.get_mut(&curr_chap).unwrap();
                chap_map.insert(verse_num, verse_text);
            }
        }
    }
    bible
}

fn find_verse(bible: &Bible, book: &str, chapter: i32, verse: i32) -> (bool, String) {
    // check if the book exists
    if !bible.books.contains_key(book) {
        return (false, "book".to_string());
    }
    let book_map = bible.books.get(book).unwrap();
    // check if the chapter exists
    if !book_map.contains_key(&chapter) {
        return (false, "chapter".to_string());
    }
    let chap_map = book_map.get(&chapter).unwrap();
    // check if the verse exists
    if !chap_map.contains_key(&verse) || verse < 1 {
        return (false, "verse".to_string());
    }
    // found it!
    let verse_text = chap_map.get(&verse).unwrap();
    return (true, verse_text.clone());
}

fn pretty_print(reference: &str, text: &str) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut line = format!("{} ", reference);
    for word in words {
        if line.len() + word.len() + 1 > 80 {
            println!("{}", line);
            line = format!("{} ", word);
        } else {
            line.push_str(word);
            line.push(' ');
        }
    }
    if !line.trim().is_empty() {
        println!("{}", line);
    }
}