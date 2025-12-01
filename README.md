# RustBibleLookup

A simple Bible verse lookup program written in Rust.

## How to Run


1. Go to the project folder:

2. Run the program:
   ```
   cargo run
   ```

3. Follow the programs prompts:
```Please enter the reference of the verse you would like to retrieve
 the book: Genesis
 the chapter: 1
 the verse: 1
The verse you requested is:
GENESIS 1:1 In the beginning God created the heaven and the earth.
```
Would you like to look up another verse (Y/N)?

5. The verse will save to verses.txt

6. Type Y to look up another verse or N to quit

## Test Cases
``` GENESIS 1:1 (test first verse in Bible)     
JAB 42:17 (note misspelling of book – test for non-existent book)   
JOB 43:17 (test non-existent chapter)   
JOB 42:18 (test non-existent verse)     
PSALMS 3:5 (test a verse in Psalms)     
SONG OF SOLOMON 6:7 (test multi-word book name)     
MARK 16:20 (test last verse in last chapter of some book)   
REVELATION 22:21 (test last verse in Bible)     
GENESIS 50:26 (test last verse in last chapter in first book)   
PSALMS 119:105 (test three-digit chapter and verse number)  
ISAIAH 42:0 (test a non-existent zeroth verse)  
PHILEMON 1:1 (test verse in book of only one chapter)   
FIRST PETER 3:5 (test two-word book name)   
REVELATION 23:1 (test non-existent last chapter in last book)   
REVELATION 22:22 (test non-existent last verse in last chapter)
```
## Authors

Jaden Butler & Isaac Lloyd
