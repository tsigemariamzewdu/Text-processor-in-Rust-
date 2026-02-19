# Word Frequency CLI in Rust

A command-line Rust program that processes a text file using a functional programming style.

It avoids imperative loops (`for`, `while`) and uses iterators, closures, and combinators to transform and analyze text.

## Features

- Reads a text file path from command-line arguments.
- Splits text into words.
- Normalizes words to lowercase for case-insensitive counting.
- Counts word frequencies with iterator pipelines and `HashMap`.
- Displays:
  - Total word count
  - Number of unique words
  - Most common word and its count
- Supports dynamic filtering with flags:
  - `--min-length N`: only count words with length greater than `N`
  - `--starts-with C`: only count words starting with character `C`
- Extra output control:
  - `--top K`: show top `K` most frequent words (default: `10`)

## Functional Programming Approach

The core processing uses chained iterator operations:

- `split_whitespace()` to tokenize
- `map(normalize_word)` to clean and lowercase
- `filter(...)` with a closure built at runtime from CLI flags
- `fold(HashMap::new(), ...)` to build frequency counts

Filtering logic is implemented using a closure from `build_filter(min_length, starts_with)`, allowing behavior to change based on provided options without branching loops.

## Project Structure

```text
wordfreq/
  src/main.rs
  input.txt
  Cargo.toml
```

## Requirements

- Rust toolchain installed (stable)
- Cargo

## Run

From the `wordfreq` directory:

```bash
cargo run -- <file_path> [--min-length N] [--starts-with C] [--top K]
```

## Examples

Run with no filters:

```bash
cargo run -- input.txt
```

Only include words longer than 3:

```bash
cargo run -- input.txt --min-length 3
```

Only include words starting with `a`:

```bash
cargo run -- input.txt --starts-with a
```

Combine both filters:

```bash
cargo run -- input.txt --min-length 4 --starts-with t
```

Show only top 5 words:

```bash
cargo run -- input.txt --top 5
```

## Sample Input

Current `input.txt` contains text designed to test:

- case-insensitive matching (`Rust`, `RUST`, `rust`)
- length filtering
- starting-letter filtering
- repeated common words

## Output Example

```text
--- Word Frequency Stats ---
Total words:   40
Unique words:  31
Most common:   'and' (4)

Top 10 most frequent words:
 1. rust            -> 4
 2. and             -> 4
 3. some            -> 3
 4. are             -> 2
 5. data            -> 1
 6. short           -> 1
 7. functional      -> 1
 8. in              -> 1
 9. be              -> 1
10. iterators       -> 1
```

## Error Handling

- Missing file path shows usage and exits.
- Invalid or missing flag values print an error and exit.
- File read failures print the OS error and exit.
