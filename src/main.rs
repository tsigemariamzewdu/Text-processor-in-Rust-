use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    // 1. Collect CLI args
    let args: Vec<String> = env::args().collect();

    // 2. Get File Path
    let file_path = args
        .get(1)
        .unwrap_or_else(|| {
            eprintln!("Usage: wordfreq <file_path> [--min-length N] [--starts-with C]");
            std::process::exit(1);
        })
        .to_string();

    // 3. Parse optional flags with robust error handling
    let min_length: Option<usize> = get_flag_value(&args, "--min-length")
        .unwrap_or_else(|err| handle_error(&err))
        .and_then(|s| {
            s.parse::<usize>().ok().or_else(|| {
                handle_error("--min-length requires a valid positive number")
            })
        });

    let starts_with: Option<char> = get_flag_value(&args, "--starts-with")
        .unwrap_or_else(|err| handle_error(&err))
        .and_then(|s| s.chars().next());

    // 4. Read file
    let content = fs::read_to_string(&file_path).unwrap_or_else(|err| {
        eprintln!("Failed to read file '{}': {}", file_path, err);
        std::process::exit(1);
    });

    // 5. Build filter and process words
    let filter = build_filter(min_length, starts_with);

    let word_counts: HashMap<String, usize> = content
        .split_whitespace()
        .map(normalize_word)
        .filter(|w| !w.is_empty() && filter(w))
        
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        });

    // 6. Compute and Output Stats
    let total_words: usize = word_counts.values().sum();
    let unique_words: usize = word_counts.len();
    let most_common = word_counts.iter().max_by_key(|(_, count)| *count);

    println!("--- Word Frequency Stats ---");
    println!("Total words:   {}", total_words);
    println!("Unique words:  {}", unique_words);

    match most_common {
        Some((word, count)) => println!("Most common:   '{}' ({})", word, count),
        None => println!("Most common:   (none)"),
    }
}

/// Helper to print error and exit
fn handle_error(msg: &str) -> ! {
    eprintln!("Error: {}", msg);
    std::process::exit(1);
}

/// Robust flag extractor: returns Ok(None) if flag is missing, 
/// Ok(Some) if value exists, and Err if flag is present but value is missing.
fn get_flag_value(args: &[String], flag: &str) -> Result<Option<String>, String> {
    match args.iter().position(|a| a == flag) {
        None => Ok(None),
        Some(i) => match args.get(i + 1) {
            Some(val) => Ok(Some(val.clone())),
            None => Err(format!("Flag '{}' was provided but no value followed it", flag)),
        },
    }
}

fn normalize_word(word: &str) -> String {
    word.trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase()
}

fn build_filter(
    min_length: Option<usize>,
    starts_with: Option<char>,
) -> impl Fn(&String) -> bool {
    // move captures the values into the closure's "backpack"
    move |word: &String| {
        let len_ok = min_length.map_or(true, |n| word.len() > n);
        let starts_ok = starts_with.map_or(true, |c| {
            word.chars()
                .next()
                .map_or(false, |first| first == c.to_lowercase().next().unwrap_or(c))
        });
        len_ok && starts_ok
    }
}
