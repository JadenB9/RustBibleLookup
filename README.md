# RustBibleLookup

A little command-line Bible verse lookup written in Rust, as a way to learn the
language (a Programming Language Survey assignment, written with Isaac Lloyd).
You give it a book, chapter, and verse; it prints the verse wrapped to 80
columns and appends it to `verses.txt` so you keep a running list of everything
you've looked up.

No dependencies, just the standard library. The full KJV text (public domain)
ships in `Bible.txt` and is baked into the binary at compile time, so the
program runs from any directory.

## Run

Interactive, the way the assignment asked for it:

```bash
cargo run
```

```
Please enter the reference of the verse you would like to find
 Book: John
 Chapter: 3
 Verse: 16
The verse you requested is:

JOHN 3:16:
For God so loved the world, that he gave his only begotten Son, that whosoever
believeth in him should not perish, but have everlasting life.

Would you like to look up another verse (Y/N)?
```

Or pass the reference on the command line and it prints the verse and exits:

```bash
cargo run -- John 3:16
cargo run -- Ps 23 1
cargo run -- "1 Kings" 2:3
```

Book names can be abbreviated (`Gen`, `Ps`, `Rev`, see
`Bible_Abbreviations.csv`) and are case-insensitive. The exit code is 0 when
the verse was found and 1 when it wasn't, so it works in scripts. `--help` and
`--version` do what you'd expect.

`cargo install --path .` puts a `RustBibleLookup` binary on your PATH that
works anywhere, since the text is inside it.

## What it handles

Input is checked one level at a time, so these all print a clear message
instead of crashing:

| Input | Result |
|---|---|
| `JAB 42:17` | unknown book |
| `JOB 43:17` | chapter doesn't exist |
| `JOB 42:18` | verse doesn't exist |
| `SONG OF SOLOMON 6:7` | multi-word book names work |
| `REVELATION 22:21` | last verse in the Bible |

## How it works

`Bible.txt` is the plain-text KJV with `THE BOOK OF ...` headings, `CHAPTER n`
headings, and one numbered verse per line. Psalms is the odd one out: its
chapters are headed `PSALM n`, so the parser looks for both. At startup the
whole thing is walked once into a nested
`HashMap<book, HashMap<chapter, HashMap<verse, text>>>`, which is why lookups
are instant.

`Bible_Abbreviations.csv` is a two-column `abbreviation,full name` file. Both
sides are uppercased when loaded, and user input is uppercased before the
lookup, so `gen`, `Gen`, and `GEN` all land on `GENESIS`.

The verse wrapping works on chars rather than bytes, so a non-ASCII character
can't land in the middle of a codepoint.

## Tests

```bash
cargo test
```

Covers the parser against the real text (first verse, last verse, a Psalm, a
multi-word book), the three not-found cases, the abbreviation table, the
command-line reference parser, and the 80-column wrap.

## Layout

- `src/main.rs` - prompt loop, one-shot mode, `load_bible`, `load_abbreviations`, `find_verse`, `pretty_print`, and the tests
- `Bible.txt` - KJV source text (public domain), embedded with `include_str!`
- `Bible_Abbreviations.csv` - abbreviation to full book name
- `.github/workflows/ci.yml` - fmt, clippy, test, release build
