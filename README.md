# RustBibleLookup

A little command-line Bible verse lookup written in Rust, as a way to learn the
language. You type a book, chapter, and verse; it prints the verse and appends
it to `verses.txt` so you keep a running list of everything you've looked up.

No dependencies — just the standard library. The full KJV text (public domain)
ships in `Bible.txt` and is loaded into a nested
`HashMap<book, HashMap<chapter, HashMap<verse, text>>>` at startup, so lookups
are instant. `Bible_Abbreviations.csv` lets you type `Gen` or `Ps` instead of
the full book name.

## Run

```bash
cargo run
```

Then follow the prompts:

```
Please enter the reference of the verse you would like to find
 Book: John
 Chapter: 3
 Verse: 16
The verse you requested is:

JOHN 3:16:
For God so loved the world, that he gave his only begotten Son, that
whosoever believeth in him should not perish, but have everlasting life.

Would you like to look up another verse (Y/N)?
```

Answer `Y` to look up another or `N` to quit. Every verse you retrieve is saved
to `verses.txt`.

## What it handles

Input is validated at each level, so these all fail cleanly instead of crashing:

| Input | Result |
|---|---|
| `JAB 42:17` | unknown book |
| `JOB 43:17` | chapter doesn't exist |
| `JOB 42:18` | verse doesn't exist |
| `SONG OF SOLOMON 6:7` | multi-word book names work |
| `REVELATION 22:21` | last verse in the Bible |

## Layout

- `src/main.rs` — prompt loop, lookup, and the two loaders (`load_bible`, `load_abbreviations`)
- `Bible.txt` — KJV source text (public domain)
- `Bible_Abbreviations.csv` — abbreviation → full book name
