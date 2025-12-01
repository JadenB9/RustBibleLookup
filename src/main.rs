// Rust Bible Verse Lookup Program
// Reads Bible.txt and returns the correct verse with pretty print. 
// If the verse does not exist, it throws the correct error.
// Authored by: Jaden Butler and Isaac Lloyd


use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    // load the abbreviations file
    let abbr = load_abbreviations("Bible_Abbreviations.csv");

    // load the whole Bible into memory
    let lines = load_bible("Bible.txt");

    // keep asking for verses until user says no
    loop {
        println!("Please enter the reference of the verse you would like to retrieve");

        // get the book name
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

        // check if they typed an abbreviation and convert it
        let full_book = if abbr.contains_key(&book) {
            abbr.get(&book).unwrap().clone()
        } else {
            book.clone()
        };

        // try to find the verse
        let result = find_verse(&lines, &full_book, chapter, verse);

        if result.0 {
            // we found it!
            let verse_text = &result.1;
            let reference = format!("{} {}:{}", full_book, chapter, verse);
            println!("The verse you requested is:");
            pretty_print(&reference, verse_text);

            // append to the output file
            let mut out_file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("verses.txt")
                .unwrap();
            writeln!(out_file, "{} {}", reference, verse_text).unwrap();
        } else {
            // didn't find it - tell them what's wrong
            let error_type = &result.1;
            if error_type == "book" {
                println!("The Bible does not contain the book \"{}\".", full_book);
            } else if error_type == "chapter" {
                println!("The book of {} does not have chapter {}.", full_book, chapter);
            } else if error_type == "verse" {
                println!("Chapter {} of {} does not have verse {}.", chapter, full_book, verse);
            }
        }

        // see if they want to keep going
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

            // store both the abbreviation and the full name
            abbr.insert(abbrev, full_name.clone());
            abbr.insert(full_name.clone(), full_name);
        }
    }

    abbr
}

fn load_bible(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines
}

fn find_verse(lines: &Vec<String>, book: &str, chapter: i32, verse: i32) -> (bool, String) {
    let mut curr_book = String::new();
    let mut curr_chap = 0;
    let mut chap_verses: HashMap<String, HashMap<i32, i32>> = HashMap::new();

    // go through every line in the Bible
    for line in lines {
        if line.starts_with("THE BOOK OF ") {
            // found a new book
            curr_book = line[12..].trim().to_uppercase();
            curr_chap = 0;
        } else if line.starts_with("CHAPTER ") {
            // found a new chapter
            curr_chap = line[8..].trim().parse().unwrap_or(0);
        } else if line.starts_with("PSALM ") {
            // psalms are formatted differently
            curr_chap = line[6..].trim().parse().unwrap_or(0);
        } else if !curr_book.is_empty() && curr_chap > 0 && !line.trim().is_empty()
                  && line.chars().next().unwrap().is_digit(10) {
            // this looks like a verse line
            let parts: Vec<&str> = line.trim().splitn(2, ' ').collect();
            if parts.len() == 2 {
                let verse_num: i32 = parts[0].parse().unwrap_or(0);
                let verse_text = parts[1];

                // keep track of how many verses each chapter has
                if !chap_verses.contains_key(&curr_book) {
                    chap_verses.insert(curr_book.clone(), HashMap::new());
                }
                let book_map = chap_verses.get_mut(&curr_book).unwrap();
                let max_verse = book_map.get(&curr_chap).unwrap_or(&0);
                book_map.insert(curr_chap, std::cmp::max(*max_verse, verse_num));

                // is this the verse we're looking for?
                if curr_book == book && curr_chap == chapter && verse_num == verse {
                    return (true, verse_text.to_string());
                }
            }
        }
    }

    // we didn't find it - figure out why
    if !chap_verses.contains_key(book) {
        return (false, "book".to_string());
    } else if !chap_verses.get(book).unwrap().contains_key(&chapter) {
        return (false, "chapter".to_string());
    } else if verse > *chap_verses.get(book).unwrap().get(&chapter).unwrap() || verse < 1 {
        return (false, "verse".to_string());
    } else {
        return (false, "verse".to_string());
    }
}

fn pretty_print(reference: &str, text: &str) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut line = format!("{} ", reference);

    for word in words {
        // check if adding this word would make the line too long
        if line.len() + word.len() + 1 > 80 {
            println!("{}", line);
            line = format!("{} ", word);
        } else {
            line.push_str(word);
            line.push(' ');
        }
    }

    // print whatever's left
    if !line.trim().is_empty() {
        println!("{}", line);
    }
}
