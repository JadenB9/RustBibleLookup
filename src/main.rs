// CS-3210-01 Programming Language Survey 2025FA
// Authored by: Jaden Butler and Isaac Lloyd
// Last Updated: Wed, Oct 15th 2026

// L6: Rust Bible Verse Lookup Program
// Input bible reference and will look it up in Bible.txt
// and return the correct verse with pretty print.
// If the verse does not exist, it throws the correct error.
// Bible.txt and Bible_Abbreviations.csv are baked into the binary
// at compile time, so it runs from any directory.

use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::io::{Write, stdin, stdout};
use std::process;

// the whole KJV and the abbreviation table, embedded at compile time
const BIBLE_TEXT: &str = include_str!("../Bible.txt");
const ABBR_TEXT: &str = include_str!("../Bible_Abbreviations.csv");
const VERSION: &str = env!("CARGO_PKG_VERSION");

// define Bible as a big hashmap of maps (book: chapter: verse)
type Bible = HashMap<String, HashMap<String, HashMap<String, String>>>;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("RustBibleLookup {}", VERSION);
        return;
    }

    // load the abbreviations table
    let abbr = load_abbreviations(ABBR_TEXT);

    // load the whole Bible into memory
    let bible = load_bible(BIBLE_TEXT);

    // one-shot mode: the reference came in on the command line
    if !args.is_empty() {
        let (book, chapter, verse) = match parse_reference(&args) {
            Some(parts) => parts,
            None => {
                eprintln!("usage: RustBibleLookup BOOK CHAPTER:VERSE   (see --help)");
                process::exit(2);
            }
        };
        let found = lookup(&bible, &abbr, &book, &chapter, &verse);
        process::exit(if found { 0 } else { 1 });
    }

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

        lookup(&bible, &abbr, &book, &chapter, &verse);

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

fn print_help() {
    println!("RustBibleLookup {}", VERSION);
    println!("Look up a King James Version verse from the command line.");
    println!();
    println!("usage:");
    println!("  RustBibleLookup                      interactive prompts");
    println!("  RustBibleLookup BOOK CHAPTER:VERSE   one-shot lookup");
    println!("  RustBibleLookup BOOK CHAPTER VERSE   same thing");
    println!();
    println!("examples:");
    println!("  RustBibleLookup John 3:16");
    println!("  RustBibleLookup Ps 23 1");
    println!("  RustBibleLookup \"1 Kings\" 2:3");
    println!();
    println!("Full book names and the abbreviations in Bible_Abbreviations.csv");
    println!("(Gen, Ps, Rev, ...) both work. Every verse found is appended to");
    println!("verses.txt in the current directory.");
    println!();
    println!("options:");
    println!("  -h, --help       show this help");
    println!("  -V, --version    print the version");
}

// turn the command line args into (BOOK, chapter, verse). The book can be
// more than one word ("1 Kings", "Song of Solomon") so everything before the
// chapter is joined back together.
fn parse_reference(args: &[String]) -> Option<(String, String, String)> {
    let joined = args.join(" ");
    let words: Vec<&str> = joined.split_whitespace().collect();
    if words.is_empty() {
        return None;
    }

    let last = words[words.len() - 1];
    let (book_words, chapter, verse) = if let Some((c, v)) = last.split_once(':') {
        // "John 3:16"
        (&words[..words.len() - 1], c, v)
    } else if words.len() >= 3 {
        // "John 3 16"
        (&words[..words.len() - 2], words[words.len() - 2], last)
    } else {
        return None;
    };

    if book_words.is_empty() || chapter.is_empty() || verse.is_empty() {
        return None;
    }

    Some((
        book_words.join(" ").to_uppercase(),
        chapter.to_string(),
        verse.to_string(),
    ))
}

// resolve the abbreviation, find the verse, print it (or the right error),
// and append it to verses.txt. Returns true if the verse was found.
fn lookup(
    bible: &Bible,
    abbr: &HashMap<String, String>,
    book: &str,
    chapter: &str,
    verse: &str,
) -> bool {
    // check for abbvr
    let full_book = match abbr.get(book) {
        Some(name) => name.clone(),
        None => book.to_string(),
    };

    // Try to find the verse
    let result = find_verse(bible, &full_book, chapter, verse);

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
        true
    } else {
        // didn't find it
        if result.1 == "book" {
            println!("The Bible does not contain the book \"{}\".", full_book);
        } else if result.1 == "chapter" {
            println!(
                "The book of {} does not have chapter {}.",
                full_book, chapter
            );
        } else if result.1 == "verse" {
            println!(
                "Chapter {} of {} does not have verse {}.",
                chapter, full_book, verse
            );
        }
        false
    }
}

fn load_abbreviations(text: &str) -> HashMap<String, String> {
    let mut abbr = HashMap::new();

    for line in text.lines() {
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

fn load_bible(text: &str) -> Bible {
    let mut bible: Bible = HashMap::new();

    let mut curr_book = String::new();
    let mut curr_chap = String::new();

    // go through every line in the Bible and parse it into our struct
    for line in text.lines() {
        let line = line.to_string();

        if line.trim().is_empty() {
            continue;
        }

        if let Some(name) = line.strip_prefix("THE BOOK OF ") {
            // found a new book
            curr_book = name.trim().to_uppercase();
            curr_chap = String::new();

            // add the book to our hashmap if it doesn't exist
            if !bible.contains_key(&curr_book) {
                bible.insert(curr_book.clone(), HashMap::new());
            }
        } else if let Some(num) = line.strip_prefix("CHAPTER ") {
            // found new chapter
            curr_chap = num.trim().to_string();
            // add the chapter to the book
            if !curr_book.is_empty() && !curr_chap.is_empty() {
                let book_map = bible.get_mut(&curr_book).unwrap();
                if !book_map.contains_key(&curr_chap) {
                    book_map.insert(curr_chap.clone(), HashMap::new());
                }
            }
        } else if let Some(num) = line.strip_prefix("PSALM ") {
            // for the different format in psalms
            curr_chap = num.trim().to_string();
            // add the chapter to the book
            if !curr_book.is_empty() && !curr_chap.is_empty() {
                let book_map = bible.get_mut(&curr_book).unwrap();
                if !book_map.contains_key(&curr_chap) {
                    book_map.insert(curr_chap.clone(), HashMap::new());
                }
            }
        } else if !curr_book.is_empty()
            && !curr_chap.is_empty()
            && line.starts_with(|c: char| c.is_ascii_digit())
        {
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
    // work in chars, not bytes, so a non-ASCII character can't land us
    // in the middle of a codepoint when we slice
    let mut text: Vec<char> = text.chars().collect();

    // while there are still more than 80 chars left to wrap
    while text.len() > 80 {
        // cut point is at 80 in case no spaces are found
        let mut cut_point = 80;

        // find the last space before 81
        for (i, ch) in text.iter().enumerate() {
            if i >= 80 {
                break;
            }
            if *ch == ' ' {
                cut_point = i;
            }
        }

        result.extend(&text[..cut_point]);
        result.push('\n');

        // drop what we printed plus any leading whitespace
        let rest: String = text[cut_point..].iter().collect();
        text = rest.trim_start().chars().collect();
    }

    result.extend(&text);

    result
}
