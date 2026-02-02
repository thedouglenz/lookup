use clap::Parser;
use serde::Deserialize;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "lookup")]
#[command(about = "Fetch dictionary definitions and Wikipedia summaries")]
struct Args {
    /// The word or term to look up
    term: String,
}

// Dictionary API response types
#[derive(Deserialize)]
struct DictionaryEntry {
    meanings: Vec<Meaning>,
}

#[derive(Deserialize)]
struct Meaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<Definition>,
}

#[derive(Deserialize)]
struct Definition {
    definition: String,
}


fn fetch_dictionary(word: &str) -> Result<Vec<DictionaryEntry>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        urlencoded(word)
    );
    let response = reqwest::blocking::get(&url)?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(vec![]);
    }

    let entries: Vec<DictionaryEntry> = response.json()?;
    Ok(entries)
}

fn fetch_wikipedia(term: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=extracts&exintro&explaintext&redirects&titles={}&format=json",
        urlencoded(term)
    );
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "lookup-cli/0.1 (https://github.com/example/lookup)")
        .send()?;
    let response: serde_json::Value = resp.json()?;

    if let Some(pages) = response.get("query").and_then(|q| q.get("pages")) {
        if let Some(pages_obj) = pages.as_object() {
            for (_page_id, page) in pages_obj {
                if let Some(extract) = page.get("extract").and_then(|e| e.as_str()) {
                    if !extract.is_empty() {
                        return Ok(Some(extract.to_string()));
                    }
                }
            }
        }
    }
    Ok(None)
}

fn urlencoded(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
            ' ' => result.push_str("%20"),
            _ => {
                for b in c.to_string().bytes() {
                    result.push_str(&format!("%{:02X}", b));
                }
            }
        }
    }
    result
}

fn print_definitions(entries: &[DictionaryEntry]) {
    println!("Definitions:");
    if entries.is_empty() {
        println!("N/A");
        return;
    }

    let mut count = 0;
    for entry in entries {
        for meaning in &entry.meanings {
            for def in &meaning.definitions {
                count += 1;
                println!("{}. ({}) {}", count, meaning.part_of_speech, def.definition);
            }
        }
    }

    if count == 0 {
        println!("N/A");
    }
}

fn print_wikipedia(extract: Option<String>) {
    println!("\nWikipedia:");
    match extract {
        Some(text) => println!("{}", text.trim()),
        None => println!("N/A"),
    }
}

fn main() -> ExitCode {
    let args = Args::parse();

    let dict_result = fetch_dictionary(&args.term);
    let wiki_result = fetch_wikipedia(&args.term);

    match dict_result {
        Ok(entries) => print_definitions(&entries),
        Err(e) => {
            eprintln!("Error fetching dictionary: {}", e);
            return ExitCode::from(1);
        }
    }

    match wiki_result {
        Ok(extract) => print_wikipedia(extract),
        Err(e) => {
            eprintln!("Error fetching Wikipedia: {}", e);
            return ExitCode::from(1);
        }
    }

    ExitCode::SUCCESS
}
